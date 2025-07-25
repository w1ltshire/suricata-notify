use std::collections::HashMap;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub event_file: String,
    pub backends: HashMap<String, AlertBackendConfig>,
}

#[derive(Debug, Deserialize)]
pub struct AlertBackendConfig {}

pub async fn init(path: &str) -> anyhow::Result<AppConfig> {
    let settings = Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    Ok(settings.try_deserialize::<AppConfig>().unwrap())
}
