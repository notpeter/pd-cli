use image::{DynamicImage, GrayImage, ImageBuffer, ImageFormat, Luma};
use nusb::MaybeFuture;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::fd::AsRawFd;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

const PLAYDATE_VENDOR_ID: u16 = 0x1331;
const PLAYDATE_PRODUCT_ID_MSC: u16 = 0x5741;
const PLAYDATE_PRODUCT_ID_APP: u16 = 0x5740;
const SCREEN_PREFIX: &[u8] = b"screen\r\n~screen:\n";
const SCREEN_PREFIX_LEGACY: &[u8] = b"\r\nscreen~:\n";
const SCREEN_BITMAP_BYTES: usize = 12_000;
const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 240;
const SCREEN_CAPTURE_TIMEOUT: Duration = Duration::from_secs(2);
const SCREEN_CAPTURE_IDLE: Duration = Duration::from_millis(300);
const DEVICE_USAGE: &str = "usage: pd device list | pd device [-d <serial>] eject | pd device [-d <serial>] mount | pd device [-d <serial>] datadisk | pd device [-d <serial>] serial <command> | pd device [-d <serial>] stats [--json] | pd device [-d <serial>] screenshot [-f <filename>] [--open]";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Device {
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeviceCommand {
    List,
    Eject {
        device_id: Option<String>,
    },
    Serial {
        device_id: Option<String>,
        command: String,
    },
    Screenshot {
        device_id: Option<String>,
        filename: Option<String>,
        open: bool,
    },
    Stats {
        device_id: Option<String>,
        json: bool,
    },
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
        Some("device") => run_device_command(&args[2..]),
        _ => Err(DEVICE_USAGE.to_string()),
    }
}

fn run_device_command(args: &[String]) -> Result<(), String> {
    match parse_device_command(args)? {
        DeviceCommand::List => {
            let devices = list_devices()?;
            print_devices(&devices);
            Ok(())
        }
        DeviceCommand::Eject { device_id } => {
            let serial = eject_device(device_id.as_deref())?;
            println!("ejected {serial}");
            Ok(())
        }
        DeviceCommand::Serial { device_id, command } => {
            let (serial, port) = send_serial_command_to_device(device_id.as_deref(), &command)?;
            println!("sent '{command}' to {serial} on {port}");
            Ok(())
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
            Ok(())
        }
        DeviceCommand::Stats { device_id, json } => {
            let (serial, entries, raw) = fetch_stats(device_id.as_deref())?;
            if json {
                print_stats_json(&entries);
            } else if entries.is_empty() {
                println!("stats from {serial}");
                println!("{}", raw.trim());
            } else {
                println!("stats from {serial}");
                for (k, v) in entries {
                    println!("{k}: {v}");
                }
            }
            Ok(())
        }
    }
}

fn parse_device_command(args: &[String]) -> Result<DeviceCommand, String> {
    match args {
        [command] if command == "list" => Ok(DeviceCommand::List),
        [flag, device_id, command]
            if (flag == "-d" || flag == "--device")
                && (command == "eject" || command == "unmount") =>
        {
            Ok(DeviceCommand::Eject {
                device_id: Some(device_id.clone()),
            })
        }
        [command, flag, device_id]
            if (command == "eject" || command == "unmount")
                && (flag == "-d" || flag == "--device") =>
        {
            Ok(DeviceCommand::Eject {
                device_id: Some(device_id.clone()),
            })
        }
        [flag, device_id, command]
            if (flag == "-d" || flag == "--device")
                && (command == "mount" || command == "datadisk") =>
        {
            Ok(DeviceCommand::Serial {
                device_id: Some(device_id.clone()),
                command: "datadisk".to_string(),
            })
        }
        [command, flag, device_id]
            if (command == "mount" || command == "datadisk")
                && (flag == "-d" || flag == "--device") =>
        {
            Ok(DeviceCommand::Serial {
                device_id: Some(device_id.clone()),
                command: "datadisk".to_string(),
            })
        }
        [flag, device_id, serial_keyword, command]
            if (flag == "-d" || flag == "--device") && serial_keyword == "serial" =>
        {
            Ok(DeviceCommand::Serial {
                device_id: Some(device_id.clone()),
                command: command.clone(),
            })
        }
        [serial_keyword, flag, device_id, command]
            if serial_keyword == "serial" && (flag == "-d" || flag == "--device") =>
        {
            Ok(DeviceCommand::Serial {
                device_id: Some(device_id.clone()),
                command: command.clone(),
            })
        }
        [command] if command == "eject" || command == "unmount" => {
            Ok(DeviceCommand::Eject { device_id: None })
        }
        [command] if command == "mount" || command == "datadisk" => Ok(DeviceCommand::Serial {
            device_id: None,
            command: "datadisk".to_string(),
        }),
        [serial_keyword, command] if serial_keyword == "serial" => Ok(DeviceCommand::Serial {
            device_id: None,
            command: command.clone(),
        }),
        _ if args.iter().any(|arg| arg == "stats") => parse_stats_command(args),
        _ if args.iter().any(|arg| arg == "screenshot") => parse_screenshot_command(args),
        _ => Err(DEVICE_USAGE.to_string()),
    }
}

