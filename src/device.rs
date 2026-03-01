use crate::platform::{
    build_disk_mounts, eject_target, list_mounts, list_playdate_disks_by_serial, list_serial_ports,
    send_serial_command,
};
use crate::{PLAYDATE_PRODUCT_ID_APP, PLAYDATE_PRODUCT_ID_MSC, PLAYDATE_VENDOR_ID, normalize};
use nusb::MaybeFuture;
use std::time::{Duration, Instant};

const MOUNT_WAIT_TIMEOUT: Duration = Duration::from_secs(25);
const MOUNT_WAIT_POLL: Duration = Duration::from_millis(250);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Device {
    pub(crate) device: String,
    pub(crate) port: String,
    pub(crate) mounted: bool,
    pub(crate) mount_path: String,
    pub(crate) disk: String,
}

pub(crate) fn resolve_device(
    devices: Vec<Device>,
    device_id: Option<&str>,
) -> Result<Device, String> {
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

pub(crate) fn list_devices() -> Result<Vec<Device>, String> {
    let serial_ports = list_serial_ports();
    let mounts = list_mounts()
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>();
    let disk_mounts = build_disk_mounts(&mounts);
    let serial_to_disks = list_playdate_disks_by_serial().unwrap_or_default();

    let mut devices = Vec::new();

    let usb_devices = nusb::list_devices()
        .wait()
        .map_err(|e| format!("failed to list USB devices: {e}"))?;

    for usb in usb_devices {
        if usb.vendor_id() != PLAYDATE_VENDOR_ID
            || (usb.product_id() != PLAYDATE_PRODUCT_ID_MSC
                && usb.product_id() != PLAYDATE_PRODUCT_ID_APP)
        {
            continue;
        }

        let serial = usb.serial_number().unwrap_or("unknown").to_string();
        let disk = serial_to_disks
            .get(&normalize(&serial))
            .and_then(|v| v.first().cloned())
            .unwrap_or_default();
        let normalized_serial = normalize(&serial);
        let port = serial_ports
            .iter()
            .find(|port| normalize(port).contains(&normalized_serial))
            .cloned()
            .unwrap_or_default();
        let mount_path =
            find_mount_path_for_serial(&serial, &serial_to_disks, &disk_mounts).unwrap_or_default();

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

pub(crate) fn print_devices(devices: &[Device]) {
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

pub(crate) fn send_serial_command_to_device(
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

pub(crate) fn mount_device(device_id: Option<&str>) -> Result<(String, String), String> {
    let start = Instant::now();

    loop {
        let devices = list_devices().unwrap_or_default();
        match select_mount_target(&devices, device_id) {
            Ok(Some(device)) => {
                if device.port.is_empty() {
                    let mount_path = wait_for_mount_ready(&device.device)?;
                    return Ok((device.device, mount_path));
                }

                send_serial_command(&device.port, "datadisk")?;
                let mount_path = wait_for_mount_ready(&device.device)?;
                return Ok((device.device, mount_path));
            }
            Ok(None) => {}
            Err(e) => return Err(e),
        }

        if Instant::now().duration_since(start) >= MOUNT_WAIT_TIMEOUT {
            return Err(
                "timed out waiting for device before mount: no Playdate devices found".to_string(),
            );
        }

        std::thread::sleep(MOUNT_WAIT_POLL);
    }
}

pub(crate) fn select_mount_target(
    devices: &[Device],
    device_id: Option<&str>,
) -> Result<Option<Device>, String> {
    match device_id {
        Some(id) => {
            let needle = normalize(id);
            Ok(devices
                .iter()
                .find(|d| normalize(&d.device) == needle)
                .cloned())
        }
        None => match devices.len() {
            0 => Ok(None),
            1 => Ok(Some(devices[0].clone())),
            _ => Err("multiple Playdate devices found; specify one with `-d <serial>`".to_string()),
        },
    }
}

pub(crate) fn wait_for_mount_ready(serial: &str) -> Result<String, String> {
    let start = Instant::now();
    let mut last_seen_mount: Option<String> = None;
    let mut last_error: Option<String> = None;

    loop {
        let mounts = list_mounts().unwrap_or_default();
        if let Some(path) = find_mount_path_for_serial_live(serial, &mounts) {
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

pub(crate) fn eject_device(device_id: Option<&str>) -> Result<String, String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    eject_target(&device.disk, &device.mount_path)?;
    Ok(device.device)
}

fn find_mount_path_for_serial_live(serial: &str, mounts: &[(String, String)]) -> Option<String> {
    let disk_mounts = build_disk_mounts(mounts);
    let serial_to_disks = list_playdate_disks_by_serial().unwrap_or_default();
    find_mount_path_for_serial(serial, &serial_to_disks, &disk_mounts)
}

pub(crate) fn find_mount_path_for_serial(
    serial: &str,
    serial_to_disks: &std::collections::HashMap<String, Vec<String>>,
    disk_mounts: &std::collections::HashMap<String, String>,
) -> Option<String> {
    let serial_key = normalize(serial);
    let disks = serial_to_disks.get(&serial_key)?;

    disks.iter().find_map(|disk| disk_mounts.get(disk).cloned())
}
