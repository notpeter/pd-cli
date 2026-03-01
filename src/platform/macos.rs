use core_foundation_sys::{
    base::{CFGetTypeID, CFRelease, CFTypeRef, kCFAllocatorDefault},
    number::{CFNumberGetTypeID, CFNumberGetValue, CFNumberRef, kCFNumberSInt32Type},
    string::{CFStringGetCString, CFStringGetTypeID, CFStringRef, kCFStringEncodingUTF8},
};
use io_kit_sys::{
    CFSTR, IOIteratorNext, IOObjectRelease, IORegistryEntryCreateCFProperty,
    IORegistryEntryGetChildIterator, IOServiceGetMatchingServices, IOServiceMatching,
    kIOMasterPortDefault,
    keys::kIOServicePlane,
    types::{io_iterator_t, io_object_t, io_registry_entry_t},
};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::os::raw::{c_char, c_void};
use std::process::Command;

const PLAYDATE_VENDOR_ID: u16 = 0x1331;
const PLAYDATE_PRODUCT_ID_MSC: u16 = 0x5741;
const PLAYDATE_PRODUCT_ID_APP: u16 = 0x5740;

pub(crate) fn open_with_default_viewer(path: &str) -> Result<(), String> {
    let status = Command::new("open")
        .arg(path)
        .status()
        .map_err(|e| format!("failed to run open: {e}"))?;
    if status.success() {
        return Ok(());
    }
    Err(format!("open failed for '{path}'"))
}

pub(crate) fn eject_target(disk: &str, mount_path: &str) -> Result<(), String> {
    let target = if !disk.is_empty() {
        format!("/dev/{disk}")
    } else if !mount_path.is_empty() {
        mount_path.to_string()
    } else {
        return Err("device has no known disk or mount path to eject".to_string());
    };

    let output = Command::new("diskutil")
        .args(["eject", &target])
        .output()
        .map_err(|e| format!("failed to run diskutil: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    Err(format!("failed to eject '{target}': {stdout}{stderr}"))
}

pub(crate) fn list_serial_ports() -> Vec<String> {
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
            ports.push(format!("/dev/{name}"));
        }
    }

    ports.sort();
    ports
}

