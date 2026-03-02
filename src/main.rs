mod cli;
mod command;
mod device;
mod platform;
mod screenshot;
mod stats;
mod usb;
mod version;

use crate::cli::parse_command_from_env;
use crate::command::{Command, LogFormat};
use crate::device::{DeviceList, DeviceLog};
use crate::platform::open_with_default_viewer;
use crate::screenshot::capture_screenshot;
use crate::stats::print_stats_json;
use crate::usb::{get_device, wait_for_selected_device};
use crate::version::print_version_json;

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
    let parsed = parse_command_from_env()?;
    let log_format = parsed.log_format;
    match parsed.command {
        Command::List => {
            let devices = DeviceList::discover()?;
            print!("{devices}");
        }
        Command::Eject { device } => {
            let mut device = get_device(&device)?;
            device.eject_device()?;
            emit_log(log_format, device.log("Ejected device"));
        }
        Command::Serial { device, command } => {
            let device = get_device(&device)?;
            device.send_command(command.as_str())?;
            emit_log(
                log_format,
                device.log(format!("Sent serial command: {}", command.as_str())),
            );
        }
        Command::Input { device, input } => {
            let device = get_device(&device)?;
            let serial = input.to_serial_command();
            device.send_command(serial.as_str())?;
            let log = device.log(format!("Sent input command: {serial}"));
            emit_log(log_format, log);
        }
        Command::Crank { device, crank } => {
            let device = get_device(&device)?;
            let serial = crank.to_serial_command();
            device.send_command(serial.as_str())?;
            let log = device.log(format!("Sent crank command: {serial}"));
            emit_log(log_format, log);
        }
        Command::Version { device, json } => {
            let device = get_device(&device)?;
            let version = device.fetch_version()?;
            if json {
                print_version_json(&version);
            } else {
                emit_log(log_format, device.log("Version"));
                match log_format {
                    LogFormat::Text => {
                        for (k, v) in version {
                            println!("{k}: {v}");
                        }
                    }
                    LogFormat::Json => print_version_json(&version),
                }
            }
        }
        Command::Mount { device, open } => {
            let mut device = wait_for_selected_device(&device)?;
            device.mount_device()?;
            let mount_path = device
                .mount_path()
                .map(|path| path.display().to_string())
                .ok_or_else(|| "mount completed but no mount path was recorded".to_string())?;
            let log = device.log(format!("Mounted device at {mount_path}"));
            emit_log(log_format, log);
            if open {
                open_with_default_viewer(&mount_path)?;
            }
        }
        Command::Screenshot {
            device,
            filename,
            open,
        } => {
            let device = get_device(&device)?;
            let (_serial, path, bytes, inspect) = capture_screenshot(&device, filename)?;
            let log = device.log(format!("Captured screenshot to {path} ({bytes} bytes)"));
            emit_log(log_format, log);
            emit_log(log_format, device.log(inspect));
            if open {
                open_with_default_viewer(&path)?;
            }
        }
        Command::Stats { device, json } => {
            let device = get_device(&device)?;
            let stats = device.fetch_stats()?;
            if json {
                print_stats_json(&stats);
            } else {
                emit_log(log_format, device.log("Stats"));
                for (k, v) in stats {
                    println!("{k}: {v}");
                }
            }
        }
    }
    Ok(())
}

fn emit_log(log_format: LogFormat, log: DeviceLog) {
    match log_format {
        LogFormat::Text => println!("{log}"),
        LogFormat::Json => println!("{}", log.to_json()),
    }
}
