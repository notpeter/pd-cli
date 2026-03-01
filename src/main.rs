mod cli;
mod screenshot;
mod stats;

use crate::cli::{DeviceCommand, parse_cli_from_env};
use crate::screenshot::capture_screenshot;
use crate::stats::{fetch_stats, parse_metric_value, print_stats_json};
use nusb::MaybeFuture;
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::fd::AsRawFd;
use std::process::Command;
use std::time::{Duration, Instant};

const PLAYDATE_VENDOR_ID: u16 = 0x1331;
const PLAYDATE_PRODUCT_ID_MSC: u16 = 0x5741;
const PLAYDATE_PRODUCT_ID_APP: u16 = 0x5740;
const SCREEN_CAPTURE_TIMEOUT: Duration = Duration::from_secs(2);
const SCREEN_CAPTURE_IDLE: Duration = Duration::from_millis(300);
const MOUNT_WAIT_TIMEOUT: Duration = Duration::from_secs(25);
const MOUNT_WAIT_POLL: Duration = Duration::from_millis(250);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Device {
    device: String,
    port: String,
    mounted: bool,
    mount_path: String,
    disk: String,
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
    let command = parse_cli_from_env()?;
    run_device_command(command)
}

fn run_device_command(command: DeviceCommand) -> Result<(), String> {
    match command {
        DeviceCommand::List => {
            let devices = list_devices()?;
            print_devices(&devices);
        }
        DeviceCommand::Eject { device_id } => {
            let serial = eject_device(device_id.as_deref())?;
            println!("ejected {serial}");
        }
        DeviceCommand::Serial { device_id, command } => {
            let (serial, port) = send_serial_command_to_device(device_id.as_deref(), &command)?;
            println!("sent '{command}' to {serial} on {port}");
        }
        DeviceCommand::Mount { device_id } => {
            let (serial, mount_path) = mount_device(device_id.as_deref())?;
            println!("mounted {serial} at {mount_path}");
        }
        DeviceCommand::Screenshot {
            device_id,
            filename,
            open,
        } => {
            let (serial, path, bytes, inspect) =
                capture_screenshot(device_id.as_deref(), filename.as_deref())?;
            println!("captured screenshot from {serial} to {path} ({bytes} bytes)");
            println!("{inspect}");
            if open {
                open_with_default_viewer(&path)?;
                println!("opened {path}");
            }
        }
        DeviceCommand::Stats { device_id, json } => {
            let (serial, entries) = fetch_stats(device_id.as_deref())?;
            if json {
                print_stats_json(&entries);
            } else {
                println!("stats from {serial}");
                for (k, v) in entries {
                    println!("{k}: {v}");
                }
            }
        }
        DeviceCommand::Hibernate { device_id } => {
            let (serial, port) = send_serial_command_to_device(device_id.as_deref(), "hibernate")?;
            println!("sent 'hibernate' to {serial} on {port}");
        }
    }
    Ok(())
}

fn send_serial_command_to_device(
    device_id: Option<&str>,
    command: &str,
) -> Result<(String, String), String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    if device.port.is_empty() {
        return Err(format!(
            "device '{}' has no serial port available; reconnect in serial mode and try again",
            device.device
        ));
    }

    send_serial_command(&device.port, command)?;
    Ok((device.device, device.port))
}

fn mount_device(device_id: Option<&str>) -> Result<(String, String), String> {
    let start = Instant::now();

    loop {
        let devices = list_devices().unwrap_or_default();
        match select_mount_target(&devices, device_id) {
            Ok(Some(device)) => {
                if device.port.is_empty() {
                    let mount_path = wait_for_mount_ready(&device.device)?;
                    return Ok((device.device, mount_path));
                }

                send_serial_command(&device.port, "datadisk")?;
                let mount_path = wait_for_mount_ready(&device.device)?;
                return Ok((device.device, mount_path));
            }
            Ok(None) => {}
            Err(e) => return Err(e),
        }

        if Instant::now().duration_since(start) >= MOUNT_WAIT_TIMEOUT {
            return Err(
                "timed out waiting for device before mount: no Playdate devices found".to_string(),
            );
        }

        std::thread::sleep(MOUNT_WAIT_POLL);
    }
}