fn parse_stats_command(args: &[String]) -> Result<DeviceCommand, String> {
    let stats_count = args.iter().filter(|arg| arg.as_str() == "stats").count();
    if stats_count != 1 {
        return Err(DEVICE_USAGE.to_string());
    }

    let mut device_id: Option<String> = None;
    let mut json = false;
    let mut saw_stats = false;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "stats" => {
                saw_stats = true;
                i += 1;
            }
            "-d" | "--device" => {
                let value = args.get(i + 1).ok_or_else(|| DEVICE_USAGE.to_string())?;
                device_id = Some(value.clone());
                i += 2;
            }
            "--json" => {
                json = true;
                i += 1;
            }
            _ => return Err(DEVICE_USAGE.to_string()),
        }
    }

    if !saw_stats {
        return Err(DEVICE_USAGE.to_string());
    }

    Ok(DeviceCommand::Stats { device_id, json })
}

fn parse_screenshot_command(args: &[String]) -> Result<DeviceCommand, String> {
    let screenshot_count = args
        .iter()
        .filter(|arg| arg.as_str() == "screenshot")
        .count();
    if screenshot_count != 1 {
        return Err(DEVICE_USAGE.to_string());
    }

    let mut device_id: Option<String> = None;
    let mut filename: Option<String> = None;
    let mut open = false;
    let mut saw_screenshot = false;

    let mut i = 0usize;
    while i < args.len() {
        let token = args[i].as_str();
        match token {
            "screenshot" => {
                saw_screenshot = true;
                i += 1;
            }
            "-d" | "--device" => {
                let value = args.get(i + 1).ok_or_else(|| DEVICE_USAGE.to_string())?;
                device_id = Some(value.clone());
                i += 2;
            }
            "-f" => {
                let value = args.get(i + 1).ok_or_else(|| DEVICE_USAGE.to_string())?;
                filename = Some(value.clone());
                i += 2;
            }
            "--open" => {
                open = true;
                i += 1;
            }
            _ => return Err(DEVICE_USAGE.to_string()),
        }
    }

    if !saw_screenshot {
        return Err(DEVICE_USAGE.to_string());
    }

    Ok(DeviceCommand::Screenshot {
        device_id,
        filename,
        open,
    })
}

fn capture_screenshot(
    device_id: Option<&str>,
    filename: Option<&str>,
) -> Result<(String, String, usize, String), String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    if device.port.is_empty() {
        return Err(format!(
            "device '{}' has no serial port available; reconnect in serial mode and try again",
            device.device
        ));
    }

    let payload = send_serial_command_and_capture(&device.port, "screen")?;
    let path = filename
        .map(ToOwned::to_owned)
        .unwrap_or_else(default_screenshot_filename);
    write_screenshot_file(&path, &payload)?;

    let inspect = inspect_screen_payload(&payload, &path);
    Ok((device.device, path, payload.len(), inspect))
}

