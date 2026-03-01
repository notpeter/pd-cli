use std::collections::HashMap;
use std::process::Command;

pub(crate) fn open_with_default_viewer(path: &str) -> Result<(), String> {
    let status = Command::new("cmd")
        .args(["/C", "start", "", path])
        .status()
        .map_err(|e| format!("failed to run start: {e}"))?;
    if status.success() {
        return Ok(());
    }
    Err(format!("start failed for '{path}'"))
}

pub(crate) fn eject_target(_disk: &str, _mount_path: &str) -> Result<(), String> {
    Err("eject is not supported on this platform yet".to_string())
}

pub(crate) fn list_serial_ports() -> Vec<String> {
    Vec::new()
}

pub(crate) fn list_mounts() -> Result<Vec<(String, String)>, String> {
    Ok(Vec::new())
}

pub(crate) fn list_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    Ok(HashMap::new())
}

pub(crate) fn build_disk_mounts(_mounts: &[(String, String)]) -> HashMap<String, String> {
    unimplemented!("build_disk_mounts is not implemented on windows")
}

pub(crate) fn send_serial_command_and_capture(
    _port_path: &str,
    _command: &str,
) -> Result<Vec<u8>, String> {
    unimplemented!("send_serial_command_and_capture is not implemented on windows")
}

pub(crate) fn send_serial_command(_port_path: &str, _command: &str) -> Result<(), String> {
    unimplemented!("send_serial_command is not implemented on windows")
}
