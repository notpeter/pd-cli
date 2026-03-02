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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputAction {
    Tap,
    Press,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputButton {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
}

impl InputButton {
    fn parse(raw: &str) -> Option<Self> {
        match raw {
            "a" => Some(Self::A),
            "b" => Some(Self::B),
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InputCommand {
    action: InputAction,
    button: InputButton,
}

impl InputCommand {
    pub(crate) fn parse(raw: String) -> Result<Self, String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err("input cannot be empty; expected [-+]{a,b,up,down,left,right}".to_string());
        }

        let (action, button_raw) = if let Some(rest) = trimmed.strip_prefix('+') {
            (InputAction::Press, rest)
        } else if let Some(rest) = trimmed.strip_prefix('-') {
            (InputAction::Release, rest)
        } else {
            (InputAction::Tap, trimmed)
        };

        let button_lc = button_raw.trim().to_ascii_lowercase();
        let button = InputButton::parse(&button_lc).ok_or_else(|| {
            format!("invalid input '{trimmed}'; expected [-+]{{a,b,up,down,left,right}}")
        })?;

        Ok(Self { action, button })
    }

    pub(crate) fn to_serial_command(&self) -> String {
        match self.action {
            InputAction::Tap => format!("btn {}", self.button.as_str()),
            InputAction::Press => format!("btn +{}", self.button.as_str()),
            InputAction::Release => format!("btn -{}", self.button.as_str()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CrankCommand {
    Rotate(i32),
    Enable,
    Disable,
}

impl CrankCommand {
    pub(crate) fn parse(raw: String) -> Result<Self, String> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(
                "crank cannot be empty; expected [-+]<degrees> or [enable|disable]".to_string(),
            );
        }

        let lower = trimmed.to_ascii_lowercase();
        match lower.as_str() {
            "enable" => return Ok(Self::Enable),
            "disable" => return Ok(Self::Disable),
            _ => {}
        }

        let normalized = if trimmed.starts_with('+') || trimmed.starts_with('-') {
            trimmed.to_string()
        } else {
            format!("+{trimmed}")
        };

        let degrees = normalized.parse::<i32>().map_err(|_| {
            format!("invalid crank '{trimmed}'; expected [-+]<degrees> or [enable|disable]")
        })?;

        Ok(Self::Rotate(degrees))
    }

    pub(crate) fn to_serial_command(&self) -> String {
        match self {
            Self::Rotate(degrees) => format!("crank {degrees:+}"),
            Self::Enable => "crank enable".to_string(),
            Self::Disable => "crank disable".to_string(),
        }
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
    Input {
        device: DeviceSelector,
        input: InputCommand,
    },
    Crank {
        device: DeviceSelector,
        crank: CrankCommand,
    },
    Version {
        device: DeviceSelector,
        json: bool,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum LogFormat {
    Text,
    Json,
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
