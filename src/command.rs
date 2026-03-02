use crate::device::DeviceSerial;
use clap::ValueEnum;
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

    pub(crate) fn parse(raw: String) -> Result<Self, String> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum LogFormat {
    Text,
    Jsonl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedCommand {
    pub(crate) command: Command,
    pub(crate) log_format: LogFormat,
}

pub(crate) fn parse_device_selector(raw: Option<String>) -> Result<DeviceSelector, String> {
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
    use super::{DeviceSelector, SerialCommand, parse_device_selector};

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
}
