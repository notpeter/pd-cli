use crate::device::DeviceSerial;
use serde_json::Value;
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
    let script = r#"
$ErrorActionPreference = 'Stop'
Get-Partition |
  Where-Object { $_.DriveLetter } |
  Select-Object DiskNumber, DriveLetter |
  ConvertTo-Json -Compress
"#;

    let json = match run_powershell(script) {
        Ok(out) => out,
        Err(_) => return Ok(Vec::new()),
    };

    let Some(rows) = parse_json_rows(&json) else {
        return Ok(Vec::new());
    };

    Ok(rows
        .iter()
        .filter_map(|row| {
            let disk_number = row.get("DiskNumber")?.as_u64()?;
            let drive_letter = row.get("DriveLetter")?.as_str()?;
            let source = format!("disk{disk_number}");
            let target = format!("{drive_letter}:\\");
            Some((source, target))
        })
        .collect())
}

pub(crate) fn list_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    let script = r#"
$ErrorActionPreference = 'Stop'
Get-CimInstance Win32_DiskDrive |
  Where-Object { $_.PNPDeviceID -match 'VID_1331&PID_574[01]' } |
  Select-Object PNPDeviceID,DeviceID,Index |
  ConvertTo-Json -Compress
"#;

    let json = match run_powershell(script) {
        Ok(out) => out,
        Err(_) => return Ok(HashMap::new()),
    };

    let Some(rows) = parse_json_rows(&json) else {
        return Ok(HashMap::new());
    };

    let mut by_serial: HashMap<String, Vec<String>> = HashMap::new();
    for row in &rows {
        let Some(pnp_id) = row.get("PNPDeviceID").and_then(Value::as_str) else {
            continue;
        };
        let Some(serial) = parse_serial_from_pnp_device_id(pnp_id) else {
            continue;
        };
        let Some(disk) = parse_disk_key(row) else {
            continue;
        };

        by_serial
            .entry(serial.core().to_string())
            .or_default()
            .push(disk);
    }

    for disks in by_serial.values_mut() {
        disks.sort();
        disks.dedup();
    }

    Ok(by_serial)
}

pub(crate) fn build_disk_mounts(
    mounts: &[(String, String)],
) -> Result<HashMap<String, String>, String> {
    let mut by_disk = HashMap::new();

    for (source, target) in mounts {
        if !source.starts_with("disk") {
            continue;
        }
        by_disk
            .entry(source.clone())
            .or_insert_with(|| target.clone());
    }

    Ok(by_disk)
}

fn run_powershell(script: &str) -> Result<String, String> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|e| format!("failed to run powershell: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("powershell failed: {stderr}"));
    }

    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_string())
        .map_err(|e| format!("powershell returned non-UTF8 output: {e}"))
}

fn parse_json_rows(json: &str) -> Option<Vec<Value>> {
    if json.trim().is_empty() {
        return Some(Vec::new());
    }

    let parsed: Value = serde_json::from_str(json).ok()?;
    match parsed {
        Value::Array(values) => Some(values),
        Value::Object(_) => Some(vec![parsed]),
        _ => None,
    }
}

fn parse_serial_from_pnp_device_id(pnp_device_id: &str) -> Option<DeviceSerial> {
    let raw = pnp_device_id.rsplit('\\').next()?;
    let before_ampersand = raw.split('&').next().unwrap_or(raw);
    DeviceSerial::parse(before_ampersand)
}

fn parse_disk_key(row: &Value) -> Option<String> {
    if let Some(index) = row.get("Index").and_then(Value::as_u64) {
        return Some(format!("disk{index}"));
    }

    let device_id = row.get("DeviceID").and_then(Value::as_str)?;
    let disk_number = device_id
        .rsplit("PHYSICALDRIVE")
        .next()?
        .parse::<u64>()
        .ok()?;
    Some(format!("disk{disk_number}"))
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

#[cfg(test)]
mod tests {
    use super::{parse_disk_key, parse_serial_from_pnp_device_id};
    use serde_json::json;

    #[test]
    fn parses_playdate_serial_from_windows_pnp_device_id() {
        let serial = parse_serial_from_pnp_device_id(
            r"USBSTOR\DISK&VEN_PANIC&PROD_PLAYDATE&REV_1.00\PDU1_Y012345&0",
        )
        .expect("serial should parse");
        assert_eq!(serial.to_string(), "PDU1-Y012345");
    }

    #[test]
    fn parses_disk_key_from_index() {
        let row = json!({
            "Index": 7,
            "DeviceID": r"\\.\PHYSICALDRIVE3"
        });
        assert_eq!(parse_disk_key(&row), Some("disk7".to_string()));
    }

    #[test]
    fn parses_disk_key_from_device_id_when_index_missing() {
        let row = json!({
            "DeviceID": r"\\.\PHYSICALDRIVE11"
        });
        assert_eq!(parse_disk_key(&row), Some("disk11".to_string()));
    }
}
