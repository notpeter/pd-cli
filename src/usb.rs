use crate::device::{Device, DeviceList, DeviceSerial};
use crate::platform::{
    SerialPortPath, build_disk_mounts, eject_target, list_mounts, list_playdate_disks_by_serial,
    list_serial_ports,
};
use crate::{PLAYDATE_PRODUCT_ID_APP, PLAYDATE_PRODUCT_ID_MSC, PLAYDATE_VENDOR_ID};
use nusb::MaybeFuture;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const MOUNT_WAIT_TIMEOUT: Duration = Duration::from_secs(25);
const MOUNT_WAIT_POLL: Duration = Duration::from_millis(250);

pub(crate) fn resolve_device(
    devices: DeviceList,
    device_id: Option<&str>,
) -> Result<Device, String> {
    match device_id {
        Some(id) => {
            let needle = DeviceSerial::parse(id).ok_or_else(|| {
                format!("invalid device serial '{id}'; expected forms like PDU1-Y013705 or Y013705")
            })?;

            devices
                .0
                .into_iter()
                .find(|device| device.serial() == &needle)
                .ok_or_else(|| {
                    format!(
                        "device '{id}' not found; run `pd device list` to see available devices"
                    )
                })
        }
        None => {
            let mut devices = devices.0;
            match devices.len() {
                0 => Err("no Playdate devices found".to_string()),
                1 => Ok(devices.remove(0)),
                _ => Err(
                    "multiple Playdate devices found; specify one with `-d <serial>`".to_string(),
                ),
            }
        }
    }
}

pub(crate) fn list_devices() -> Result<DeviceList, String> {
    let serial_ports = list_serial_ports();
    let mounts = list_mounts().map_err(|e| format!("failed to list mounts: {e}"))?;
    let disk_mounts = build_disk_mounts(&mounts)?;
    let serial_to_disks = list_playdate_disks_by_serial()
        .map_err(|e| format!("failed to resolve Playdate disks by serial: {e}"))?;

    let usb_devices = nusb::list_devices()
        .wait()
        .map_err(|e| format!("failed to list USB devices: {e}"))?;

    let mut devices = Vec::new();
    for usb in usb_devices {
        if usb.vendor_id() != PLAYDATE_VENDOR_ID
            || (usb.product_id() != PLAYDATE_PRODUCT_ID_MSC
                && usb.product_id() != PLAYDATE_PRODUCT_ID_APP)
        {
            continue;
        }

        let Some(serial) = usb.serial_number().and_then(DeviceSerial::parse) else {
            continue;
        };

        let port = find_port_for_serial(&serial_ports, &serial);
        let mount_path =
            find_mount_path_for_serial(&serial, &serial_to_disks, &disk_mounts).map(Into::into);

        devices.push(Device::new(serial, port, mount_path));
    }

    devices.sort_by(|a, b| a.serial().cmp(b.serial()));
    devices.dedup_by(|a, b| a.serial() == b.serial());
    Ok(DeviceList(devices))
}

pub(crate) fn send_serial_command_to_device(
    device_id: Option<&str>,
    command: &str,
) -> Result<(String, String), String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    let Some(port) = device.port() else {
        return Err(format!(
            "device '{}' has no serial port available; reconnect in serial mode and try again",
            device.serial()
        ));
    };

    port.send_serial_command(command)?;
    Ok((device.serial().to_string(), port.to_string()))
}

pub(crate) fn mount_device(device_id: Option<&str>) -> Result<(String, String), String> {
    let start = Instant::now();

    loop {
        let devices = list_devices()?;
        match devices.select_mount_target(device_id)? {
            Some(device) => {
                let serial = device.serial().to_string();
                if let Some(port) = device.port() {
                    port.send_serial_command("datadisk")?;
                }

                let mount_path = device.wait_for_mount_ready()?;
                return Ok((serial, mount_path));
            }
            None => {
                if Instant::now().duration_since(start) >= MOUNT_WAIT_TIMEOUT {
                    return Err(
                        "timed out waiting for device before mount: no Playdate devices found"
                            .to_string(),
                    );
                }
            }
        }

        std::thread::sleep(MOUNT_WAIT_POLL);
    }
}

pub(crate) fn eject_device(device_id: Option<&str>) -> Result<String, String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    device.eject_device()?;

    Ok(device.serial().to_string())
}

impl Device {
    pub(crate) fn eject_device(&self) -> Result<(), String> {
        eject_target(self.mount_path())
    }

    pub(crate) fn wait_for_mount_ready(&self) -> Result<String, String> {
        let start = Instant::now();
        let mut last_seen_mount: Option<String> = None;
        let mut last_error: Option<String> = None;

        loop {
            let mounts = list_mounts()?;
            if let Some(path) = find_mount_path_for_serial_live(self.serial(), &mounts)? {
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
}

impl DeviceList {
    fn select_mount_target(&self, device_id: Option<&str>) -> Result<Option<Device>, String> {
        match device_id {
            Some(id) => {
                let needle = DeviceSerial::parse(id).ok_or_else(|| {
                    format!(
                        "invalid device serial '{id}'; expected forms like PDU1-Y013705 or Y013705"
                    )
                })?;
                Ok(self
                    .as_slice()
                    .iter()
                    .find(|device| device.serial() == &needle)
                    .cloned())
            }
            None => match self.as_slice().len() {
                0 => Ok(None),
                1 => Ok(Some(self.as_slice()[0].clone())),
                _ => Err(
                    "multiple Playdate devices found; specify one with `-d <serial>`".to_string(),
                ),
            },
        }
    }
}

fn find_port_for_serial(
    serial_ports: &[SerialPortPath],
    serial: &DeviceSerial,
) -> Option<SerialPortPath> {
    serial_ports
        .iter()
        .find(|port| port.contains_device_serial(serial))
        .cloned()
}

impl SerialPortPath {
    fn contains_device_serial(&self, serial: &DeviceSerial) -> bool {
        let Some(name) = self.as_path().file_name().and_then(|name| name.to_str()) else {
            return false;
        };

        if let Some(parsed) = DeviceSerial::parse(name) {
            if &parsed == serial {
                return true;
            }
        }

        name.split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|part| !part.is_empty())
            .filter_map(DeviceSerial::parse)
            .any(|parsed| &parsed == serial)
    }
}

fn find_mount_path_for_serial_live(
    serial: &DeviceSerial,
    mounts: &[(String, String)],
) -> Result<Option<String>, String> {
    let disk_mounts = build_disk_mounts(mounts)?;
    let serial_to_disks = list_playdate_disks_by_serial()
        .map_err(|e| format!("failed to resolve Playdate disks by serial: {e}"))?;
    Ok(find_mount_path_for_serial(
        serial,
        &serial_to_disks,
        &disk_mounts,
    ))
}

fn find_mount_path_for_serial(
    serial: &DeviceSerial,
    serial_to_disks: &HashMap<String, Vec<String>>,
    disk_mounts: &HashMap<String, String>,
) -> Option<String> {
    let disks = serial_to_disks.get(serial.core())?;
    disks.iter().find_map(|disk| disk_mounts.get(disk).cloned())
}