fn select_mount_target(
    devices: &[Device],
    device_id: Option<&str>,
) -> Result<Option<Device>, String> {
    match device_id {
        Some(id) => {
            let needle = normalize(id);
            Ok(devices
                .iter()
                .find(|d| normalize(&d.device) == needle)
                .cloned())
        }
        None => match devices.len() {
            0 => Ok(None),
            1 => Ok(Some(devices[0].clone())),
            _ => Err("multiple Playdate devices found; specify one with `-d <serial>`".to_string()),
        },
    }
}

fn wait_for_mount_ready(serial: &str) -> Result<String, String> {
    let start = Instant::now();
    let mut last_seen_mount: Option<String> = None;
    let mut last_error: Option<String> = None;

    loop {
        let mounts = list_mounts().unwrap_or_default();
        if let Some(path) = find_mount_path_for_serial_live(serial, &mounts) {
            last_seen_mount = Some(path.clone());
            match std::fs::read_dir(&path) {
                Ok(mut iter) => {
                    let _ = iter.next();
                    return Ok(path);
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                }
            }
        }

        if Instant::now().duration_since(start) >= MOUNT_WAIT_TIMEOUT {
            let seen = last_seen_mount.unwrap_or_else(|| "<not mounted>".to_string());
            let err = last_error.unwrap_or_else(|| "mount not yet readable".to_string());
            return Err(format!(
                "timed out waiting for Playdate data disk to become readable (last mount: {seen}, last error: {err})"
            ));
        }

        std::thread::sleep(MOUNT_WAIT_POLL);
    }
}

fn find_mount_path_for_serial_live(serial: &str, mounts: &[MountEntry]) -> Option<String> {
    let disk_mounts = build_disk_mount_index(mounts);

    #[cfg(target_os = "macos")]
    {
        let serial_to_disks = list_macos_playdate_disks_by_serial().unwrap_or_default();
        if let Some(path) = find_mount_path_for_serial(serial, &serial_to_disks, &disk_mounts) {
            return Some(path);
        }
    }

    find_any_playdate_mount(mounts)
}

pub(crate) fn query_metric(port_path: &str, command: &str) -> Option<String> {
    let payload = send_serial_command_and_capture(port_path, command).ok()?;
    let raw = String::from_utf8_lossy(&payload);
    parse_metric_value(command, &raw)
}

fn send_serial_command(port_path: &str, command: &str) -> Result<(), String> {
    #[cfg(unix)]
    {
        let mut port = open_serial_port(port_path)?;

        let payload = format!("{command}\n");
        port.write_all(payload.as_bytes())
            .map_err(|e| format!("failed to write command to '{port_path}': {e}"))?;
        port.flush()
            .map_err(|e| format!("failed to flush command to '{port_path}': {e}"))?;
        return Ok(());
    }

    #[cfg(not(unix))]
    {
        let _ = (port_path, command);
        Err("serial command is not supported on this platform yet".to_string())
    }
}

