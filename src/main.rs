use backends::factory::get_backend_registry;
use clap::Parser;
use once_cell::sync::OnceCell;
use std::fs::File;
use tokio::sync::broadcast;
use types::EveEvent;
use watcher::async_watch;

mod backends;
mod config;
mod parser;
mod templates;
mod types;
mod watcher;

use std::sync::atomic::{AtomicU64, Ordering};

static FILE_SIZE: AtomicU64 = AtomicU64::new(0);
static TEMPLATE: OnceCell<String> = OnceCell::new();

/// A tool to send notifications from Suricata to anywhere
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Events file to watch
    #[arg(short, long)]
    event_file: Option<String>,

    /// Config file to use
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let config = config::init(
        &args
            .config
            .unwrap_or("/etc/suricata-notify.toml".to_string()),
    )
    .await
    .unwrap();

    log::debug!("config: {:?}", config);

    TEMPLATE.set(config.template).unwrap();

    let event_file = args.event_file.unwrap_or(config.event_file);

    log::debug!("creating broadcast channel");
    let (tx, _) = broadcast::channel::<EveEvent>(100);

    log::debug!("registering backends");
    config.backends.iter().for_each(|backend| {
        let registry = get_backend_registry();

        if let Some(factory) = registry.get(backend.0.as_str()) {
            if backend
                .1
                .settings
                .get("enabled")
                .unwrap_or(&"false".to_string())
                == "true"
            {
                log::debug!("registering backend {}", backend.0);

                let mut backend = factory(tx.clone(), backend.1.settings.clone());

                tokio::spawn(async move {
                    backend.run().await;
                });
            } else {
                log::info!("{} is disabled", backend.0);
            }
        } else {
            log::warn!("unknown backend: {}", backend.0);
        }
    });

    log::info!("watching {}", event_file);

    let file = File::open(&event_file).unwrap();
    let size = file.metadata().unwrap().len();
    log::debug!("real file size: {size}");

    FILE_SIZE.store(size, Ordering::SeqCst);

    std::mem::drop(file);
    log::debug!("closed file");

    log::debug!("starting async watcher");
    if let Err(e) = async_watch(event_file, tx, config.max_severity).await {
        log::error!("{:?}", e)
    }
}
