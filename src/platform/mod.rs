#[cfg(all(unix, not(target_os = "macos")))]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(unix)]
mod unix;
#[cfg(target_os = "windows")]
mod windows;

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
pub(crate) use unix::{build_disk_mounts, send_serial_command, send_serial_command_and_capture};
#[cfg(target_os = "windows")]
pub(crate) use windows::{
    build_disk_mounts, eject_target, list_mounts, list_playdate_disks_by_serial, list_serial_ports,
    open_with_default_viewer, send_serial_command, send_serial_command_and_capture,
};
