use crate::command::{
    Command, CrankCommand, InputCommand, LogFormat, ParsedCommand, SerialCommand,
    parse_device_selector,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "pd")]
struct Cli {
    #[arg(long = "log-format", value_enum, default_value = "text", global = true)]
    log_format: LogFormat,
    #[command(subcommand)]
    command: TopLevelCommand,
}

#[derive(Debug, Subcommand)]
enum TopLevelCommand {
    Device(DeviceCli),
}

#[derive(Debug, Parser)]
struct DeviceCli {
    #[arg(short = 'd', long = "device", global = true)]
    device_id: Option<String>,
    #[command(subcommand)]
    command: DeviceSubcommand,
}

#[derive(Debug, Subcommand)]
enum DeviceSubcommand {
    List,
    #[command(alias = "unmount")]
    Eject,
    Mount {
        #[arg(long = "open")]
        open: bool,
    },
    Datadisk,
    Input {
        input: String,
    },
    Crank {
        #[arg(allow_hyphen_values = true)]
        crank: String,
    },
    Version {
        #[arg(long = "json")]
        json: bool,
    },
    Serial {
        command: String,
    },
    Stats {
        #[arg(long = "json")]
        json: bool,
    },
    Screenshot {
        #[arg(short = 'f')]
        filename: Option<String>,
        #[arg(long = "open")]
        open: bool,
    },
}

pub(crate) fn parse_command_from_env() -> Result<ParsedCommand, String> {
    let parsed = Cli::try_parse().map_err(|e| e.to_string())?;
    map_parsed_cli(parsed)
}

fn map_parsed_cli(parsed: Cli) -> Result<ParsedCommand, String> {
    match parsed.command {
        TopLevelCommand::Device(device_cli) => {
            let DeviceCli { device_id, command } = device_cli;
            let command = match command {
                DeviceSubcommand::List => Command::List,
                DeviceSubcommand::Eject => Command::Eject {
                    device: parse_device_selector(device_id)?,
                },
                DeviceSubcommand::Mount { open } => Command::Mount {
                    device: parse_device_selector(device_id)?,
                    open,
                },
                DeviceSubcommand::Datadisk => Command::Serial {
                    device: parse_device_selector(device_id)?,
                    command: SerialCommand::parse("datadisk".to_string())?,
                },
                DeviceSubcommand::Input { input } => Command::Input {
                    device: parse_device_selector(device_id)?,
                    input: InputCommand::parse(input)?,
                },
                DeviceSubcommand::Crank { crank } => Command::Crank {
                    device: parse_device_selector(device_id)?,
                    crank: CrankCommand::parse(crank)?,
                },
                DeviceSubcommand::Version { json } => Command::Version {
                    device: parse_device_selector(device_id)?,
                    json,
                },
                DeviceSubcommand::Serial { command } => Command::Serial {
                    device: parse_device_selector(device_id)?,
                    command: SerialCommand::parse(command)?,
                },
                DeviceSubcommand::Stats { json } => Command::Stats {
                    device: parse_device_selector(device_id)?,
                    json,
                },
                DeviceSubcommand::Screenshot { filename, open } => Command::Screenshot {
                    device: parse_device_selector(device_id)?,
                    filename: filename.map(PathBuf::from),
                    open,
                },
            };
            Ok(ParsedCommand {
                command,
                log_format: parsed.log_format,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cli, map_parsed_cli};
    use crate::command::{Command, DeviceSelector, LogFormat, ParsedCommand, SerialCommand};
    use clap::Parser;

    #[test]
    fn parses_mount_with_open_flag() {
        let parsed = Cli::try_parse_from(["pd", "device", "-d", "PDU1-Y012345", "mount", "--open"])
            .expect("mount parse should succeed");
        assert_eq!(
            map_parsed_cli(parsed).expect("map should succeed"),
            ParsedCommand {
                command: Command::Mount {
                    device: DeviceSelector::BySerial(
                        crate::device::DeviceSerial::parse("Y012345").unwrap()
                    ),
                    open: true
                },
                log_format: LogFormat::Text
            }
        );
    }

    #[test]
    fn parses_json_log_format() {
        let parsed = Cli::try_parse_from(["pd", "--log-format", "json", "device", "list"])
            .expect("parse should succeed");
        let parsed = map_parsed_cli(parsed).expect("map should succeed");
        assert_eq!(parsed.log_format, LogFormat::Json);
    }

    #[test]
    fn parses_datadisk_as_serial_command() {
        let parsed = Cli::try_parse_from(["pd", "device", "-d", "Y012345", "datadisk"])
            .expect("parse should succeed");
        let parsed = map_parsed_cli(parsed).expect("map should succeed");
        assert_eq!(
            parsed,
            ParsedCommand {
                command: Command::Serial {
                    device: DeviceSelector::BySerial(
                        crate::device::DeviceSerial::parse("Y012345").unwrap()
                    ),
                    command: SerialCommand::parse("datadisk".to_string()).unwrap(),
                },
                log_format: LogFormat::Text,
            }
        );
    }

    #[test]
    fn parses_version_command() {
        let parsed = Cli::try_parse_from(["pd", "device", "version"]).expect("parse should work");
        let parsed = map_parsed_cli(parsed).expect("map should succeed");
        assert_eq!(
            parsed,
            ParsedCommand {
                command: Command::Version {
                    device: DeviceSelector::Auto,
                    json: false
                },
                log_format: LogFormat::Text
            }
        );
    }

    #[test]
    fn parses_version_json_flag() {
        let parsed =
            Cli::try_parse_from(["pd", "device", "version", "--json"]).expect("parse should work");
        let parsed = map_parsed_cli(parsed).expect("map should succeed");
        assert_eq!(
            parsed,
            ParsedCommand {
                command: Command::Version {
                    device: DeviceSelector::Auto,
                    json: true
                },
                log_format: LogFormat::Text
            }
        );
    }
}