fn write_screenshot_file(path: &str, payload: &[u8]) -> Result<(), String> {
    match screenshot_format_for_path(path)? {
        Some(ImageFormat::Png) => {
            let bitmap = extract_screen_bitmap(payload)?;
            let image = bitmap_to_image(bitmap);
            DynamicImage::ImageLuma8(image)
                .save_with_format(path, ImageFormat::Png)
                .map_err(|e| format!("failed to write screenshot image '{path}': {e}"))?;
            Ok(())
        }
        Some(ImageFormat::Gif) => {
            let bitmap = extract_screen_bitmap(payload)?;
            let image = bitmap_to_image(bitmap);
            DynamicImage::ImageLuma8(image)
                .into_rgb8()
                .save_with_format(path, ImageFormat::Gif)
                .map_err(|e| format!("failed to write screenshot image '{path}': {e}"))?;
            Ok(())
        }
        Some(other) => Err(format!("unsupported image output format: {other:?}")),
        None => std::fs::write(path, payload)
            .map_err(|e| format!("failed to write screenshot file '{path}': {e}")),
    }
}

fn screenshot_format_for_path(path: &str) -> Result<Option<ImageFormat>, String> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase());

    match ext.as_deref() {
        Some("png") => Ok(Some(ImageFormat::Png)),
        Some("gif") => Ok(Some(ImageFormat::Gif)),
        Some("raw") | Some("bin") | None => Ok(None),
        Some(other) => Err(format!(
            "unsupported screenshot extension '.{other}'; use .png, .gif, .raw, or .bin"
        )),
    }
}

fn extract_screen_bitmap(payload: &[u8]) -> Result<&[u8], String> {
    if let Some(offset) = find_subslice(payload, SCREEN_PREFIX) {
        let start = offset + SCREEN_PREFIX.len();
        let end = start + SCREEN_BITMAP_BYTES;
        if payload.len() < end {
            return Err(format!(
                "screen payload is incomplete: got {} bytes, expected at least {}",
                payload.len().saturating_sub(start),
                SCREEN_BITMAP_BYTES
            ));
        }
        return Ok(&payload[start..end]);
    }

    if let Some(offset) = find_subslice(payload, SCREEN_PREFIX_LEGACY) {
        let start = offset + SCREEN_PREFIX_LEGACY.len();
        let end = start + SCREEN_BITMAP_BYTES;
        if payload.len() < end {
            return Err(format!(
                "legacy screen payload is incomplete: got {} bytes, expected at least {}",
                payload.len().saturating_sub(start),
                SCREEN_BITMAP_BYTES
            ));
        }
        return Ok(&payload[start..end]);
    }

    if payload.len() >= SCREEN_BITMAP_BYTES {
        return Ok(&payload[..SCREEN_BITMAP_BYTES]);
    }

    Err("no recognizable screen payload in serial output".to_string())
}

fn bitmap_to_image(bitmap: &[u8]) -> GrayImage {
    let mut img: GrayImage = ImageBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let stride = (SCREEN_WIDTH / 8) as usize;

    for y in 0..SCREEN_HEIGHT as usize {
        let row = &bitmap[y * stride..(y + 1) * stride];
        for x in 0..SCREEN_WIDTH as usize {
            let byte = row[x / 8];
            let bit = 7 - (x % 8);
            let is_white = ((byte >> bit) & 1) == 1;
            let value = if is_white { 255 } else { 0 };
            img.put_pixel(x as u32, y as u32, Luma([value]));
        }
    }

    img
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

fn fetch_stats(device_id: Option<&str>) -> Result<(String, Vec<(String, String)>, String), String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    if device.port.is_empty() {
        return Err(format!(
            "device '{}' has no serial port available; reconnect in serial mode and try again",
            device.device
        ));
    }

    let payload = send_serial_command_and_capture(&device.port, "stats")?;
    let raw = String::from_utf8_lossy(&payload).to_string();
    let entries = parse_stats_entries(&raw);
    Ok((device.device, entries, raw))
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

fn send_serial_command_and_capture(port_path: &str, command: &str) -> Result<Vec<u8>, String> {
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
                    ))
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

