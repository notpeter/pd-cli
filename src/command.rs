use crate::cli::{DeviceCommand as CliDeviceCommand, parse_cli_from_env};
use crate::device::DeviceSerial;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeviceSelector {
    Auto,
    BySerial(DeviceSerial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SerialCommand(String);

impl SerialCommand {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    fn parse(raw: String) -> Result<Self, String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err("serial command cannot be empty".to_string());
        }

        Ok(Self(trimmed.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Command {
    List,
    Eject {
        device: DeviceSelector,
    },
    Serial {
        device: DeviceSelector,
        command: SerialCommand,
    },
    Mount {
        device: DeviceSelector,
        open: bool,
    },
    Screenshot {
        device: DeviceSelector,
        filename: Option<PathBuf>,
        open: bool,
    },
    Stats {
        device: DeviceSelector,
        json: bool,
    },
    Hibernate {
        device: DeviceSelector,
    },
}

pub(crate) fn parse_command_from_env() -> Result<Command, String> {
    let cli_command = parse_cli_from_env()?;
    map_parsed_command(cli_command)
}

fn map_parsed_command(command: CliDeviceCommand) -> Result<Command, String> {
    match command {
        CliDeviceCommand::List => Ok(Command::List),
        CliDeviceCommand::Eject { device_id } => Ok(Command::Eject {
            device: parse_device_selector(device_id)?,
        }),
        CliDeviceCommand::Serial { device_id, command } => Ok(Command::Serial {
            device: parse_device_selector(device_id)?,
            command: SerialCommand::parse(command)?,
        }),
        CliDeviceCommand::Mount { device_id, open } => Ok(Command::Mount {
            device: parse_device_selector(device_id)?,
            open,
        }),
        CliDeviceCommand::Screenshot {
            device_id,
            filename,
            open,
        } => {
            let filename = filename.map(PathBuf::from);
            Ok(Command::Screenshot {
                device: parse_device_selector(device_id)?,
                filename,
                open,
            })
        }
        CliDeviceCommand::Stats { device_id, json } => Ok(Command::Stats {
            device: parse_device_selector(device_id)?,
            json,
        }),
        CliDeviceCommand::Hibernate { device_id } => Ok(Command::Hibernate {
            device: parse_device_selector(device_id)?,
        }),
    }
}

fn parse_device_selector(raw: Option<String>) -> Result<DeviceSelector, String> {
    match raw {
        None => Ok(DeviceSelector::Auto),
        Some(id) => {
            let serial = DeviceSerial::parse(&id).ok_or_else(|| {
                format!("invalid device serial '{id}'; expected forms like PDU1-Y012345 or Y012345")
            })?;
            Ok(DeviceSelector::BySerial(serial))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Command, DeviceSelector, SerialCommand, map_parsed_command, parse_device_selector,
    };
    use crate::cli::DeviceCommand as CliDeviceCommand;

    #[test]
    fn parses_device_selector_from_serial() {
        let selector = parse_device_selector(Some("PDU1-Y012345".to_string())).expect("selector");
        assert_eq!(
            selector,
            DeviceSelector::BySerial(crate::device::DeviceSerial::parse("Y012345").unwrap())
        );
    }

    #[test]
    fn rejects_invalid_device_selector() {
        let err = parse_device_selector(Some("bad".to_string())).expect_err("should fail");
        assert!(err.contains("invalid device serial"));
    }

    #[test]
    fn rejects_empty_serial_command() {
        let err = SerialCommand::parse("  ".to_string()).expect_err("should fail");
        assert_eq!(err, "serial command cannot be empty");
    }

    #[test]
    fn maps_stats_command_with_selector() {
        let mapped = map_parsed_command(CliDeviceCommand::Stats {
            device_id: Some("Y012345".to_string()),
            json: true,
        })
        .expect("mapping should succeed");

        assert_eq!(
            mapped,
            Command::Stats {
                device: DeviceSelector::BySerial(
                    crate::device::DeviceSerial::parse("Y012345").unwrap()
                ),
                json: true,
            }
        );
    }
}
