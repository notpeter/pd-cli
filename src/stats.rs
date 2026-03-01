use crate::device::Device;
use serde_json::{Map, Number, Value};

pub(crate) fn fetch_stats(device: &Device) -> Result<(String, Vec<(String, String)>), String> {
    let Some(port) = device.port() else {
        return Err(format!(
            "device '{}' has no serial port available; reconnect in serial mode and try again",
            device.serial()
        ));
    };

    let payload = port.send_serial_command_and_capture("stats")?;
    let raw = String::from_utf8_lossy(&payload).to_string();
    let mut entries = parse_stats_entries(&raw)
        .into_iter()
        .map(|(k, v)| normalize_stat(&k, &v))
        .collect::<Vec<_>>();

    for command in ["vbat", "batpct", "temp", "gettime"] {
        let raw_value = query_metric(port, command).unwrap_or_else(|| "unavailable".to_string());
        for (key, value) in metric_output_entries(command, &raw_value) {
            entries.push((key, value));
        }
    }

    Ok((device.serial().to_string(), entries))
}

fn query_metric(port: &crate::platform::SerialPortPath, command: &str) -> Option<String> {
    let payload = port.send_serial_command_and_capture(command).ok()?;
    let raw = String::from_utf8_lossy(&payload);
    parse_metric_value(command, &raw)
}

pub(crate) fn parse_stats_entries(raw: &str) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    let mut in_stats_block = false;

    for line in raw.replace('\r', "\n").lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if !in_stats_block {
            if line.contains("~stats:") {
                in_stats_block = true;
            }
            continue;
        }
        if line.starts_with('~') {
            break;
        }

        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() {
            continue;
        }
        entries.push((key.to_string(), value.to_string()));
    }

    entries
}

pub(crate) fn normalize_stat(key: &str, value: &str) -> (String, String) {
    let key_lc = key.to_ascii_lowercase();
    match key_lc.as_str() {
        "frame count" => ("frame_count".to_string(), value.to_string()),
        "frame time" => ("frame_time".to_string(), value.to_string()),
        "gc time" => ("lua_gc_secs".to_string(), value.to_string()),
        "disp time" => ("disp_time".to_string(), value.to_string()),
        "current time" => ("time_epoch".to_string(), value.to_string()),
        "mem alloced" => ("memory_alloced_bytes".to_string(), value.to_string()),
        "mem reserved" => ("memory_reserved_bytes".to_string(), value.to_string()),
        "mem total" => ("memory_total_bytes".to_string(), value.to_string()),
        "kernel" | "serial" | "game" | "gc" | "wifi" | "trace" | "audio" => {
            let out_key = format!("cpu_{key_lc}_percent");
            let out_value = extract_first_float(value)
                .map(|f| format!("{f:.2}"))
                .unwrap_or_else(|| value.to_string());
            (out_key, out_value)
        }
        _ => (key.to_string(), value.to_string()),
    }
}

