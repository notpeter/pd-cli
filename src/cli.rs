use clap::{Parser, Subcommand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeviceCommand {
    List,
    Eject {
        device_id: Option<String>,
    },
    Serial {
        device_id: Option<String>,
        command: String,
    },
    Mount {
        device_id: Option<String>,
    },
    Screenshot {
        device_id: Option<String>,
        filename: Option<String>,
        open: bool,
    },
    Stats {
        device_id: Option<String>,
        json: bool,
    },
    Hibernate {
        device_id: Option<String>,
    },
}

#[derive(Debug, Parser)]
#[command(name = "pd")]
struct Cli {
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
    Mount,
    Datadisk,
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
    Hibernate,
}

pub(crate) fn parse_device_command(args: &[String]) -> Result<DeviceCommand, String> {
    let mut argv = Vec::with_capacity(args.len() + 2);
    argv.push("pd".to_string());
    argv.push("device".to_string());
    argv.extend(args.iter().cloned());

    let parsed = Cli::try_parse_from(argv).map_err(|e| e.to_string())?;
    Ok(map_parsed_device_command(parsed))
}

pub(crate) fn parse_cli_from_env() -> Result<DeviceCommand, String> {
    let parsed = Cli::try_parse().map_err(|e| e.to_string())?;
    Ok(map_parsed_device_command(parsed))
}

fn map_parsed_device_command(parsed: Cli) -> DeviceCommand {
    match parsed.command {
        TopLevelCommand::Device(device_cli) => {
            let DeviceCli { device_id, command } = device_cli;
            match command {
                DeviceSubcommand::List => DeviceCommand::List,
                DeviceSubcommand::Eject => DeviceCommand::Eject { device_id },
                DeviceSubcommand::Mount => DeviceCommand::Mount { device_id },
                DeviceSubcommand::Datadisk => DeviceCommand::Serial {
                    device_id,
                    command: "datadisk".to_string(),
                },
                DeviceSubcommand::Serial { command } => {
                    DeviceCommand::Serial { device_id, command }
                }
                DeviceSubcommand::Stats { json } => DeviceCommand::Stats { device_id, json },
                DeviceSubcommand::Screenshot { filename, open } => DeviceCommand::Screenshot {
                    device_id,
                    filename,
                    open,
                },
                DeviceSubcommand::Hibernate => DeviceCommand::Hibernate { device_id },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeviceCommand, parse_device_command};

    fn parse(args: &[&str]) -> DeviceCommand {
        let owned = args.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        parse_device_command(&owned).expect("command should parse")
    }

    #[test]
    fn parses_list_command() {
        assert_eq!(parse(&["list"]), DeviceCommand::List);
    }

    #[test]
    fn parses_unmount_as_eject_alias() {
        assert_eq!(
            parse(&["-d", "PDU1-Y013705", "unmount"]),
            DeviceCommand::Eject {
                device_id: Some("PDU1-Y013705".to_string())
            }
        );
    }

    #[test]
    fn maps_datadisk_to_serial_command() {
        assert_eq!(
            parse(&["datadisk"]),
            DeviceCommand::Serial {
                device_id: None,
                command: "datadisk".to_string()
            }
        );
    }

    #[test]
    fn parses_serial_with_device() {
        assert_eq!(
            parse(&["-d", "PDU1-Y013705", "serial", "help"]),
            DeviceCommand::Serial {
                device_id: Some("PDU1-Y013705".to_string()),
                command: "help".to_string()
            }
        );
    }

    #[test]
    fn parses_stats_json_with_device() {
        assert_eq!(
            parse(&["-d", "PDU1-Y013705", "stats", "--json"]),
            DeviceCommand::Stats {
                device_id: Some("PDU1-Y013705".to_string()),
                json: true
            }
        );
    }

    #[test]
    fn parses_screenshot_flags() {
        assert_eq!(
            parse(&["screenshot", "-f", "capture.png", "--open"]),
            DeviceCommand::Screenshot {
                device_id: None,
                filename: Some("capture.png".to_string()),
                open: true
            }
        );
    }

    #[test]
    fn parses_hibernate_without_device() {
        assert_eq!(
            parse(&["hibernate"]),
            DeviceCommand::Hibernate { device_id: None }
        );
    }

    #[test]
    fn rejects_missing_subcommand_after_device_flag() {
        let args = ["-d", "PDU1-Y013705"]
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<_>>();
        assert!(parse_device_command(&args).is_err());
    }
}
