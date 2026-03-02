use crate::device::DeviceSerial;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use windows_sys::Win32::Devices::Communication::{
    COMMTIMEOUTS, DCB, GetCommState, NOPARITY, ONESTOPBIT, PURGE_RXCLEAR, PURGE_TXCLEAR, PurgeComm,
    SetCommState, SetCommTimeouts,
};
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING, ReadFile, WriteFile,
};
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
    let wmi = match init_wmi() {
        Ok(wmi) => wmi,
        Err(_) => return Vec::new(),
    };

    let query = "SELECT DeviceID, PNPDeviceID FROM Win32_SerialPort \
                 WHERE PNPDeviceID LIKE '%VID_1331&PID_5740%' \
                    OR PNPDeviceID LIKE '%VID_1331&PID_5741%'";

    let rows: Vec<SerialPortRow> = match wmi.raw_query(query) {
        Ok(rows) => rows,
        Err(_) => return Vec::new(),
    };

    let mut ports = Vec::new();
    for row in rows {
        let Some(pnp_device_id) = row.pnp_device_id.as_deref() else {
            continue;
        };
        let Some(serial) = parse_serial_from_pnp_device_id(pnp_device_id) else {
            continue;
        };

        let device_id = row.device_id.trim();
        if device_id.is_empty() {
            continue;
        }

        let device_path = format!(r"\\.\{device_id}");
        ports.push(SerialPortPath::new(
            device_path.into(),
            serial.core().to_string(),
        ));
    }

    ports.sort();
    ports.dedup();
    ports
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SerialPortRow {
    device_id: String,
    #[serde(default)]
    pnp_device_id: Option<String>,
}

impl SerialPortPath {
    pub(crate) fn send_serial_command_and_capture(&self, command: &str) -> Result<Vec<u8>, String> {
        const SCREEN_CAPTURE_TIMEOUT: Duration = Duration::from_secs(2);
        const SCREEN_CAPTURE_IDLE: Duration = Duration::from_millis(300);

        let mut port = self.open_serial_port()?;
        let payload = format!("{command}\n");
        port.write_all(payload.as_bytes())
            .map_err(|e| format!("failed to write command to '{}': {e}", self))?;

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
                    return Err(format!("failed to read serial data from '{}': {e}", self));
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
                "no serial response received from '{}'; verify the device is unlocked and in app mode",
                self
            ));
        }

        Ok(out)
    }

    pub(crate) fn send_serial_command(&self, command: &str) -> Result<(), String> {
        let mut port = self.open_serial_port()?;
        let payload = format!("{command}\n");
        port.write_all(payload.as_bytes())
            .map_err(|e| format!("failed to write command to '{}': {e}", self))?;
        Ok(())
    }

    fn open_serial_port(&self) -> Result<WindowsSerialPort, String> {
        WindowsSerialPort::open(self)
    }
}

struct WindowsSerialPort {
    handle: HANDLE,
}

impl WindowsSerialPort {
    fn open(path: &SerialPortPath) -> Result<Self, String> {
        let wide_name: Vec<u16> = path
            .as_path()
            .to_string_lossy()
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let handle = unsafe {
            CreateFileW(
                wide_name.as_ptr(),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                std::ptr::null_mut(),
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(format!("failed to open serial port '{}'", path));
        }

        let mut port = Self { handle };
        port.configure_8n1_115200(path)?;
        Ok(port)
    }

    fn configure_8n1_115200(&mut self, path: &SerialPortPath) -> Result<(), String> {
        let mut dcb: DCB = unsafe { std::mem::zeroed() };
        dcb.DCBlength = std::mem::size_of::<DCB>() as u32;
        let ok = unsafe { GetCommState(self.handle, &mut dcb) };
        if ok == 0 {
            return Err(format!("failed to read serial settings for '{}'", path));
        }

        dcb.BaudRate = 115_200;
        dcb.ByteSize = 8;
        dcb.Parity = NOPARITY;
        dcb.StopBits = ONESTOPBIT;
        // DCB bitfield: keep only fBinary=1 and disable flow control/parity helpers.
        dcb._bitfield = 1;

        let ok = unsafe { SetCommState(self.handle, &dcb) };
        if ok == 0 {
            return Err(format!(
                "failed to configure serial settings for '{}'",
                path
            ));
        }

        let timeouts = COMMTIMEOUTS {
            ReadIntervalTimeout: 10,
            ReadTotalTimeoutMultiplier: 0,
            ReadTotalTimeoutConstant: 100,
            WriteTotalTimeoutMultiplier: 0,
            WriteTotalTimeoutConstant: 1000,
        };
        let ok = unsafe { SetCommTimeouts(self.handle, &timeouts) };
        if ok == 0 {
            return Err(format!("failed to set serial timeouts for '{}'", path));
        }

        unsafe {
            PurgeComm(self.handle, PURGE_RXCLEAR | PURGE_TXCLEAR);
        }
        Ok(())
    }

    fn write_all(&mut self, payload: &[u8]) -> Result<(), String> {
        let mut written_total = 0usize;
        while written_total < payload.len() {
            let mut wrote = 0u32;
            let ok = unsafe {
                WriteFile(
                    self.handle,
                    payload[written_total..].as_ptr(),
                    (payload.len() - written_total) as u32,
                    &mut wrote,
                    std::ptr::null_mut(),
                )
            };
            if ok == 0 {
                return Err("WriteFile failed".to_string());
            }
            if wrote == 0 {
                return Err("WriteFile wrote zero bytes".to_string());
            }
            written_total += wrote as usize;
        }
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let mut read = 0u32;
        let ok = unsafe {
            ReadFile(
                self.handle,
                buf.as_mut_ptr(),
                buf.len() as u32,
                &mut read,
                std::ptr::null_mut(),
            )
        };
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(read as usize)
    }
}

impl Drop for WindowsSerialPort {
    fn drop(&mut self) {
        if self.handle != INVALID_HANDLE_VALUE {
            unsafe {
                CloseHandle(self.handle);
            }
        }
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

    #[test]
    fn normalizes_com_port_path_at_discovery() {
        use crate::platform::SerialPortPath;
        let port = SerialPortPath::new(format!(r"\\.\COM3").into(), "Y012345".to_string());
        assert_eq!(port.to_string(), r"\\.\COM3");
    }
}
