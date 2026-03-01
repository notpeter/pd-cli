mod cli;
mod device;
mod platform;
mod screenshot;
mod stats;

use crate::cli::{DeviceCommand, parse_cli_from_env};
use crate::device::{
    eject_device, list_devices, mount_device, print_devices, send_serial_command_to_device,
};
use crate::platform::open_with_default_viewer;
use crate::screenshot::capture_screenshot;
use crate::stats::{fetch_stats, print_stats_json};

pub(crate) const PLAYDATE_VENDOR_ID: u16 = 0x1331;
pub(crate) const PLAYDATE_PRODUCT_ID_MSC: u16 = 0x5741;
pub(crate) const PLAYDATE_PRODUCT_ID_APP: u16 = 0x5740;

fn main() {
    let result = (|| -> Result<(), String> {
        let command = parse_cli_from_env()?;
        match command {
            DeviceCommand::List => {
                let devices = list_devices()?;
                print_devices(&devices);
            }
            DeviceCommand::Eject { device_id } => {
                let serial = eject_device(device_id.as_deref())?;
                println!("ejected {serial}");
            }
            DeviceCommand::Serial { device_id, command } => {
                let (serial, port) = send_serial_command_to_device(device_id.as_deref(), &command)?;
                println!("sent '{command}' to {serial} on {port}");
            }
            DeviceCommand::Mount { device_id, open } => {
                let (serial, mount_path) = mount_device(device_id.as_deref())?;
                println!("mounted {serial} at {mount_path}");
                if open {
                    open_with_default_viewer(&mount_path)?;
                    println!("opened {mount_path}");
                }
            }
            DeviceCommand::Screenshot {
                device_id,
                filename,
                open,
            } => {
                let (serial, path, bytes, inspect) =
                    capture_screenshot(device_id.as_deref(), filename.as_deref())?;
                println!("captured screenshot from {serial} to {path} ({bytes} bytes)");
                println!("{inspect}");
                if open {
                    open_with_default_viewer(&path)?;
                    println!("opened {path}");
                }
            }
            DeviceCommand::Stats { device_id, json } => {
                let (serial, entries) = fetch_stats(device_id.as_deref())?;
                if json {
                    print_stats_json(&entries);
                } else {
                    println!("stats from {serial}");
                    for (k, v) in entries {
                        println!("{k}: {v}");
                    }
                }
            }
            DeviceCommand::Hibernate { device_id } => {
                let (serial, port) =
                    send_serial_command_to_device(device_id.as_deref(), "hibernate")?;
                println!("sent 'hibernate' to {serial} on {port}");
            }
        }
        Ok(())
    })();

    if let Err(err) = result {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

pub(crate) fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}
