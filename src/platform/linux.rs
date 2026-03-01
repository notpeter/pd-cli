use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use super::SerialPortPath;
use super::unix::{list_mounts_via_mount_command, list_serial_ports_from_dev};

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

pub(crate) fn eject_target(mount_path: Option<&Path>) -> Result<(), String> {
    let Some(mount_path) = mount_path else {
        return Err("device is not mounted; cannot eject on this platform".to_string());
    };

    let output = Command::new("umount")
        .arg(mount_path.as_os_str())
        .output()
        .map_err(|e| format!("failed to run umount: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(format!(
        "failed to unmount '{}': {stderr}",
        mount_path.display()
    ))
}

pub(crate) fn list_serial_ports() -> Vec<SerialPortPath> {
    list_serial_ports_from_dev()
}

pub(crate) fn list_mounts() -> Result<Vec<(String, String)>, String> {
    list_mounts_via_mount_command()
}

pub(crate) fn list_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    Ok(HashMap::new())
}
