use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use super::SerialPortPath;

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

pub(crate) fn eject_target(_mount_path: Option<&Path>) -> Result<(), String> {
    Err("eject is not supported on this platform yet".to_string())
}

pub(crate) fn list_serial_ports() -> Vec<SerialPortPath> {
    Vec::new()
}

pub(crate) fn list_mounts() -> Result<Vec<(String, String)>, String> {
    Ok(Vec::new())
}

pub(crate) fn list_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    Ok(HashMap::new())
}

pub(crate) fn build_disk_mounts(
    _mounts: &[(String, String)],
) -> Result<HashMap<String, String>, String> {
    Err("build_disk_mounts is not supported on windows yet".to_string())
}

impl SerialPortPath {
    pub(crate) fn send_serial_command_and_capture(
        &self,
        _command: &str,
    ) -> Result<Vec<u8>, String> {
        self.open_serial_port()?;
        Err("send_serial_command_and_capture is not supported on windows yet".to_string())
    }

    pub(crate) fn send_serial_command(&self, _command: &str) -> Result<(), String> {
        self.open_serial_port()?;
        Err("send_serial_command is not supported on windows yet".to_string())
    }

    fn open_serial_port(&self) -> Result<(), String> {
        Err(format!(
            "opening serial port '{}' is not supported on windows yet",
            self
        ))
    }
}
