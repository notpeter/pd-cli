use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

const DEV_DIR: &str = "/dev";
const USB_MODEM_PREFIX: &str = "cu.usbmodem";
const PLAYDATE_VENDOR_ID: &str = "0x1331";
const PLAYDATE_PRODUCT_ID: &str = "0x5741";
const PLAYDATE_MANUFACTURER: &str = "panic inc";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Device {
    device: String,
    port: String,
}

#[derive(Debug, Clone, Default)]
struct UsbBlock {
    title: String,
    fields: HashMap<String, String>,
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
    let ioreg_output = read_ioreg_usb_output()?;
    let blocks = parse_ioreg_usb_blocks(&ioreg_output);
    let serial_ports = list_serial_ports().map_err(|e| e.to_string())?;

    let mut devices = Vec::new();

    for block in blocks {
        if !is_playdate_block(&block) {
            continue;
        }

        let serial = block
            .fields
            .get("USB Serial Number")
            .or_else(|| block.fields.get("kUSBSerialNumberString"))
            .cloned()
            .unwrap_or_else(|| block.title.clone());

        let port = find_port_for_serial(&serial, &serial_ports).unwrap_or_default();

        devices.push(Device {
            device: serial,
            port,
        });
    }

    devices.sort_by(|a, b| a.device.cmp(&b.device));
    devices.dedup();
    Ok(devices)
}

fn read_ioreg_usb_output() -> Result<String, String> {
    let output = Command::new("ioreg")
        .args(["-p", "IOUSB", "-l", "-w", "0"])
        .output()
        .map_err(|e| format!("failed to run ioreg: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ioreg failed: {stderr}"));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("ioreg returned non-UTF8 output: {e}"))
}

fn parse_ioreg_usb_blocks(input: &str) -> Vec<UsbBlock> {
    let mut blocks = Vec::new();
    let mut current: Option<UsbBlock> = None;
    let mut in_usb_host_device = false;

    for line in input.lines() {
        let trimmed = strip_ioreg_tree_prefix(line);

        if !in_usb_host_device && trimmed.contains("<class IOUSBHostDevice") {
            in_usb_host_device = true;
            current = Some(UsbBlock {
                title: parse_title_from_ioreg_header(trimmed),
                fields: HashMap::new(),
            });
            continue;
        }

        if !in_usb_host_device {
            continue;
        }

        if trimmed == "}" {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
            in_usb_host_device = false;
            continue;
        }

        if let Some((key, value)) = parse_ioreg_key_value_line(trimmed) {
            if let Some(block) = current.as_mut() {
                block.fields.insert(key, value);
            }
        }
    }

    blocks
}

fn parse_title_from_ioreg_header(header: &str) -> String {
    let left = header.split('@').next().unwrap_or(header).trim();
    let title = left.rsplit(' ').next().unwrap_or("Unknown");
    title.to_string()
}

fn strip_ioreg_tree_prefix(line: &str) -> &str {
    let mut s = line.trim_start();
    while let Some(rest) = s.strip_prefix('|') {
        s = rest.trim_start();
    }
    s
}

fn parse_ioreg_key_value_line(trimmed_line: &str) -> Option<(String, String)> {
    if !trimmed_line.starts_with('"') {
        return None;
    }

    let key_end = trimmed_line[1..].find('"')? + 1;
    let key = trimmed_line.get(1..key_end)?.to_string();
    let (_, rhs) = trimmed_line.split_once('=')?;
    let value = normalize_ioreg_value(rhs.trim());

    if value.is_empty() {
        return None;
    }

    Some((key, value))
}

fn normalize_ioreg_value(raw: &str) -> String {
    let value = raw.trim();
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        return value[1..value.len() - 1].to_string();
    }

    if let Ok(decimal) = value.parse::<u32>() {
        return format!("0x{decimal:04x}");
    }

    value.to_string()
}

fn is_playdate_block(block: &UsbBlock) -> bool {
    let vendor = block
        .fields
        .get("idVendor")
        .or_else(|| block.fields.get("USB Vendor ID"))
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();
    let product = block
        .fields
        .get("idProduct")
        .or_else(|| block.fields.get("USB Product ID"))
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();
    let manufacturer = block
        .fields
        .get("USB Vendor Name")
        .or_else(|| block.fields.get("Manufacturer"))
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();

    vendor.contains(PLAYDATE_VENDOR_ID)
        && product.contains(PLAYDATE_PRODUCT_ID)
        && manufacturer.contains(PLAYDATE_MANUFACTURER)
}

fn list_serial_ports() -> io::Result<Vec<String>> {
    let mut ports = Vec::new();

    for entry in fs::read_dir(Path::new(DEV_DIR))? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !name.starts_with(USB_MODEM_PREFIX) {
            continue;
        }

        ports.push(format!("{DEV_DIR}/{name}"));
    }

    ports.sort();
    Ok(ports)
}

fn find_port_for_serial(serial: &str, ports: &[String]) -> Option<String> {
    let normalized_serial = serial.to_ascii_lowercase();

    ports
        .iter()
        .find(|port| {
            let name = port.rsplit('/').next().unwrap_or("");
            port_to_serial(name)
                .to_ascii_lowercase()
                .contains(&normalized_serial)
        })
        .cloned()
}

fn port_to_serial(port_name: &str) -> String {
    let raw = port_name
        .strip_prefix(USB_MODEM_PREFIX)
        .unwrap_or(port_name)
        .trim_start_matches('_');

    raw.replace('_', "-")
}

fn print_devices(devices: &[Device]) {
    let header_device = "device";
    let header_port = "port";

    let device_width = devices
        .iter()
        .map(|d| d.device.len())
        .max()
        .unwrap_or(0)
        .max(header_device.len());

    println!(
        "{:<device_width$}  {}",
        header_device,
        header_port,
        device_width = device_width
    );

    for d in devices {
        println!(
            "{:<device_width$}  {}",
            d.device,
            d.port,
            device_width = device_width
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{find_port_for_serial, parse_ioreg_usb_blocks, port_to_serial};

    #[test]
    fn parses_playdate_ioreg_block() {
        let input = r#"
  +-o Playdate@14400000  <class IOUSBHostDevice, id 0x1003a223d, registered, matched, active, busy 0 (332 ms), retain 28>
    {
      "idProduct" = 22337
      "kUSBSerialNumberString" = "PDU1-Y013705"
      "idVendor" = 4913
      "USB Serial Number" = "PDU1-Y013705"
      "USB Vendor Name" = "Panic Inc"
    }
"#;

        let blocks = parse_ioreg_usb_blocks(input);
        let playdate = blocks
            .into_iter()
            .find(|b| b.title == "Playdate")
            .expect("expected Playdate block");

        assert_eq!(
            playdate.fields.get("USB Serial Number").map(String::as_str),
            Some("PDU1-Y013705")
        );
        assert_eq!(
            playdate.fields.get("idVendor").map(String::as_str),
            Some("0x1331")
        );
    }

    #[test]
    fn converts_port_name_to_device_serial_shape() {
        let got = port_to_serial("cu.usbmodemPDU1_Y0137051");
        assert_eq!(got, "PDU1-Y0137051");
    }

    #[test]
    fn finds_matching_port_for_serial() {
        let ports = vec![
            "/dev/cu.usbmodemPDU1_Y0137051".to_string(),
            "/dev/cu.usbmodemrandom".to_string(),
        ];

        let got = find_port_for_serial("PDU1-Y0137051", &ports);
        assert_eq!(got.as_deref(), Some("/dev/cu.usbmodemPDU1_Y0137051"));
    }
}
