use crate::device::Device;
use serde_json::{Map, Value};

impl Device {
    pub(crate) fn fetch_version(&self) -> Result<Vec<(String, String)>, String> {
        let payload = self.send_command_and_capture("version")?;
        let raw = String::from_utf8_lossy(&payload).to_string();
        Ok(parse_version_entries(&raw))
    }
}

pub(crate) fn parse_version_entries(raw: &str) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    let mut saw_version_header = false;

    for line in raw.replace('\r', "\n").lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.eq_ignore_ascii_case("version") {
            continue;
        }

        if line.starts_with("~version:") {
            saw_version_header = true;
            if let Some((_, value)) = line.split_once(':') {
                let value = value.trim();
                if !value.is_empty() {
                    entries.push(("version".to_string(), value.to_string()));
                }
            }
            continue;
        }

        if saw_version_header && line.starts_with('~') {
            break;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = normalize_version_key(key);
            let value = value.trim();
            if !key.is_empty() && !value.is_empty() {
                entries.push((key, value.to_string()));
            }
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = normalize_version_key(key);
            let value = value.trim();
            if !key.is_empty() && !value.is_empty() {
                entries.push((key, value.to_string()));
            }
            continue;
        }

        if entries.is_empty() {
            entries.push(("version".to_string(), line.to_string()));
        }
    }

    entries
}

fn normalize_version_key(key: &str) -> String {
    key.trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

pub(crate) fn print_version_json(entries: &[(String, String)]) {
    let mut object = Map::with_capacity(entries.len());
    for (key, value) in entries {
        object.insert(key.clone(), Value::String(value.clone()));
    }
    let json = serde_json::to_string_pretty(&Value::Object(object))
        .expect("serializing version map should not fail");
    println!("{json}");
}

#[cfg(test)]
mod tests {
    use super::parse_version_entries;

    #[test]
    fn parses_version_key_value_lines() {
        let raw = "version\r\n~version:\nSDK Version: 2.6.2\nSystem Build=abc123\n";
        let entries = parse_version_entries(raw);
        assert_eq!(
            entries,
            vec![
                ("sdk_version".to_string(), "2.6.2".to_string()),
                ("system_build".to_string(), "abc123".to_string()),
            ]
        );
    }

    #[test]
    fn falls_back_to_single_version_value() {
        let raw = "version\r\n~version:\n2.6.2\n";
        let entries = parse_version_entries(raw);
        assert_eq!(entries, vec![("version".to_string(), "2.6.2".to_string())]);
    }
}