pub(crate) fn list_mounts() -> Result<Vec<(String, String)>, String> {
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

pub(crate) fn list_playdate_disks_by_serial() -> Result<HashMap<String, Vec<String>>, String> {
    const USB_HOST_DEVICE_CLASS: &[u8] = b"IOUSBHostDevice\0";

    let matching = unsafe { IOServiceMatching(USB_HOST_DEVICE_CLASS.as_ptr() as *const c_char) };
    if matching.is_null() {
        return Err("failed to create IOKit matching dictionary for IOUSBHostDevice".to_string());
    }

    let mut iterator: io_iterator_t = 0;
    let result =
        unsafe { IOServiceGetMatchingServices(kIOMasterPortDefault, matching, &mut iterator) };
    if result != 0 {
        return Err(format!(
            "IOServiceGetMatchingServices failed for IOUSBHostDevice (kern_return={result})"
        ));
    }
    let _iterator_guard = IoObjectGuard(iterator as io_object_t);

    let mut by_serial: HashMap<String, HashSet<String>> = HashMap::new();
    loop {
        let service = unsafe { IOIteratorNext(iterator) };
        if service == 0 {
            break;
        }
        let _service_guard = IoObjectGuard(service);

        let vendor = registry_u16_property(service as io_registry_entry_t, b"idVendor\0");
        let product = registry_u16_property(service as io_registry_entry_t, b"idProduct\0");
        if vendor != Some(PLAYDATE_VENDOR_ID) {
            continue;
        }
        if product != Some(PLAYDATE_PRODUCT_ID_MSC) && product != Some(PLAYDATE_PRODUCT_ID_APP) {
            continue;
        }

        let serial =
            registry_string_property(service as io_registry_entry_t, b"kUSBSerialNumberString\0")
                .or_else(|| {
                    registry_string_property(service as io_registry_entry_t, b"USB Serial Number\0")
                });
        let Some(serial) = serial else {
            continue;
        };

        let mut disks = HashSet::new();
        collect_bsd_disks(service as io_registry_entry_t, &mut disks);
        if disks.is_empty() {
            continue;
        }

        by_serial
            .entry(normalize(&serial))
            .or_default()
            .extend(disks);
    }

    Ok(by_serial
        .into_iter()
        .map(|(serial, disks)| {
            let mut sorted = disks.into_iter().collect::<Vec<_>>();
            sorted.sort();
            (serial, sorted)
        })
        .collect())
}

fn collect_bsd_disks(entry: io_registry_entry_t, disks: &mut HashSet<String>) {
    if let Some(name) = registry_string_property(entry, b"BSD Name\0") {
        let disk = if name.starts_with("disk") {
            let suffix = &name["disk".len()..];
            let digits: String = suffix.chars().take_while(|c| c.is_ascii_digit()).collect();
            if digits.is_empty() {
                None
            } else {
                Some(format!("disk{digits}"))
            }
        } else {
            None
        };
        if let Some(disk) = disk {
            disks.insert(disk);
        }
    }

    let mut children: io_iterator_t = 0;
    let result = unsafe {
        IORegistryEntryGetChildIterator(entry, kIOServicePlane as *mut c_char, &mut children)
    };
    if result != 0 {
        return;
    }
    let _children_guard = IoObjectGuard(children as io_object_t);

    loop {
        let child = unsafe { IOIteratorNext(children) };
        if child == 0 {
            break;
        }
        let _child_guard = IoObjectGuard(child);
        collect_bsd_disks(child as io_registry_entry_t, disks);
    }
}

fn registry_u16_property(entry: io_registry_entry_t, key: &[u8]) -> Option<u16> {
    let value = registry_property(entry, key)?;
    let _value_guard = CfTypeGuard(value);

    let type_id = unsafe { CFGetTypeID(value) };
    if type_id == unsafe { CFNumberGetTypeID() } {
        let mut parsed: i32 = 0;
        let ok = unsafe {
            CFNumberGetValue(
                value as CFNumberRef,
                kCFNumberSInt32Type,
                &mut parsed as *mut i32 as *mut c_void,
            )
        };
        return if ok { u16::try_from(parsed).ok() } else { None };
    }

    if type_id == unsafe { CFStringGetTypeID() } {
        let parsed = cfstring_to_string(value as CFStringRef)?;
        return parsed.parse::<u16>().ok();
    }

    None
}

fn registry_string_property(entry: io_registry_entry_t, key: &[u8]) -> Option<String> {
    let value = registry_property(entry, key)?;
    let _value_guard = CfTypeGuard(value);
    if unsafe { CFGetTypeID(value) } != unsafe { CFStringGetTypeID() } {
        return None;
    }
    cfstring_to_string(value as CFStringRef)
}

fn registry_property(entry: io_registry_entry_t, key: &[u8]) -> Option<CFTypeRef> {
    let key_ref = CFSTR(key.as_ptr() as *const c_char);
    let value = unsafe { IORegistryEntryCreateCFProperty(entry, key_ref, kCFAllocatorDefault, 0) };
    if value.is_null() { None } else { Some(value) }
}

fn cfstring_to_string(value: CFStringRef) -> Option<String> {
    let mut buf = vec![0 as c_char; 1024];
    let ok = unsafe {
        CFStringGetCString(
            value,
            buf.as_mut_ptr(),
            buf.len() as isize,
            kCFStringEncodingUTF8,
        )
    };
    if ok == 0 {
        return None;
    }

    let cstr = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) };
    cstr.to_str().ok().map(ToOwned::to_owned)
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

struct IoObjectGuard(io_object_t);

impl Drop for IoObjectGuard {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe {
                IOObjectRelease(self.0);
            }
        }
    }
}

struct CfTypeGuard(CFTypeRef);

impl Drop for CfTypeGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                CFRelease(self.0);
            }
        }
    }
}