fn default_screenshot_filename() -> String {
    format!("playdate_{}.gif", timestamp_now())
}

#[cfg(unix)]
fn timestamp_now() -> String {
    let output = Command::new("date").arg("+%Y-%m-%d_%H%M%S").output();
    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => "1970-01-01_000000".to_string(),
    }
}

#[cfg(not(unix))]
fn timestamp_now() -> String {
    "1970-01-01_000000".to_string()
}

fn inspect_screen_payload(payload: &[u8], path: &str) -> String {
    if let Some(offset) = find_subslice(payload, SCREEN_PREFIX)
        .or_else(|| find_subslice(payload, SCREEN_PREFIX_LEGACY))
    {
        let image_start = offset + SCREEN_PREFIX.len();
        let remaining = payload.len().saturating_sub(image_start);
        if remaining >= SCREEN_BITMAP_BYTES {
            return format!(
                "detected screen header at byte {offset}; bitmap payload appears complete ({remaining} bytes after header), wrote {}",
                screenshot_kind_for_path(path)
            );
        }

        return format!(
            "detected screen header at byte {offset}; bitmap payload appears partial ({remaining}/{SCREEN_BITMAP_BYTES} bytes after header), wrote {}",
            screenshot_kind_for_path(path)
        );
    }

    let preview_len = payload.len().min(32);
    let preview = payload[..preview_len]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "no known screen header found; first {preview_len} bytes (hex): {preview}; wrote {}",
        screenshot_kind_for_path(path)
    )
}

fn screenshot_kind_for_path(path: &str) -> &'static str {
    match screenshot_format_for_path(path) {
        Ok(Some(ImageFormat::Png)) => "PNG image",
        Ok(Some(ImageFormat::Gif)) => "GIF image",
        _ => "raw serial bytes",
    }
}

fn parse_stats_entries(raw: &str) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    let normalized = raw.replace('\r', "\n");
    let mut in_stats = false;

    for line in normalized.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.contains("~stats:") {
            in_stats = true;
            continue;
        }
        if !in_stats {
            continue;
        }
        if line.starts_with('~') {
            break;
        }

        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim();
            let value = v.trim();
            if !key.is_empty() && !value.is_empty() {
                entries.push((key.to_string(), value.to_string()));
            }
        }
    }

    entries
}

fn print_stats_json(entries: &[(String, String)]) {
    println!("{{");
    for (idx, (k, v)) in entries.iter().enumerate() {
        let comma = if idx + 1 < entries.len() { "," } else { "" };
        if let Ok(i) = v.parse::<i64>() {
            println!("  \"{}\": {}{}", escape_json(k), i, comma);
            continue;
        }
        if let Ok(f) = v.parse::<f64>() {
            if f.is_finite() {
                println!("  \"{}\": {}{}", escape_json(k), f, comma);
                continue;
            }
        }
        println!("  \"{}\": \"{}\"{}", escape_json(k), escape_json(v), comma);
    }
    println!("}}");
}

fn escape_json(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            _ => vec![c],
        })
        .collect()
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

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }

    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn eject_device(device_id: Option<&str>) -> Result<String, String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    eject_target(&device.disk, &device.mount_path)?;
    Ok(device.device)
}

