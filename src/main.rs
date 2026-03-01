mod cli;
mod command;
mod device;
mod platform;
mod screenshot;
mod stats;
mod usb;

use crate::command::{Command, parse_command_from_env};
use crate::device::DeviceList;
use crate::platform::open_with_default_viewer;
use crate::screenshot::capture_screenshot;
use crate::stats::{fetch_stats, print_stats_json};
use crate::usb::{resolve_mount_target, resolve_selected_device};

pub(crate) const PLAYDATE_VENDOR_ID: u16 = 0x1331;
pub(crate) const PLAYDATE_PRODUCT_ID_MSC: u16 = 0x5741;
pub(crate) const PLAYDATE_PRODUCT_ID_APP: u16 = 0x5740;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let command = parse_command_from_env()?;
    match command {
        Command::List => {
            let devices = DeviceList::discover()?;
            print!("{devices}");
        }
        Command::Eject { device } => {
            let mut resolved = resolve_selected_device(&device)?;
            let serial = resolved.serial().to_string();
            resolved.eject_device()?;
            println!("ejected {serial}");
        }
        Command::Serial { device, command } => {
            let resolved = resolve_selected_device(&device)?;
            let (serial, port) = resolved.send_command(command.as_str())?;
            println!("sent '{}' to {serial} on {port}", command.as_str());
        }
        Command::Mount { device, open } => {
            let mut resolved = resolve_mount_target(&device)?;
            let serial = resolved.serial().to_string();
            resolved.mount_device()?;
            let mount_path = resolved
                .mount_path()
                .map(|path| path.display().to_string())
                .ok_or_else(|| "mount completed but no mount path was recorded".to_string())?;
            println!("mounted {serial} at {mount_path}");
            if open {
                open_with_default_viewer(&mount_path)?;
                println!("opened {mount_path}");
            }
        }
        Command::Screenshot {
            device,
            filename,
            open,
        } => {
            let resolved = resolve_selected_device(&device)?;
            let (serial, path, bytes, inspect) = capture_screenshot(&resolved, filename)?;
            println!("captured screenshot from {serial} to {path} ({bytes} bytes)");
            println!("{inspect}");
            if open {
                open_with_default_viewer(&path)?;
                println!("opened {path}");
            }
        }
        Command::Stats { device, json } => {
            let resolved = resolve_selected_device(&device)?;
            let (serial, entries) = fetch_stats(&resolved)?;
            if json {
                print_stats_json(&entries);
            } else {
                println!("stats from {serial}");
                for (k, v) in entries {
                    println!("{k}: {v}");
                }
            }
        }
        Command::Hibernate { device } => {
            let resolved = resolve_selected_device(&device)?;
            let (serial, port) = resolved.send_command("hibernate")?;
            println!("sent 'hibernate' to {serial} on {port}");
        }
    }
    Ok(())
}