pub(crate) fn parse_metric_value(command: &str, raw: &str) -> Option<String> {
    let normalized = raw.replace('\r', "\n");
    let header = format!("~{command}:");
    let command_prefix = format!("{command}:");
    let mut after_header = false;
    let mut saw_command_echo = false;
    let command_lc = command.to_ascii_lowercase();

    for line in normalized.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let line_lc = line.to_ascii_lowercase();

        if line.contains(&header) {
            after_header = true;
            if let Some((_, rhs)) = line.split_once(':') {
                let value = rhs.trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
            continue;
        }

        if line_lc == command_lc {
            saw_command_echo = true;
            continue;
        }

        if line_lc.starts_with(&command_prefix) {
            if let Some((_, rhs)) = line.split_once(':') {
                let value = rhs.trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }

        if let Some((lhs, rhs)) = line.split_once('=') {
            let lhs_lc = lhs.trim().to_ascii_lowercase();
            if lhs_lc.contains(&command_lc)
                || command_lc == "vbat"
                || command_lc == "batpct"
                || command_lc == "temp"
            {
                let value = rhs.trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }

        if after_header {
            return Some(line.to_string());
        }

        if saw_command_echo {
            return Some(line.to_string());
        }
    }

    None
}

pub(crate) fn normalize_metric_value(command: &str, value: &str) -> String {
    match command {
        "vbat" => extract_first_float(value).map(|n| n.to_string()),
        "batpct" => extract_first_float(value).map(|n| format!("{n:.2}")),
        "temp" => extract_first_int(value).map(|n| n.to_string()),
        _ => None,
    }
    .unwrap_or_else(|| value.to_string())
}

pub(crate) fn metric_output_entries(command: &str, value: &str) -> Vec<(String, String)> {
    match command {
        "vbat" => vec![(
            "battery_volts".to_string(),
            normalize_metric_value("vbat", value),
        )],
        "batpct" => vec![(
            "battery_percent".to_string(),
            normalize_metric_value("batpct", value),
        )],
        "temp" => vec![(
            "temp_celsius".to_string(),
            normalize_metric_value("temp", value),
        )],
        "gettime" => {
            let (utc, weekday) = split_time_fields(value);
            vec![
                ("time_utc".to_string(), utc),
                ("time_weekday".to_string(), weekday),
            ]
        }
        _ => vec![(command.to_string(), value.to_string())],
    }
}

pub(crate) fn split_time_fields(value: &str) -> (String, String) {
    let trimmed = value.trim();
    if let Some((utc, rest)) = trimmed.split_once(" (") {
        let weekday = rest.trim_end_matches(')').trim();
        let utc = utc.trim();
        let weekday = if weekday.is_empty() {
            "unavailable"
        } else {
            weekday
        };
        return (utc.to_string(), weekday.to_string());
    }

    (trimmed.to_string(), "unavailable".to_string())
}

pub(crate) fn print_stats_json(entries: &[(String, String)]) {
    let mut object = Map::with_capacity(entries.len());
    for (key, value) in entries {
        object.insert(key.clone(), infer_json_value(key, value));
    }

    let json = serde_json::to_string_pretty(&Value::Object(object))
        .expect("serializing stats map should not fail");
    println!("{json}");
}

fn infer_json_value(key: &str, value: &str) -> Value {
    if key.starts_with("cpu_") && key.ends_with("_percent") {
        if let Ok(parsed) = value.parse::<f64>() {
            if parsed.is_finite() {
                let rounded = (parsed * 10.0).round() / 10.0;
                if let Some(number) = Number::from_f64(rounded) {
                    return Value::Number(number);
                }
            }
        }
    }

    if key == "battery_percent" {
        if let Ok(parsed) = value.parse::<f64>() {
            if parsed.is_finite() {
                let rounded = (parsed * 100.0).round() / 100.0;
                if let Some(number) = Number::from_f64(rounded) {
                    return Value::Number(number);
                }
            }
        }
    }

    if let Ok(parsed) = value.parse::<i64>() {
        return Value::Number(parsed.into());
    }
    if let Ok(parsed) = value.parse::<f64>() {
        if parsed.is_finite() {
            if let Some(number) = Number::from_f64(parsed) {
                return Value::Number(number);
            }
        }
    }

    Value::String(value.to_string())
}

fn extract_first_float(s: &str) -> Option<f64> {
    s.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
        .filter(|token| !token.is_empty() && *token != "-" && *token != "." && *token != "-.")
        .find_map(|token| token.parse::<f64>().ok())
}

fn extract_first_int(s: &str) -> Option<i64> {
    s.split(|c: char| !(c.is_ascii_digit() || c == '-'))
        .filter(|token| !token.is_empty() && *token != "-")
        .find_map(|token| token.parse::<i64>().ok())
}

#[cfg(test)]
mod tests {
    use super::{
        normalize_metric_value, normalize_stat, parse_metric_value, parse_stats_entries,
        split_time_fields,
    };

    #[test]
    fn parses_stats_payload_entries() {
        let raw = "stats\r\n~stats:\nframe count: 194503\nframe time: 0.000977\nkernel: 0.1%\n";
        let entries = parse_stats_entries(raw);
        assert_eq!(
            entries,
            vec![
                ("frame count".to_string(), "194503".to_string()),
                ("frame time".to_string(), "0.000977".to_string()),
                ("kernel".to_string(), "0.1%".to_string()),
            ]
        );
    }

    #[test]
    fn normalizes_stats_current_time_and_cpu_fields() {
        assert_eq!(
            normalize_stat("frame count", "119424"),
            ("frame_count".to_string(), "119424".to_string())
        );
        assert_eq!(
            normalize_stat("frame time", "0.005371"),
            ("frame_time".to_string(), "0.005371".to_string())
        );
        assert_eq!(
            normalize_stat("gc time", "0.004395"),
            ("lua_gc_secs".to_string(), "0.004395".to_string())
        );
        assert_eq!(
            normalize_stat("disp time", "13"),
            ("disp_time".to_string(), "13".to_string())
        );
        assert_eq!(
            normalize_stat("current time", "5698432"),
            ("time_epoch".to_string(), "5698432".to_string())
        );
        assert_eq!(
            normalize_stat("mem alloced", "1523196"),
            ("memory_alloced_bytes".to_string(), "1523196".to_string())
        );
        assert_eq!(
            normalize_stat("mem reserved", "1849152"),
            ("memory_reserved_bytes".to_string(), "1849152".to_string())
        );
        assert_eq!(
            normalize_stat("mem total", "16514612"),
            ("memory_total_bytes".to_string(), "16514612".to_string())
        );
        assert_eq!(
            normalize_stat("GC", "9.4%"),
            ("cpu_gc_percent".to_string(), "9.40".to_string())
        );
        assert_eq!(
            normalize_stat("audio", "0.4%"),
            ("cpu_audio_percent".to_string(), "0.40".to_string())
        );
    }

    #[test]
    fn parses_metric_value_with_header_line() {
        let raw = "vbat\r\n~vbat:\n4.12\r\n";
        let value = parse_metric_value("vbat", raw).expect("vbat value");
        assert_eq!(value, "4.12");
    }

    #[test]
    fn parses_metric_value_inline() {
        let raw = "batpct: 96%\r\n";
        let value = parse_metric_value("batpct", raw).expect("batpct value");
        assert_eq!(value, "96%");
    }

    #[test]
    fn parses_metric_value_equals_style() {
        let raw = "vbat\r\nVBAT=4.202000\r\n";
        let value = parse_metric_value("vbat", raw).expect("vbat equals value");
        assert_eq!(value, "4.202000");
    }

    #[test]
    fn parses_metric_value_after_command_echo() {
        let raw = "gettime\r\n2026-03-01T03:02:02.534Z (Sunday)\r\n";
        let value = parse_metric_value("gettime", raw).expect("gettime value");
        assert_eq!(value, "2026-03-01T03:02:02.534Z (Sunday)");
    }

    #[test]
    fn normalizes_battery_and_temp_values() {
        assert_eq!(normalize_metric_value("vbat", "VBAT=4.202000"), "4.202");
        assert_eq!(
            normalize_metric_value("batpct", "PCT=100.00% RAWPCT=100.00%"),
            "100.00"
        );
        assert_eq!(normalize_metric_value("temp", "TEMP=20 C"), "20");
    }

    #[test]
    fn splits_time_into_utc_and_weekday() {
        let (utc, weekday) = split_time_fields("2026-03-01T03:08:40.152Z (Sunday)");
        assert_eq!(utc, "2026-03-01T03:08:40.152Z");
        assert_eq!(weekday, "Sunday");
    }
}
