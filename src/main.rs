use std::env;
use std::fs;
use std::io;
use std::path::Path;

const DEV_DIR: &str = "/dev";
const PORT_PREFIX: &str = "cu.usbmodemPDU1";
const USBMODEM_PREFIX: &str = "cu.usbmodem";

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
                let devices = list_devices().map_err(|e| e.to_string())?;
                print_devices(&devices);
                Ok(())
            }
            _ => Err("usage: pd device list".to_string()),
        },
        _ => Err("usage: pd device list".to_string()),
    }
}

fn list_devices() -> io::Result<Vec<Device>> {
    let mut devices = Vec::new();

    for entry in fs::read_dir(Path::new(DEV_DIR))? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !name.starts_with(PORT_PREFIX) {
            continue;
        }

        let port = format!("{DEV_DIR}/{name}");
        let device = derive_device_name(&name);
        devices.push(Device { device, port });
    }

    devices.sort_by(|a, b| a.port.cmp(&b.port));
    Ok(devices)
}

fn derive_device_name(port_name: &str) -> String {
    let raw = port_name
        .strip_prefix(USBMODEM_PREFIX)
        .unwrap_or(port_name)
        .trim_start_matches('_');

    raw.replace('_', "-")
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
    use super::derive_device_name;

    #[test]
    fn derives_expected_device_name() {
        let got = derive_device_name("cu.usbmodemPDU1_Y0137051");
        assert_eq!(got, "PDU1-Y0137051");
    }

    #[test]
    fn handles_missing_prefix_gracefully() {
        let got = derive_device_name("PDU1_Y0137051");
        assert_eq!(got, "PDU1-Y0137051");
    }
}
