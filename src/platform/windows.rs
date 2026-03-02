use crate::device::DeviceSerial;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use wmi::{COMLibrary, WMIConnection};

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
    let wmi = match init_wmi() {
        Ok(wmi) => wmi,
        Err(_) => return Ok(Vec::new()),
    };

    let disks: Vec<LogicalDiskRow> =
        match wmi.raw_query("SELECT DeviceID FROM Win32_LogicalDisk WHERE DeviceID IS NOT NULL") {
            Ok(rows) => rows,
            Err(_) => return Ok(Vec::new()),
        };

    let mut mounts = Vec::new();
    for disk in disks {
        let Some(source) = normalize_drive_id(&disk.device_id) else {
            continue;
        };
        mounts.push((source.clone(), format!("{source}\\")));
    }

    mounts.sort();
    mounts.dedup();
    Ok(mounts)
}

pub(crate) fn list_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    let wmi = match init_wmi() {
        Ok(wmi) => wmi,
        Err(_) => return Ok(HashMap::new()),
    };

    let query = "SELECT PNPDeviceID, DeviceID FROM Win32_DiskDrive \
                 WHERE PNPDeviceID LIKE '%VID_1331&PID_5740%' \
                    OR PNPDeviceID LIKE '%VID_1331&PID_5741%'";

    let drives: Vec<DiskDriveRow> = match wmi.raw_query(query) {
        Ok(rows) => rows,
        Err(_) => return Ok(HashMap::new()),
    };

    let mut by_serial: HashMap<String, Vec<String>> = HashMap::new();
    for drive in drives {
        let Some(pnp_id) = drive.pnp_device_id.as_deref() else {
            continue;
        };
        let Some(serial) = parse_serial_from_pnp_device_id(pnp_id) else {
            continue;
        };

        let partitions = match query_partitions_for_disk(&wmi, &drive.device_id) {
            Ok(rows) => rows,
            Err(_) => continue,
        };

        for partition in partitions {
            let logical_disks = match query_logical_disks_for_partition(&wmi, &partition.device_id)
            {
                Ok(rows) => rows,
                Err(_) => continue,
            };
            for logical_disk in logical_disks {
                let Some(drive_id) = normalize_drive_id(&logical_disk.device_id) else {
                    continue;
                };
                by_serial
                    .entry(serial.core().to_string())
                    .or_default()
                    .push(drive_id);
            }
        }
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
        if normalize_drive_id(source).is_none() {
            continue;
        }
        by_disk
            .entry(source.clone())
            .or_insert_with(|| target.clone());
    }

    Ok(by_disk)
}

fn init_wmi() -> Result<WMIConnection, String> {
    let com = COMLibrary::new().map_err(|e| format!("failed to initialize COM: {e}"))?;
    WMIConnection::new(com.into()).map_err(|e| format!("failed to connect to WMI: {e}"))
}

fn query_partitions_for_disk(
    wmi: &WMIConnection,
    disk_device_id: &str,
) -> Result<Vec<DiskPartitionRow>, String> {
    let query = format!(
        "ASSOCIATORS OF {{Win32_DiskDrive.DeviceID='{}'}} WHERE AssocClass = Win32_DiskDriveToDiskPartition",
        wql_escape(disk_device_id)
    );
    wmi.raw_query(&query)
        .map_err(|e| format!("failed to query disk partitions: {e}"))
}

fn query_logical_disks_for_partition(
    wmi: &WMIConnection,
    partition_device_id: &str,
) -> Result<Vec<LogicalDiskRow>, String> {
    let query = format!(
        "ASSOCIATORS OF {{Win32_DiskPartition.DeviceID='{}'}} WHERE AssocClass = Win32_LogicalDiskToPartition",
        wql_escape(partition_device_id)
    );
    wmi.raw_query(&query)
        .map_err(|e| format!("failed to query logical disks: {e}"))
}

fn parse_serial_from_pnp_device_id(pnp_device_id: &str) -> Option<DeviceSerial> {
    let raw = pnp_device_id.rsplit('\\').next()?;
    let before_ampersand = raw.split('&').next().unwrap_or(raw);
    DeviceSerial::parse(before_ampersand)
}

fn normalize_drive_id(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.len() < 2 {
        return None;
    }
    let bytes = trimmed.as_bytes();
    if !bytes[0].is_ascii_alphabetic() || bytes[1] != b':' {
        return None;
    }
    Some(trimmed[..2].to_ascii_uppercase())
}

fn wql_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\'', "''")
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DiskDriveRow {
    #[serde(default)]
    pnp_device_id: Option<String>,
    device_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DiskPartitionRow {
    device_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LogicalDiskRow {
    device_id: String,
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
    use super::{normalize_drive_id, parse_serial_from_pnp_device_id, wql_escape};

    #[test]
    fn parses_playdate_serial_from_windows_pnp_device_id() {
        let serial = parse_serial_from_pnp_device_id(
            r"USBSTOR\DISK&VEN_PANIC&PROD_PLAYDATE&REV_1.00\PDU1_Y012345&0",
        )
        .expect("serial should parse");
        assert_eq!(serial.to_string(), "PDU1-Y012345");
    }

    #[test]
    fn normalizes_drive_letter_to_uppercase() {
        assert_eq!(normalize_drive_id("e:"), Some("E:".to_string()));
        assert_eq!(normalize_drive_id("C:\\"), Some("C:".to_string()));
    }

    #[test]
    fn escapes_backslashes_and_quotes_for_wql() {
        assert_eq!(
            wql_escape(r"\\.\PHYSICALDRIVE3's"),
            r"\\\\.\\PHYSICALDRIVE3''s"
        );
    }
}
