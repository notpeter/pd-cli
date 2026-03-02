use crate::platform::SerialPortPath;
use serde_json::json;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct DeviceSerial {
    core: String,
}

impl DeviceSerial {
    pub(crate) fn parse(raw: &str) -> Option<Self> {
        let compact: String = raw
            .trim()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .flat_map(|c| c.to_uppercase())
            .collect();

        let core = compact.strip_prefix("PDU1").unwrap_or(&compact);
        if core.len() != 7 || !core.starts_with('Y') {
            return None;
        }
        if !core[1..].chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        Some(Self {
            core: core.to_string(),
        })
    }

    pub(crate) fn core(&self) -> &str {
        &self.core
    }
}

impl fmt::Display for DeviceSerial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PDU1-{}", self.core)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Device {
    serial: DeviceSerial,
    port: Option<SerialPortPath>,
    mount_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeviceLog {
    pub(crate) serial_number: String,
    pub(crate) msg: String,
}

impl DeviceLog {
    #[allow(dead_code)]
    pub(crate) fn to_json(&self) -> String {
        json!({
            "serial_number": self.serial_number,
            "msg": self.msg,
        })
        .to_string()
    }
}

impl Device {
    pub(crate) fn new(
        serial: DeviceSerial,
        port: Option<SerialPortPath>,
        mount_path: Option<PathBuf>,
    ) -> Self {
        Self {
            serial,
            port,
            mount_path,
        }
    }

    pub(crate) fn serial(&self) -> &DeviceSerial {
        &self.serial
    }

    pub(crate) fn port(&self) -> Option<&SerialPortPath> {
        self.port.as_ref()
    }

    pub(crate) fn mount_path(&self) -> Option<&Path> {
        self.mount_path.as_deref()
    }

    pub(crate) fn mounted(&self) -> bool {
        self.mount_path.is_some()
    }

    pub(crate) fn set_mount_path(&mut self, mount_path: PathBuf) {
        self.mount_path = Some(mount_path);
    }

    pub(crate) fn clear_mount_path(&mut self) {
        self.mount_path = None;
    }

    pub(crate) fn log(&self, msg: impl Into<String>) -> DeviceLog {
        DeviceLog {
            serial_number: self.serial().to_string(),
            msg: msg.into(),
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serial)
    }
}

impl fmt::Display for DeviceLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.serial_number, self.msg)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeviceList(pub(crate) Vec<Device>);

impl DeviceList {
    pub(crate) fn as_slice(&self) -> &[Device] {
        &self.0
    }
}

impl fmt::Display for DeviceList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header_device = "device";
        let header_port = "port";
        let header_mounted = "mounted";
        let header_mount_path = "mount_path";

        let device_width = self
            .0
            .iter()
            .map(|device| device.serial().to_string().len())
            .max()
            .unwrap_or(0)
            .max(header_device.len());

        let port_width = self
            .0
            .iter()
            .map(|device| device.port().map(|p| p.to_string().len()).unwrap_or(0))
            .max()
            .unwrap_or(0)
            .max(header_port.len());

        writeln!(
            f,
            "{:<device_width$}  {:<port_width$}  {:<7}  {}",
            header_device,
            header_port,
            header_mounted,
            header_mount_path,
            device_width = device_width,
            port_width = port_width
        )?;

        for device in &self.0 {
            let mounted = if device.mounted() { "yes" } else { "no" };
            let port = device.port().map(ToString::to_string).unwrap_or_default();
            let mount_path = device
                .mount_path()
                .map(|p| p.display().to_string())
                .unwrap_or_default();
            writeln!(
                f,
                "{:<device_width$}  {:<port_width$}  {:<7}  {}",
                device.serial(),
                port,
                mounted,
                mount_path,
                device_width = device_width,
                port_width = port_width
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Device, DeviceSerial};

    #[test]
    fn device_log_displays_with_serial_prefix() {
        let device = Device::new(DeviceSerial::parse("Y012345").unwrap(), None, None);
        let log = device.log("Ejected device");
        assert_eq!(log.to_string(), "PDU1-Y012345: Ejected device");
    }

    #[test]
    fn device_log_renders_json() {
        let device = Device::new(DeviceSerial::parse("Y012345").unwrap(), None, None);
        let log = device.log("Sent serial command: hibernate");
        assert_eq!(
            log.to_json(),
            r#"{"msg":"Sent serial command: hibernate","serial_number":"PDU1-Y012345"}"#
        );
    }

    #[test]
    fn serial_parse_rejects_invalid_lengths_and_non_digits() {
        assert!(DeviceSerial::parse("Y01234").is_none());
        assert!(DeviceSerial::parse("Y0123456").is_none());
        assert!(DeviceSerial::parse("Y01A345").is_none());
    }
}
