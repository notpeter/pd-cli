use crate::command::DeviceSelector;
use crate::device::{Device, DeviceList, DeviceSerial};
use crate::platform::{
    SerialPortPath, build_disk_mounts, eject_target, list_mounts, list_playdate_disks_by_serial,
    list_serial_ports,
};
use crate::{PLAYDATE_PRODUCT_ID_APP, PLAYDATE_PRODUCT_ID_MSC, PLAYDATE_VENDOR_ID};
use nusb::MaybeFuture;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const MOUNT_WAIT_TIMEOUT: Duration = Duration::from_secs(25);
const MOUNT_WAIT_POLL: Duration = Duration::from_millis(250);

pub(crate) fn resolve_device(
    devices: DeviceList,
    device: &DeviceSelector,
) -> Result<Device, String> {
    match device {
        DeviceSelector::BySerial(needle) => devices
            .0
            .into_iter()
            .find(|candidate| candidate.serial() == needle)
            .ok_or_else(|| {
                format!(
                    "device '{needle}' not found; run `pd device list` to see available devices"
                )
            }),
        DeviceSelector::Auto => {
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

pub(crate) fn get_device(device: &DeviceSelector) -> Result<Device, String> {
    let devices = DeviceList::discover()?;
    resolve_device(devices, device)
}

pub(crate) fn resolve_mount_target(device: &DeviceSelector) -> Result<Device, String> {
    let start = Instant::now();

    loop {
        let devices = DeviceList::discover()?;
        let device = match device {
            DeviceSelector::BySerial(needle) => devices
                .as_slice()
                .iter()
                .find(|candidate| candidate.serial() == needle)
                .cloned(),
            DeviceSelector::Auto => match devices.as_slice().len() {
                0 => None,
                1 => Some(devices.as_slice()[0].clone()),
                _ => return Err("multiple devices found; specify with `-d <serial>`".to_string()),
            },
        };

        match device {
            Some(device) => return Ok(device),
            None => {
                if Instant::now().duration_since(start) >= MOUNT_WAIT_TIMEOUT {
                    return Err("timeout waiting for device mount: no devices found".to_string());
                }
            }
        }

        std::thread::sleep(MOUNT_WAIT_POLL);
    }
}

impl Device {
    pub(crate) fn send_command(&self, command: &str) -> Result<(), String> {
        let Some(port) = self.port() else {
            return Err(format!(
                "{self} has no serial port available; is the disk mounted?"
            ));
        };
        port.send_serial_command(command)?;
        Ok(())
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

    pub(crate) fn mount_device(&mut self) -> Result<(), String> {
        if let Some(port) = self.port() {
            port.send_serial_command("datadisk")?;
        }

        let mount_path = self.wait_for_mount_ready()?;
        self.set_mount_path(PathBuf::from(mount_path));
        Ok(())
    }

    pub(crate) fn eject_device(&mut self) -> Result<(), String> {
        eject_target(self.mount_path())?;
        self.clear_mount_path();
        Ok(())
    }
}

impl DeviceList {
    pub(crate) fn discover() -> Result<Self, String> {
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

        if name.starts_with("cu.usbmodemPDU1") || name.starts_with("tty.usbmodemPDU1") {
            let name_upper = name.to_ascii_uppercase();
            return name_upper.contains(serial.core());
        }
        false
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

#[cfg(test)]
mod tests {
    use super::find_port_for_serial;
    use crate::device::DeviceSerial;
    use crate::platform::SerialPortPath;
    use std::path::PathBuf;

    #[test]
    fn matches_port_with_interface_suffix_digit() {
        let serial = DeviceSerial::parse("Y012345").expect("valid serial");
        // Note the extra trailing '1'
        let path = PathBuf::from("/dev/cu.usbmodemPDU1_Y123451");
        let ports = vec![SerialPortPath::new(path)];
        let port = find_port_for_serial(&ports, &serial).expect("port should resolve");
        assert_eq!(port.to_string(), "/dev/cu.usbmodemPDU1_Y0137051");
    }
}
