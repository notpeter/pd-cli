mod cli;
mod command;
mod device;
mod platform;
mod screenshot;
mod stats;
mod usb;

use crate::cli::parse_command_from_env;
use crate::command::{Command, LogFormat};
use crate::device::{DeviceList, DeviceLog};
use crate::platform::open_with_default_viewer;
use crate::screenshot::capture_screenshot;
use crate::stats::print_stats_json;
use crate::usb::{get_device, wait_for_selected_device};

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
            let resolved = get_device(&device)?;
            resolved.send_command(command.as_str())?;
            emit_log(
                log_format,
                resolved.log(format!("Sent serial command: {}", command.as_str())),
            );
        }
        Command::Mount { device, open } => {
            let mut device = wait_for_selected_device(&device)?;
            device.mount_device()?;
            let mount_path = device
                .mount_path()
                .map(|path| path.display().to_string())
                .ok_or_else(|| "mount completed but no mount path was recorded".to_string())?;
            emit_log(
                log_format,
                device.log(format!("Mounted device at {mount_path}")),
            );
            if open {
                open_with_default_viewer(&mount_path)?;
                emit_log(log_format, device.log(format!("Opened {mount_path}")));
            }
        }
        Command::Screenshot {
            device,
            filename,
            open,
        } => {
            let device = get_device(&device)?;
            let (_serial, path, bytes, inspect) = capture_screenshot(&device, filename)?;
            emit_log(
                log_format,
                device.log(format!("Captured screenshot to {path} ({bytes} bytes)")),
            );
            emit_log(log_format, device.log(inspect));
            if open {
                open_with_default_viewer(&path)?;
                emit_log(log_format, device.log(format!("Opened {path}")));
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
        Command::Hibernate { device } => {
            let resolved = get_device(&device)?;
            resolved.send_command("hibernate")?;
            emit_log(log_format, resolved.log("Sent serial command: hibernate"));
        }
    }
    Ok(())
}

fn emit_log(log_format: LogFormat, log: DeviceLog) {
    match log_format {
        LogFormat::Text => println!("{log}"),
        LogFormat::Jsonl => println!("{}", log.to_jsonl()),
    }
}
