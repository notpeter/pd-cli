use image::{DynamicImage, GrayImage, ImageBuffer, ImageFormat, Luma};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::device::{list_devices, resolve_device};
use crate::platform::send_serial_command_and_capture;

const SCREEN_PREFIX: &[u8] = b"screen\r\n~screen:\n";
const SCREEN_PREFIX_LEGACY: &[u8] = b"\r\nscreen~:\n";
const SCREEN_BITMAP_BYTES: usize = 12_000;
const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 240;

pub(crate) fn capture_screenshot(
    device_id: Option<&str>,
    filename: Option<&str>,
) -> Result<(String, String, usize, String), String> {
    let devices = list_devices()?;
    let device = resolve_device(devices, device_id)?;

    if device.port.is_empty() {
        return Err(format!(
            "device '{}' has no serial port available; reconnect in serial mode and try again",
            device.device
        ));
    }

    let payload = send_serial_command_and_capture(&device.port, "screen")?;
    let path = filename
        .map(ToOwned::to_owned)
        .unwrap_or_else(default_screenshot_filename);
    write_screenshot_file(&path, &payload)?;

    let inspect = inspect_screen_payload(&payload, &path);
    Ok((device.device, path, payload.len(), inspect))
}

fn write_screenshot_file(path: &str, payload: &[u8]) -> Result<(), String> {
    match screenshot_format_for_path(path)? {
        Some(ImageFormat::Png) => {
            let bitmap = extract_screen_bitmap(payload)?;
            let image = bitmap_to_image(bitmap);
            DynamicImage::ImageLuma8(image)
                .save_with_format(path, ImageFormat::Png)
                .map_err(|e| format!("failed to write screenshot image '{path}': {e}"))?;
            Ok(())
        }
        Some(ImageFormat::Gif) => {
            let bitmap = extract_screen_bitmap(payload)?;
            let image = bitmap_to_image(bitmap);
            DynamicImage::ImageLuma8(image)
                .into_rgb8()
                .save_with_format(path, ImageFormat::Gif)
                .map_err(|e| format!("failed to write screenshot image '{path}': {e}"))?;
            Ok(())
        }
        Some(other) => Err(format!("unsupported image output format: {other:?}")),
        None => std::fs::write(path, payload)
            .map_err(|e| format!("failed to write screenshot file '{path}': {e}")),
    }
}

fn screenshot_format_for_path(path: &str) -> Result<Option<ImageFormat>, String> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase());

    match ext.as_deref() {
        Some("png") => Ok(Some(ImageFormat::Png)),
        Some("gif") => Ok(Some(ImageFormat::Gif)),
        Some("raw") | Some("bin") | None => Ok(None),
        _ => Err(format!(
            "unsupported screenshot extension; use .png, .gif, .raw, or .bin"
        )),
    }
}

fn extract_screen_bitmap(payload: &[u8]) -> Result<&[u8], String> {
    if let Some(offset) = find_subslice(payload, SCREEN_PREFIX) {
        let start = offset + SCREEN_PREFIX.len();
        let end = start + SCREEN_BITMAP_BYTES;
        if payload.len() < end {
            return Err(format!(
                "screen payload is incomplete: got {} bytes, expected at least {}",
                payload.len().saturating_sub(start),
                SCREEN_BITMAP_BYTES
            ));
        }
        return Ok(&payload[start..end]);
    }

    if let Some(offset) = find_subslice(payload, SCREEN_PREFIX_LEGACY) {
        let start = offset + SCREEN_PREFIX_LEGACY.len();
        let end = start + SCREEN_BITMAP_BYTES;
        if payload.len() < end {
            return Err(format!(
                "legacy screen payload is incomplete: got {} bytes, expected at least {}",
                payload.len().saturating_sub(start),
                SCREEN_BITMAP_BYTES
            ));
        }
        return Ok(&payload[start..end]);
    }

    if payload.len() >= SCREEN_BITMAP_BYTES {
        return Ok(&payload[..SCREEN_BITMAP_BYTES]);
    }

    Err("no recognizable screen payload in serial output".to_string())
}

fn bitmap_to_image(bitmap: &[u8]) -> GrayImage {
    let mut img: GrayImage = ImageBuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let stride = (SCREEN_WIDTH / 8) as usize;

    for y in 0..SCREEN_HEIGHT as usize {
        let row = &bitmap[y * stride..(y + 1) * stride];
        for x in 0..SCREEN_WIDTH as usize {
            let byte = row[x / 8];
            let bit = 7 - (x % 8);
            let is_white = ((byte >> bit) & 1) == 1;
            let value = if is_white { 255 } else { 0 };
            img.put_pixel(x as u32, y as u32, Luma([value]));
        }
    }

    img
}

fn default_screenshot_filename() -> String {
    format!("playdate_{}.gif", timestamp_now())
}

fn timestamp_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    secs.to_string()
}

fn inspect_screen_payload(payload: &[u8], path: &str) -> String {
    if let Some(offset) = find_subslice(payload, SCREEN_PREFIX)
        .or_else(|| find_subslice(payload, SCREEN_PREFIX_LEGACY))
    {
        let image_start = offset + SCREEN_PREFIX.len();
        let remaining = payload.len().saturating_sub(image_start);
        if remaining >= SCREEN_BITMAP_BYTES {
            return format!(
                "detected screen header at byte {offset}; bitmap payload appears complete ({remaining} bytes after header), wrote {}",
                screenshot_kind_for_path(path)
            );
        }

        return format!(
            "detected screen header at byte {offset}; bitmap payload appears partial ({remaining}/{SCREEN_BITMAP_BYTES} bytes after header), wrote {}",
            screenshot_kind_for_path(path)
        );
    }

    let preview_len = payload.len().min(32);
    let preview = payload[..preview_len]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "no known screen header found; first {preview_len} bytes (hex): {preview}; wrote {}",
        screenshot_kind_for_path(path)
    )
}

fn screenshot_kind_for_path(path: &str) -> &'static str {
    match screenshot_format_for_path(path) {
        Ok(Some(ImageFormat::Png)) => "PNG image",
        Ok(Some(ImageFormat::Gif)) => "GIF image",
        _ => "raw serial bytes",
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }

    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::{SCREEN_BITMAP_BYTES, SCREEN_PREFIX, extract_screen_bitmap};

    #[test]
    fn has_expected_screen_prefix_signature() {
        assert_eq!(SCREEN_PREFIX, b"screen\r\n~screen:\n");
    }

    #[test]
    fn extracts_bitmap_after_screen_prefix() {
        let mut payload = Vec::new();
        payload.extend_from_slice(SCREEN_PREFIX);
        payload.extend_from_slice(&vec![0xAA; SCREEN_BITMAP_BYTES]);
        payload.extend_from_slice(b"\r\n");

        let bitmap = extract_screen_bitmap(&payload).expect("bitmap extraction");
        assert_eq!(bitmap.len(), SCREEN_BITMAP_BYTES);
        assert_eq!(bitmap[0], 0xAA);
    }
}
