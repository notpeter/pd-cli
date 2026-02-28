use nusb::MaybeFuture;
use std::collections::{HashMap, HashSet};
use std::env;
use std::process::Command;

const PLAYDATE_VENDOR_ID: u16 = 0x1331;
const PLAYDATE_PRODUCT_ID: u16 = 0x5741;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Device {
    device: String,
    port: String,
    mounted: bool,
    mount_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MountEntry {
    source: String,
    target: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("device") => match args.get(2).map(String::as_str) {
            Some("list") => {
                let devices = list_devices()?;
                print_devices(&devices);
                Ok(())
            }
            _ => Err("usage: pd device list".to_string()),
        },
        _ => Err("usage: pd device list".to_string()),
    }
}

fn list_devices() -> Result<Vec<Device>, String> {
    let serial_ports = list_serial_ports();
    let mounts = list_mounts().unwrap_or_default();
    let disk_mounts = build_disk_mount_index(&mounts);

    #[cfg(target_os = "macos")]
    let serial_to_disks = list_macos_playdate_disks_by_serial().unwrap_or_default();

    #[cfg(not(target_os = "macos"))]
    let serial_to_disks: HashMap<String, Vec<String>> = HashMap::new();

    let any_playdate_mount_path = find_any_playdate_mount(&mounts).unwrap_or_default();

    let mut devices = Vec::new();

    let usb_devices = nusb::list_devices()
        .wait()
        .map_err(|e| format!("failed to list USB devices: {e}"))?;

    for usb in usb_devices {
        if usb.vendor_id() != PLAYDATE_VENDOR_ID || usb.product_id() != PLAYDATE_PRODUCT_ID {
            continue;
        }

        let serial = usb.serial_number().unwrap_or("unknown").to_string();
        let port = find_port_for_serial(&serial, &serial_ports).unwrap_or_default();
        let mount_path = find_mount_path_for_serial(&serial, &serial_to_disks, &disk_mounts)
            .or_else(|| {
                if serial_to_disks.is_empty() && !any_playdate_mount_path.is_empty() {
                    Some(any_playdate_mount_path.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        devices.push(Device {
            device: serial,
            port,
            mounted: !mount_path.is_empty(),
            mount_path,
        });
    }

    devices.sort_by(|a, b| a.device.cmp(&b.device));
    devices.dedup();
    Ok(devices)
}

#[cfg(unix)]
fn list_serial_ports() -> Vec<String> {
    use std::fs;

    let mut ports = Vec::new();

    let entries = match fs::read_dir("/dev") {
        Ok(entries) => entries,
        Err(_) => return ports,
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();

        let is_usb_serial = name.starts_with("cu.usbmodem")
            || name.starts_with("tty.usbmodem")
            || name.starts_with("ttyACM")
            || name.starts_with("ttyUSB");

        if is_usb_serial {
            ports.push(format!("/dev/{name}"));
        }
    }

    ports.sort();
    ports
}

#[cfg(not(unix))]
fn list_serial_ports() -> Vec<String> {
    Vec::new()
}

#[cfg(unix)]
fn list_mounts() -> Result<Vec<MountEntry>, String> {
    let output = Command::new("mount")
        .output()
        .map_err(|e| format!("failed to run mount: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("mount failed: {stderr}"));
    }

    let text = String::from_utf8(output.stdout)
        .map_err(|e| format!("mount returned non-UTF8 output: {e}"))?;

    Ok(parse_mount_entries(&text))
}

#[cfg(not(unix))]
fn list_mounts() -> Result<Vec<MountEntry>, String> {
    Ok(Vec::new())
}

fn parse_mount_entries(input: &str) -> Vec<MountEntry> {
    input
        .lines()
        .filter_map(parse_mount_line)
        .collect::<Vec<_>>()
}

fn parse_mount_line(line: &str) -> Option<MountEntry> {
    let (source, rest) = line.split_once(" on ")?;
    let (target, _) = rest.split_once(" (")?;

    Some(MountEntry {
        source: source.trim().to_string(),
        target: target.trim().to_string(),
    })
}

fn build_disk_mount_index(mounts: &[MountEntry]) -> HashMap<String, String> {
    let mut by_disk = HashMap::new();

    for mount in mounts {
        if let Some(disk) = extract_disk_from_device_path(&mount.source) {
            by_disk.entry(disk).or_insert_with(|| mount.target.clone());
        }
    }

    by_disk
}

fn extract_disk_from_device_path(path: &str) -> Option<String> {
    let name = path.strip_prefix("/dev/")?;
    if !name.starts_with("disk") {
        return None;
    }

    let whole = name.split('s').next().unwrap_or(name);
    Some(whole.to_string())
}

fn find_any_playdate_mount(mounts: &[MountEntry]) -> Option<String> {
    mounts
        .iter()
        .find(|m| m.target.eq_ignore_ascii_case("/Volumes/PLAYDATE"))
        .map(|m| m.target.clone())
}

fn find_mount_path_for_serial(
    serial: &str,
    serial_to_disks: &HashMap<String, Vec<String>>,
    disk_mounts: &HashMap<String, String>,
) -> Option<String> {
    let serial_key = normalize(serial);
    let disks = serial_to_disks.get(&serial_key)?;

    disks.iter().find_map(|disk| disk_mounts.get(disk).cloned())
}

#[cfg(target_os = "macos")]
fn list_macos_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    let output = Command::new("ioreg")
        .args(["-p", "IOService", "-r", "-n", "Playdate", "-l", "-w", "0"])
        .output()
        .map_err(|e| format!("failed to run ioreg: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ioreg failed: {stderr}"));
    }

    let text = String::from_utf8(output.stdout)
        .map_err(|e| format!("ioreg returned non-UTF8 output: {e}"))?;

    Ok(parse_macos_playdate_disks_by_serial(&text))
}

#[cfg(not(target_os = "macos"))]
fn list_macos_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    Ok(HashMap::new())
}

fn parse_macos_playdate_disks_by_serial(input: &str) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    let mut in_playdate = false;
    let mut saw_vendor = false;
    let mut saw_product = false;
    let mut serial = String::new();
    let mut disks: HashSet<String> = HashSet::new();

    let finalize = |result: &mut HashMap<String, HashSet<String>>,
                    saw_vendor: bool,
                    saw_product: bool,
                    serial: &str,
                    disks: &HashSet<String>| {
        if saw_vendor && saw_product && !serial.is_empty() && !disks.is_empty() {
            result
                .entry(normalize(serial))
                .or_default()
                .extend(disks.iter().cloned());
        }
    };

    for raw in input.lines() {
        let line = strip_ioreg_tree_prefix(raw);

        if line.contains("<class IOUSBHostDevice") {
            if in_playdate {
                finalize(&mut result, saw_vendor, saw_product, &serial, &disks);
            }

            in_playdate = line.contains("Playdate@");
            saw_vendor = false;
            saw_product = false;
            serial.clear();
            disks.clear();
            continue;
        }

        if !in_playdate {
            continue;
        }

        if line.starts_with('}') {
            finalize(&mut result, saw_vendor, saw_product, &serial, &disks);
            in_playdate = false;
            continue;
        }

        if line.contains("\"idVendor\" =") {
            if let Some(v) = parse_ioreg_u16_value(line) {
                saw_vendor = v == PLAYDATE_VENDOR_ID;
            }
            continue;
        }

        if line.contains("\"idProduct\" =") {
            if let Some(v) = parse_ioreg_u16_value(line) {
                saw_product = v == PLAYDATE_PRODUCT_ID;
            }
            continue;
        }

        if line.contains("\"kUSBSerialNumberString\" =") || line.contains("\"USB Serial Number\" =")
        {
            if let Some(s) = parse_ioreg_quoted_value(line) {
                serial = s;
            }
            continue;
        }

        if line.contains("\"BSD Name\" =") {
            if let Some(name) = parse_ioreg_quoted_value(line) {
                if let Some(disk) = extract_disk_name(&name) {
                    disks.insert(disk);
                }
            }
        }
    }

    if in_playdate {
        finalize(&mut result, saw_vendor, saw_product, &serial, &disks);
    }

    result
        .into_iter()
        .map(|(k, set)| {
            let mut v = set.into_iter().collect::<Vec<_>>();
            v.sort();
            (k, v)
        })
        .collect()
}

fn strip_ioreg_tree_prefix(line: &str) -> &str {
    let mut s = line.trim_start();
    while let Some(rest) = s.strip_prefix('|') {
        s = rest.trim_start();
    }
    s
}

fn parse_ioreg_u16_value(line: &str) -> Option<u16> {
    let (_, rhs) = line.split_once('=')?;
    let value = rhs.trim();
    let decimal = value.parse::<u32>().ok()?;
    u16::try_from(decimal).ok()
}

fn parse_ioreg_quoted_value(line: &str) -> Option<String> {
    let (_, rhs) = line.split_once('=')?;
    let value = rhs.trim();
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        return Some(value[1..value.len() - 1].to_string());
    }
    None
}

