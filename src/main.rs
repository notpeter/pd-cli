use nusb::MaybeFuture;
use std::env;

const PLAYDATE_VENDOR_ID: u16 = 0x1331;
const PLAYDATE_PRODUCT_ID: u16 = 0x5741;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Device {
    device: String,
    port: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("device") => match args.get(2).map(String::as_str) {
            Some("list") => {
                let devices = list_devices()?;
                print_devices(&devices);
                Ok(())
            }
            _ => Err("usage: pd device list".to_string()),
        },
        _ => Err("usage: pd device list".to_string()),
    }
}

fn list_devices() -> Result<Vec<Device>, String> {
    let serial_ports = list_serial_ports();
    let mut devices = Vec::new();

    let usb_devices = nusb::list_devices()
        .wait()
        .map_err(|e| format!("failed to list USB devices: {e}"))?;

    for usb in usb_devices {
        if usb.vendor_id() != PLAYDATE_VENDOR_ID || usb.product_id() != PLAYDATE_PRODUCT_ID {
            continue;
        }

        let serial = usb.serial_number().unwrap_or("unknown").to_string();
        let port = find_port_for_serial(&serial, &serial_ports).unwrap_or_default();

        devices.push(Device {
            device: serial,
            port,
        });
    }

    devices.sort_by(|a, b| a.device.cmp(&b.device));
    devices.dedup();
    Ok(devices)
}

#[cfg(unix)]
fn list_serial_ports() -> Vec<String> {
    use std::fs;

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

#[cfg(not(unix))]
fn list_serial_ports() -> Vec<String> {
    Vec::new()
}

fn find_port_for_serial(serial: &str, ports: &[String]) -> Option<String> {
    let normalized_serial = normalize(serial);

    ports
        .iter()
        .find(|port| normalize(port).contains(&normalized_serial))
        .cloned()
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn print_devices(devices: &[Device]) {
    let header_device = "device";
    let header_port = "port";

    let device_width = devices
        .iter()
        .map(|d| d.device.len())
        .max()
        .unwrap_or(0)
        .max(header_device.len());

    println!(
        "{:<device_width$}  {}",
        header_device,
        header_port,
        device_width = device_width
    );

    for d in devices {
        println!(
            "{:<device_width$}  {}",
            d.device,
            d.port,
            device_width = device_width
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{find_port_for_serial, normalize};

    #[test]
    fn normalize_ignores_punctuation() {
        assert_eq!(normalize("PDU1-Y013705"), "pdu1y013705");
        assert_eq!(
            normalize("/dev/cu.usbmodemPDU1_Y013705"),
            "devcuusbmodempdu1y013705"
        );
    }

    #[test]
    fn finds_matching_port_for_serial() {
        let ports = vec![
            "/dev/cu.usbmodemPDU1_Y013705".to_string(),
            "/dev/ttyACM0".to_string(),
        ];

        let got = find_port_for_serial("PDU1-Y013705", &ports);
        assert_eq!(got.as_deref(), Some("/dev/cu.usbmodemPDU1_Y013705"));
    }
}
