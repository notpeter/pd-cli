use crate::device::DeviceSerial;
use crate::platform::SerialPortPath;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::process::Command;
use std::time::{Duration, Instant};

#[cfg(unix)]
pub(crate) fn configure_serial_port_8n1_115200(
    fd: std::os::fd::RawFd,
) -> Result<(), std::io::Error> {
    let mut term = unsafe { std::mem::zeroed::<libc::termios>() };

    if unsafe { libc::tcgetattr(fd, &mut term) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    if unsafe { libc::cfsetispeed(&mut term, libc::B115200) } != 0 {
        return Err(std::io::Error::last_os_error());
    }
    if unsafe { libc::cfsetospeed(&mut term, libc::B115200) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    term.c_iflag = 0;
    term.c_oflag = 0;
    term.c_lflag = 0;
    term.c_cflag &= !(libc::PARENB | libc::CSTOPB | libc::CSIZE | libc::CRTSCTS);
    term.c_cflag |= libc::CS8 | libc::CLOCAL | libc::CREAD;
    term.c_cc[libc::VMIN] = 0;
    term.c_cc[libc::VTIME] = 1;

    if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &term) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

pub(crate) fn list_serial_ports_from_dev() -> Vec<SerialPortPath> {
    let mut ports = Vec::new();

    let entries = match fs::read_dir("/dev") {
        Ok(entries) => entries,
        Err(_) => return ports,
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();

        let is_usb_serial = name.starts_with("cu.usbmodem")
            || name.starts_with("tty.usbmodem")
            || name.starts_with("ttyACM")
            || name.starts_with("ttyUSB");

        if is_usb_serial {
            let Some(device_key) = extract_device_key_from_port_name(&name) else {
                continue;
            };
            ports.push(SerialPortPath::new(
                format!("/dev/{name}").into(),
                device_key,
            ));
        }
    }

    ports.sort();
    ports
}

fn extract_device_key_from_port_name(name: &str) -> Option<String> {
    let upper = name.to_ascii_uppercase();
    let bytes = upper.as_bytes();
    for start in 0..bytes.len() {
        if bytes[start] != b'Y' {
            continue;
        }
        if start + 7 > bytes.len() {
            break;
        }
        let candidate = &upper[start..start + 7];
        if !candidate.chars().all(|c| c.is_ascii_alphanumeric()) {
            continue;
        }
        if let Some(serial) = DeviceSerial::parse(candidate) {
            return Some(serial.core().to_string());
        }
    }
    None
}

pub(crate) fn list_mounts_via_mount_command() -> Result<Vec<(String, String)>, String> {
    let output = Command::new("mount")
        .output()
        .map_err(|e| format!("failed to run mount: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("mount failed: {stderr}"));
    }

    let text = String::from_utf8(output.stdout)
        .map_err(|e| format!("mount returned non-UTF8 output: {e}"))?;

    Ok(text
        .lines()
        .filter_map(|line| {
            let (source, rest) = line.split_once(" on ")?;
            let (target, _) = rest.split_once(" (")?;
            Some((source.trim().to_string(), target.trim().to_string()))
        })
        .collect())
}

#[cfg(unix)]
pub(crate) fn build_disk_mounts(
    mounts: &[(String, String)],
) -> Result<HashMap<String, String>, String> {
    let mut by_disk = HashMap::new();

    for (source, target) in mounts {
        let disk = source.strip_prefix("/dev/").and_then(|name| {
            if !name.starts_with("disk") {
                return None;
            }
            let suffix = &name["disk".len()..];
            let digits: String = suffix.chars().take_while(|c| c.is_ascii_digit()).collect();
            if digits.is_empty() {
                None
            } else {
                Some(format!("disk{digits}"))
            }
        });
        if let Some(disk) = disk {
            by_disk.entry(disk).or_insert_with(|| target.clone());
        }
    }

    Ok(by_disk)
}

impl SerialPortPath {
    pub(crate) fn send_serial_command_and_capture(&self, command: &str) -> Result<Vec<u8>, String> {
        const SCREEN_CAPTURE_TIMEOUT: Duration = Duration::from_secs(2);
        const SCREEN_CAPTURE_IDLE: Duration = Duration::from_millis(300);

        let mut port = self.open_serial_port()?;
        let payload = format!("{command}\n");
        port.write_all(payload.as_bytes())
            .map_err(|e| format!("failed to write command to '{}': {e}", self))?;
        port.flush()
            .map_err(|e| format!("failed to flush command to '{}': {e}", self))?;

        let mut out = Vec::new();
        let mut buf = [0u8; 4096];
        let start = Instant::now();
        let mut last_read = Instant::now();

        loop {
            match port.read(&mut buf) {
                Ok(n) if n > 0 => {
                    out.extend_from_slice(&buf[..n]);
                    last_read = Instant::now();
                }
                Ok(_) => {}
                Err(e)
                    if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    return Err(format!("failed to read serial data from '{}': {e}", self));
                }
            }

            if !out.is_empty() && Instant::now().duration_since(last_read) >= SCREEN_CAPTURE_IDLE {
                break;
            }

            if Instant::now().duration_since(start) >= SCREEN_CAPTURE_TIMEOUT {
                break;
            }
        }

        if out.is_empty() {
            return Err(format!(
                "no screenshot data received from '{}'; verify the device is unlocked and in app mode",
                self
            ));
        }

        Ok(out)
    }

    pub(crate) fn send_serial_command(&self, command: &str) -> Result<(), String> {
        let mut port = self.open_serial_port()?;
        let payload = format!("{command}\n");
        port.write_all(payload.as_bytes())
            .map_err(|e| format!("failed to write command to '{}': {e}", self))?;
        port.flush()
            .map_err(|e| format!("failed to flush command to '{}': {e}", self))?;
        Ok(())
    }

    fn open_serial_port(&self) -> Result<std::fs::File, String> {
        let port = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.as_path())
            .map_err(|e| format!("failed to open serial port '{}': {e}", self))?;

        configure_serial_port_8n1_115200(port.as_raw_fd())
            .map_err(|e| format!("failed to configure serial port '{}': {e}", self))?;

        Ok(port)
    }
}
