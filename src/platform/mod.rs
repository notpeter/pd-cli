use std::fmt;
use std::path::{Path, PathBuf};

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(unix)]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SerialPortPath(PathBuf);

impl SerialPortPath {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub(crate) fn as_path(&self) -> &Path {
        &self.0
    }
}

impl fmt::Display for SerialPortPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) use linux::{
    eject_target, list_mounts, list_playdate_disks_by_serial, list_serial_ports,
    open_with_default_viewer,
};
#[cfg(target_os = "macos")]
pub(crate) use macos::{
    eject_target, list_mounts, list_playdate_disks_by_serial, list_serial_ports,
    open_with_default_viewer,
};
#[cfg(unix)]
pub(crate) use unix::build_disk_mounts;
#[cfg(target_os = "windows")]
pub(crate) use windows::{
    build_disk_mounts, eject_target, list_mounts, list_playdate_disks_by_serial, list_serial_ports,
    open_with_default_viewer,
};