pub(crate) fn send_serial_command_and_capture(
    port_path: &str,
    command: &str,
) -> Result<Vec<u8>, String> {
    #[cfg(unix)]
    {
        let mut port = open_serial_port(port_path)?;
        let payload = format!("{command}\n");
        port.write_all(payload.as_bytes())
            .map_err(|e| format!("failed to write command to '{port_path}': {e}"))?;
        port.flush()
            .map_err(|e| format!("failed to flush command to '{port_path}': {e}"))?;

        let mut out = Vec::new();
        let mut buf = [0u8; 4096];
        let start = Instant::now();
        let mut last_read = Instant::now();

        loop {
            match port.read(&mut buf) {
                Ok(n) if n > 0 => {
                    out.extend_from_slice(&buf[..n]);
                    last_read = Instant::now();
                }
                Ok(_) => {}
                Err(e)
                    if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    return Err(format!(
                        "failed to read serial data from '{port_path}': {e}"
                    ));
                }
            }

            if !out.is_empty() && Instant::now().duration_since(last_read) >= SCREEN_CAPTURE_IDLE {
                break;
            }

            if Instant::now().duration_since(start) >= SCREEN_CAPTURE_TIMEOUT {
                break;
            }
        }

        if out.is_empty() {
            return Err(format!(
                "no screenshot data received from '{}'; verify the device is unlocked and in app mode",
                port_path
            ));
        }

        return Ok(out);
    }

    #[cfg(not(unix))]
    {
        let _ = (port_path, command);
        Err("screenshot capture is not supported on this platform yet".to_string())
    }
}

#[cfg(unix)]
fn open_serial_port(port_path: &str) -> Result<std::fs::File, String> {
    let port = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(port_path)
        .map_err(|e| format!("failed to open serial port '{port_path}': {e}"))?;

    configure_serial_port_8n1_115200(port.as_raw_fd())
        .map_err(|e| format!("failed to configure serial port '{port_path}': {e}"))?;

    Ok(port)
}

