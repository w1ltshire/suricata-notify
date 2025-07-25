use std::collections::HashMap;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub event_file: String,
}

pub async fn init(path: &str) -> anyhow::Result<AppConfig> {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(path))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    Ok(settings.try_deserialize::<AppConfig>().unwrap())
}
