use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub(crate) fn open_with_default_viewer(path: &str) -> Result<(), String> {
    let status = Command::new("xdg-open")
        .arg(path)
        .status()
        .map_err(|e| format!("failed to run xdg-open: {e}"))?;
    if status.success() {
        return Ok(());
    }
    Err(format!("xdg-open failed for '{path}'"))
}

pub(crate) fn eject_target(_disk: &str, mount_path: &str) -> Result<(), String> {
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

pub(crate) fn list_serial_ports() -> Vec<String> {
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

pub(crate) fn list_mounts() -> Result<Vec<(String, String)>, String> {
    let output = Command::new("mount")
        .output()
        .map_err(|e| format!("failed to run mount: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("mount failed: {stderr}"));
    }

    let text = String::from_utf8(output.stdout)
        .map_err(|e| format!("mount returned non-UTF8 output: {e}"))?;

    Ok(text
        .lines()
        .filter_map(|line| {
            let (source, rest) = line.split_once(" on ")?;
            let (target, _) = rest.split_once(" (")?;
            Some((source.trim().to_string(), target.trim().to_string()))
        })
        .collect())
}

pub(crate) fn list_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    Ok(HashMap::new())
}
