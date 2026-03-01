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
        open: bool,
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
    Mount {
        #[arg(long = "open")]
        open: bool,
    },
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
                DeviceSubcommand::Mount { open } => DeviceCommand::Mount { device_id, open },
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
    use super::{Cli, DeviceCommand, map_parsed_device_command};
    use clap::Parser;

    #[test]
    fn parses_mount_with_open_flag() {
        let parsed = Cli::try_parse_from(["pd", "device", "-d", "PDU1-Y012345", "mount", "--open"])
            .expect("mount parse should succeed");
        assert_eq!(
            map_parsed_device_command(parsed),
            DeviceCommand::Mount {
                device_id: Some("PDU1-Y012345".to_string()),
                open: true
            }
        );
    }
}
