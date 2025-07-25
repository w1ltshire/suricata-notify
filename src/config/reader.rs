use std::collections::HashMap;

pub struct SettingsReader<'a> {
    settings: &'a HashMap<String, String>,
}

impl<'a> SettingsReader<'a> {
    pub fn new(settings: &'a HashMap<String, String>) -> Self {
        Self { settings }
    }

    pub fn required(&self, key: &str) -> Result<String, String> {
        self.settings
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Missing required setting '{}'", key))
    }

    pub fn optional(&self, key: &str) -> Option<String> {
        self.settings.get(key).cloned()
    }

    pub fn parse_or<T: std::str::FromStr>(&self, key: &str, default: T) -> T {
        self.settings
            .get(key)
            .and_then(|s| s.parse::<T>().ok())
            .unwrap_or(default)
    }

    pub fn headers(&self) -> HashMap<String, String> {
        self.settings
            .iter()
            .filter_map(|(k, v)| {
                k.strip_prefix("header_")
                    .map(|hk| (hk.to_string(), v.to_string()))
            })
            .collect()
    }
}