#[cfg(unix)]
fn configure_serial_port_8n1_115200(fd: std::os::fd::RawFd) -> Result<(), std::io::Error> {
    let mut term = unsafe { std::mem::zeroed::<libc::termios>() };

    if unsafe { libc::tcgetattr(fd, &mut term) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    if unsafe { libc::cfsetispeed(&mut term, libc::B115200) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    if unsafe { libc::cfsetospeed(&mut term, libc::B115200) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    term.c_iflag = 0;
    term.c_oflag = 0;
    term.c_lflag = 0;
    term.c_cflag &= !(libc::PARENB | libc::CSTOPB | libc::CSIZE | libc::CRTSCTS);
    term.c_cflag |= libc::CS8 | libc::CLOCAL | libc::CREAD;
    term.c_cc[libc::VMIN] = 0;
    term.c_cc[libc::VTIME] = 1;

    if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &term) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

fn open_with_default_viewer(path: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let status = Command::new("open")
            .arg(path)
            .status()
            .map_err(|e| format!("failed to run open: {e}"))?;
        if status.success() {
            return Ok(());
        }
        return Err(format!("open failed for '{path}'"));
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let status = Command::new("xdg-open")
            .arg(path)
            .status()
            .map_err(|e| format!("failed to run xdg-open: {e}"))?;
        if status.success() {
            return Ok(());
        }
        return Err(format!("xdg-open failed for '{path}'"));
    }

    #[cfg(target_os = "windows")]
    {
        let status = Command::new("cmd")
            .args(["/C", "start", "", path])
            .status()
            .map_err(|e| format!("failed to run start: {e}"))?;
        if status.success() {
            return Ok(());
        }
        return Err(format!("start failed for '{path}'"));
    }
}

fn eject_device(device_id: Option<&str>) -> Result<String, String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    eject_target(&device.disk, &device.mount_path)?;
    Ok(device.device)
}

pub(crate) fn resolve_device(
    devices: Vec<Device>,
    device_id: Option<&str>,
) -> Result<Device, String> {
    match device_id {
        Some(id) => {
            let needle = normalize(id);
            devices
                .into_iter()
                .find(|d| normalize(&d.device) == needle)
                .ok_or_else(|| {
                    format!(
                        "device '{id}' not found; run `pd device list` to see available devices"
                    )
                })
        }
        None => match devices.len() {
            0 => Err("no Playdate devices found".to_string()),
            1 => Ok(devices[0].clone()),
            _ => Err("multiple Playdate devices found; specify one with `-d <serial>`".to_string()),
        },
    }
}

#[cfg(target_os = "macos")]
fn eject_target(disk: &str, mount_path: &str) -> Result<(), String> {
    let target = if !disk.is_empty() {
        format!("/dev/{disk}")
    } else if !mount_path.is_empty() {
        mount_path.to_string()
    } else {
        return Err("device has no known disk or mount path to eject".to_string());
    };

    let output = Command::new("diskutil")
        .args(["eject", &target])
        .output()
        .map_err(|e| format!("failed to run diskutil: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    Err(format!("failed to eject '{target}': {stdout}{stderr}"))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn eject_target(_disk: &str, mount_path: &str) -> Result<(), String> {
    if mount_path.is_empty() {
        return Err("device is not mounted; cannot eject on this platform".to_string());
    }

    let output = Command::new("umount")
        .arg(mount_path)
        .output()
        .map_err(|e| format!("failed to run umount: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(format!("failed to unmount '{mount_path}': {stderr}"))
}

#[cfg(not(unix))]
fn eject_target(_disk: &str, _mount_path: &str) -> Result<(), String> {
    Err("eject is not supported on this platform yet".to_string())
}

pub(crate) fn list_devices() -> Result<Vec<Device>, String> {
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
        if usb.vendor_id() != PLAYDATE_VENDOR_ID
            || (usb.product_id() != PLAYDATE_PRODUCT_ID_MSC
                && usb.product_id() != PLAYDATE_PRODUCT_ID_APP)
        {
            continue;
        }

        let serial = usb.serial_number().unwrap_or("unknown").to_string();
        let disk = serial_to_disks
            .get(&normalize(&serial))
            .and_then(|v| v.first().cloned())
            .unwrap_or_default();
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
            disk,
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

    let mounts = text
        .lines()
        .filter_map(|line| {
            let (source, rest) = line.split_once(" on ")?;
            let (target, _) = rest.split_once(" (")?;
            Some(MountEntry {
                source: source.trim().to_string(),
                target: target.trim().to_string(),
            })
        })
        .collect::<Vec<_>>();

    Ok(mounts)
}

#[cfg(not(unix))]
fn list_mounts() -> Result<Vec<MountEntry>, String> {
    Ok(Vec::new())
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
    extract_disk_name(name)
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

        if line.contains("\"idVendor\" =") {
            if let Some(v) = parse_ioreg_u16_value(line) {
                saw_vendor = v == PLAYDATE_VENDOR_ID;
            }
            continue;
        }

        if line.contains("\"idProduct\" =") {
            if let Some(v) = parse_ioreg_u16_value(line) {
                saw_product = v == PLAYDATE_PRODUCT_ID_MSC || v == PLAYDATE_PRODUCT_ID_APP;
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

    let suffix = &name["disk".len()..];
    let digits: String = suffix.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }

    Some(format!("disk{digits}"))
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
        Device, MountEntry, build_disk_mount_index, extract_disk_from_device_path,
        find_mount_path_for_serial, find_port_for_serial, normalize,
        parse_macos_playdate_disks_by_serial, resolve_device,
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

    #[test]
    fn resolve_device_auto_selects_single_device() {
        let devices = vec![Device {
            device: "PDU1-Y013705".to_string(),
            port: "/dev/cu.usbmodemPDU1_Y013705".to_string(),
            mounted: false,
            mount_path: String::new(),
            disk: "disk8".to_string(),
        }];

        let resolved = resolve_device(devices, None).expect("expected single device to resolve");
        assert_eq!(resolved.device, "PDU1-Y013705");
    }

    #[test]
    fn resolve_device_requires_flag_when_multiple_devices() {
        let devices = vec![
            Device {
                device: "PDU1-Y013705".to_string(),
                port: String::new(),
                mounted: false,
                mount_path: String::new(),
                disk: String::new(),
            },
            Device {
                device: "PDU1-Y013706".to_string(),
                port: String::new(),
                mounted: false,
                mount_path: String::new(),
                disk: String::new(),
            },
        ];

        let err = resolve_device(devices, None).expect_err("expected multiple-device error");
        assert!(err.contains("multiple Playdate devices found"));
    }
}