fn extract_disk_name(name: &str) -> Option<String> {
    if !name.starts_with("disk") {
        return None;
    }

    let whole = name.split('s').next().unwrap_or(name);
    Some(whole.to_string())
}

fn find_port_for_serial(serial: &str, ports: &[String]) -> Option<String> {
    let normalized_serial = normalize(serial);

    ports
        .iter()
        .find(|port| normalize(port).contains(&normalized_serial))
        .cloned()
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn print_devices(devices: &[Device]) {
    let header_device = "device";
    let header_port = "port";
    let header_mounted = "mounted";
    let header_mount_path = "mount_path";

    let device_width = devices
        .iter()
        .map(|d| d.device.len())
        .max()
        .unwrap_or(0)
        .max(header_device.len());

    let port_width = devices
        .iter()
        .map(|d| d.port.len())
        .max()
        .unwrap_or(0)
        .max(header_port.len());

    println!(
        "{:<device_width$}  {:<port_width$}  {:<7}  {}",
        header_device,
        header_port,
        header_mounted,
        header_mount_path,
        device_width = device_width,
        port_width = port_width
    );

    for d in devices {
        let mounted = if d.mounted { "yes" } else { "no" };
        println!(
            "{:<device_width$}  {:<port_width$}  {:<7}  {}",
            d.device,
            d.port,
            mounted,
            d.mount_path,
            device_width = device_width,
            port_width = port_width
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_disk_mount_index, extract_disk_from_device_path, find_mount_path_for_serial,
        find_port_for_serial, normalize, parse_macos_playdate_disks_by_serial, parse_mount_entries,
        MountEntry,
    };
    use std::collections::HashMap;

    #[test]
    fn normalize_ignores_punctuation() {
        assert_eq!(normalize("PDU1-Y013705"), "pdu1y013705");
        assert_eq!(
            normalize("/dev/cu.usbmodemPDU1_Y013705"),
            "devcuusbmodempdu1y013705"
        );
    }

    #[test]
    fn finds_matching_port_for_serial() {
        let ports = vec![
            "/dev/cu.usbmodemPDU1_Y013705".to_string(),
            "/dev/ttyACM0".to_string(),
        ];

        let got = find_port_for_serial("PDU1-Y013705", &ports);
        assert_eq!(got.as_deref(), Some("/dev/cu.usbmodemPDU1_Y013705"));
    }

    #[test]
    fn parses_mount_entries() {
        let input = r#"
/dev/disk8s1 on /Volumes/PLAYDATE (msdos, local)
/dev/disk1s1 on /System/Volumes/Data (apfs, local)
"#;

        let mounts = parse_mount_entries(input);
        assert_eq!(
            mounts,
            vec![
                MountEntry {
                    source: "/dev/disk8s1".to_string(),
                    target: "/Volumes/PLAYDATE".to_string()
                },
                MountEntry {
                    source: "/dev/disk1s1".to_string(),
                    target: "/System/Volumes/Data".to_string()
                }
            ]
        );
    }

    #[test]
    fn extracts_whole_disk_name() {
        assert_eq!(
            extract_disk_from_device_path("/dev/disk8s1").as_deref(),
            Some("disk8")
        );
        assert_eq!(
            extract_disk_from_device_path("/dev/disk8").as_deref(),
            Some("disk8")
        );
        assert_eq!(extract_disk_from_device_path("devfs"), None);
    }

    #[test]
    fn parses_playdate_disks_by_serial_from_ioreg() {
        let input = r#"
+-o Playdate@14400000  <class IOUSBHostDevice, id 0x1>
  | {
  |   "idProduct" = 22337
  |   "kUSBSerialNumberString" = "PDU1-Y013705"
  |   "idVendor" = 4913
  | }
  | +-o Panic Playdate Media  <class IOMedia, id 0x2>
  |   | {
  |   |   "BSD Name" = "disk8"
  |   | }
  |   +-o Untitled 1@1  <class IOMedia, id 0x3>
  |     | {
  |     |   "BSD Name" = "disk8s1"
  |     | }
"#;

        let map = parse_macos_playdate_disks_by_serial(input);
        let disks = map.get("pdu1y013705").cloned().unwrap_or_default();
        assert_eq!(disks, vec!["disk8".to_string()]);
    }

    #[test]
    fn finds_mount_path_for_serial() {
        let mut serial_to_disks = HashMap::new();
        serial_to_disks.insert("pdu1y013705".to_string(), vec!["disk8".to_string()]);

        let mounts = vec![
            MountEntry {
                source: "/dev/disk8s1".to_string(),
                target: "/Volumes/PLAYDATE".to_string(),
            },
            MountEntry {
                source: "/dev/disk1s1".to_string(),
                target: "/System/Volumes/Data".to_string(),
            },
        ];

        let index = build_disk_mount_index(&mounts);
        let path = find_mount_path_for_serial("PDU1-Y013705", &serial_to_disks, &index);

        assert_eq!(path.as_deref(), Some("/Volumes/PLAYDATE"));
    }
}
