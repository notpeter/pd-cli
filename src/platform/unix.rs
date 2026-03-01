use std::collections::HashMap;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
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

#[cfg(unix)]
pub(crate) fn build_disk_mounts(mounts: &[(String, String)]) -> HashMap<String, String> {
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

    by_disk
}

pub(crate) fn send_serial_command_and_capture(
    port_path: &str,
    command: &str,
) -> Result<Vec<u8>, String> {
    const SCREEN_CAPTURE_TIMEOUT: Duration = Duration::from_secs(2);
    const SCREEN_CAPTURE_IDLE: Duration = Duration::from_millis(300);

    let mut port = open_serial_port(port_path)?;

    let payload = format!("{command}\n");
    port.write_all(payload.as_bytes())
        .map_err(|e| format!("failed to write command to '{port_path}': {e}"))?;
    port.flush()
        .map_err(|e| format!("failed to flush command to '{port_path}': {e}"))?;

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
                return Err(format!(
                    "failed to read serial data from '{port_path}': {e}"
                ));
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
            port_path
        ));
    }

    Ok(out)
}

pub(crate) fn send_serial_command(port_path: &str, command: &str) -> Result<(), String> {
    let mut port = open_serial_port(port_path)?;
    let payload = format!("{command}\n");
    port.write_all(payload.as_bytes())
        .map_err(|e| format!("failed to write command to '{port_path}': {e}"))?;
    port.flush()
        .map_err(|e| format!("failed to flush command to '{port_path}': {e}"))?;
    Ok(())
}

fn open_serial_port(port_path: &str) -> Result<std::fs::File, String> {
    let port = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(port_path)
        .map_err(|e| format!("failed to open serial port '{port_path}': {e}"))?;

    configure_serial_port_8n1_115200(port.as_raw_fd())
        .map_err(|e| format!("failed to configure serial port '{port_path}': {e}"))?;

    Ok(port)
}