fn resolve_device(devices: Vec<Device>, device_id: Option<&str>) -> Result<Device, String> {
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
        if !is_playdate_usb_device(usb.vendor_id(), usb.product_id()) {
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
                saw_product = is_playdate_product_id(v);
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

fn is_playdate_usb_device(vendor_id: u16, product_id: u16) -> bool {
    vendor_id == PLAYDATE_VENDOR_ID && is_playdate_product_id(product_id)
}

fn is_playdate_product_id(product_id: u16) -> bool {
    product_id == PLAYDATE_PRODUCT_ID_MSC || product_id == PLAYDATE_PRODUCT_ID_APP
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
        build_disk_mount_index, extract_disk_from_device_path, extract_screen_bitmap,
        find_mount_path_for_serial, find_port_for_serial, is_playdate_product_id, normalize,
        parse_device_command, parse_macos_playdate_disks_by_serial, parse_mount_entries,
        parse_stats_entries, resolve_device, screenshot_format_for_path, Device, DeviceCommand,
        MountEntry, SCREEN_BITMAP_BYTES, SCREEN_PREFIX,
    };
    use image::ImageFormat;
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

    #[test]
    fn parses_device_list_command() {
        let args = vec!["list".to_string()];
        let cmd = parse_device_command(&args).expect("expected list command");
        assert_eq!(cmd, DeviceCommand::List);
    }

    #[test]
    fn parses_device_unmount_command_with_flag_first() {
        let args = vec![
            "-d".to_string(),
            "PDU1-Y013705".to_string(),
            "unmount".to_string(),
        ];

        let cmd = parse_device_command(&args).expect("expected eject command");
        assert_eq!(
            cmd,
            DeviceCommand::Eject {
                device_id: Some("PDU1-Y013705".to_string())
            }
        );
    }

    #[test]
    fn parses_device_eject_command_with_subcommand_first() {
        let args = vec![
            "eject".to_string(),
            "--device".to_string(),
            "PDU1-Y013705".to_string(),
        ];

        let cmd = parse_device_command(&args).expect("expected eject command");
        assert_eq!(
            cmd,
            DeviceCommand::Eject {
                device_id: Some("PDU1-Y013705".to_string())
            }
        );
    }

    #[test]
    fn recognizes_both_playdate_product_ids() {
        assert!(is_playdate_product_id(0x5740));
        assert!(is_playdate_product_id(0x5741));
        assert!(!is_playdate_product_id(0x5742));
    }

    #[test]
    fn parses_device_mount_alias_command() {
        let args = vec![
            "-d".to_string(),
            "PDU1-Y013705".to_string(),
            "mount".to_string(),
        ];

        let cmd = parse_device_command(&args).expect("expected serial datadisk command");
        assert_eq!(
            cmd,
            DeviceCommand::Serial {
                device_id: Some("PDU1-Y013705".to_string()),
                command: "datadisk".to_string()
            }
        );
    }

    #[test]
    fn parses_device_serial_command() {
        let args = vec![
            "-d".to_string(),
            "PDU1-Y013705".to_string(),
            "serial".to_string(),
            "help".to_string(),
        ];

        let cmd = parse_device_command(&args).expect("expected serial command");
        assert_eq!(
            cmd,
            DeviceCommand::Serial {
                device_id: Some("PDU1-Y013705".to_string()),
                command: "help".to_string()
            }
        );
    }

    #[test]
    fn parses_device_eject_without_device_flag() {
        let args = vec!["eject".to_string()];
        let cmd = parse_device_command(&args).expect("expected eject command");
        assert_eq!(cmd, DeviceCommand::Eject { device_id: None });
    }

    #[test]
    fn parses_device_mount_without_device_flag() {
        let args = vec!["mount".to_string()];
        let cmd = parse_device_command(&args).expect("expected mount/datadisk command");
        assert_eq!(
            cmd,
            DeviceCommand::Serial {
                device_id: None,
                command: "datadisk".to_string()
            }
        );
    }

    #[test]
    fn parses_device_serial_without_device_flag() {
        let args = vec!["serial".to_string(), "help".to_string()];
        let cmd = parse_device_command(&args).expect("expected serial command");
        assert_eq!(
            cmd,
            DeviceCommand::Serial {
                device_id: None,
                command: "help".to_string()
            }
        );
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

    #[test]
    fn parses_device_screenshot_without_flags() {
        let args = vec!["screenshot".to_string()];
        let cmd = parse_device_command(&args).expect("expected screenshot command");
        assert_eq!(
            cmd,
            DeviceCommand::Screenshot {
                device_id: None,
                filename: None,
                open: false
            }
        );
    }

    #[test]
    fn parses_device_screenshot_with_filename() {
        let args = vec![
            "screenshot".to_string(),
            "-f".to_string(),
            "capture.gif".to_string(),
        ];
        let cmd = parse_device_command(&args).expect("expected screenshot command");
        assert_eq!(
            cmd,
            DeviceCommand::Screenshot {
                device_id: None,
                filename: Some("capture.gif".to_string()),
                open: false
            }
        );
    }

    #[test]
    fn parses_device_screenshot_with_device_and_filename() {
        let args = vec![
            "-d".to_string(),
            "PDU1-Y013705".to_string(),
            "screenshot".to_string(),
            "-f".to_string(),
            "capture.gif".to_string(),
        ];
        let cmd = parse_device_command(&args).expect("expected screenshot command");
        assert_eq!(
            cmd,
            DeviceCommand::Screenshot {
                device_id: Some("PDU1-Y013705".to_string()),
                filename: Some("capture.gif".to_string()),
                open: false
            }
        );
    }

    #[test]
    fn parses_device_screenshot_with_open_flag() {
        let args = vec!["screenshot".to_string(), "--open".to_string()];
        let cmd = parse_device_command(&args).expect("expected screenshot command");
        assert_eq!(
            cmd,
            DeviceCommand::Screenshot {
                device_id: None,
                filename: None,
                open: true
            }
        );
    }

    #[test]
    fn parses_device_screenshot_with_open_and_filename_and_device() {
        let args = vec![
            "-d".to_string(),
            "PDU1-Y013705".to_string(),
            "screenshot".to_string(),
            "-f".to_string(),
            "capture.png".to_string(),
            "--open".to_string(),
        ];
        let cmd = parse_device_command(&args).expect("expected screenshot command");
        assert_eq!(
            cmd,
            DeviceCommand::Screenshot {
                device_id: Some("PDU1-Y013705".to_string()),
                filename: Some("capture.png".to_string()),
                open: true
            }
        );
    }

    #[test]
    fn has_expected_screen_prefix_signature() {
        assert_eq!(SCREEN_PREFIX, b"screen\r\n~screen:\n");
    }

    #[test]
    fn screenshot_format_detects_png_and_gif() {
        assert_eq!(
            screenshot_format_for_path("capture.png").expect("png format"),
            Some(ImageFormat::Png)
        );
        assert_eq!(
            screenshot_format_for_path("capture.gif").expect("gif format"),
            Some(ImageFormat::Gif)
        );
        assert!(screenshot_format_for_path("capture.raw")
            .expect("raw format")
            .is_none());
    }

    #[test]
    fn extracts_bitmap_after_screen_prefix() {
        let mut payload = Vec::new();
        payload.extend_from_slice(SCREEN_PREFIX);
        payload.extend_from_slice(&vec![0xAA; SCREEN_BITMAP_BYTES]);
        payload.extend_from_slice(b"\r\n");

        let bitmap = extract_screen_bitmap(&payload).expect("bitmap extraction");
        assert_eq!(bitmap.len(), SCREEN_BITMAP_BYTES);
        assert_eq!(bitmap[0], 0xAA);
    }

    #[test]
    fn parses_stats_command_with_json() {
        let args = vec!["stats".to_string(), "--json".to_string()];
        let cmd = parse_device_command(&args).expect("expected stats command");
        assert_eq!(
            cmd,
            DeviceCommand::Stats {
                device_id: None,
                json: true
            }
        );
    }

    #[test]
    fn parses_stats_command_with_device() {
        let args = vec![
            "-d".to_string(),
            "PDU1-Y013705".to_string(),
            "stats".to_string(),
        ];
        let cmd = parse_device_command(&args).expect("expected stats command");
        assert_eq!(
            cmd,
            DeviceCommand::Stats {
                device_id: Some("PDU1-Y013705".to_string()),
                json: false
            }
        );
    }

    #[test]
    fn parses_stats_payload_entries() {
        let raw = "stats\r\n~stats:\nframe count: 194503\nframe time: 0.000977\nkernel: 0.1%\n";
        let entries = parse_stats_entries(raw);
        assert_eq!(
            entries,
            vec![
                ("frame count".to_string(), "194503".to_string()),
                ("frame time".to_string(), "0.000977".to_string()),
                ("kernel".to_string(), "0.1%".to_string()),
            ]
        );
    }
}
