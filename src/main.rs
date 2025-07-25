use backends::factory::get_backend_registry;
use clap::Parser;
use std::fs::File;
use tokio::sync::broadcast;
use types::EveEvent;
use watcher::async_watch;

mod backends;
mod config;
mod parser;
mod types;
mod watcher;

use std::sync::atomic::{AtomicU64, Ordering};

static FILE_SIZE: AtomicU64 = AtomicU64::new(0);

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

    let event_file = args.event_file.unwrap_or(config.event_file);

    let (tx, _) = broadcast::channel::<EveEvent>(100);

    config.backends.iter().for_each(|backend| {
        let registry = get_backend_registry();

        if let Some(factory) = registry.get(backend.0.as_str()) {
            log::debug!("registering backend {}", backend.0);

            let mut backend = factory(tx.clone());

            // You can now store `backend` in a vector or spawn its run
            tokio::spawn(async move {
                backend.run().await;
            });
        } else {
            log::warn!("Unknown backend: {}", backend.0);
        }
    });

    log::info!("watching {}", event_file);

    let file = File::open(&event_file).unwrap();
    let size = file.metadata().unwrap().len();
    log::debug!("real file size: {size}");

    FILE_SIZE.store(size, Ordering::SeqCst);

    std::mem::drop(file);
    log::debug!("closed file");

    log::debug!("creating broadcast channel");
    //    let (tx, _) = broadcast::channel::<EveEvent>(100); // buffer size 100

    log::debug!("starting async watcher");
    if let Err(e) = async_watch(event_file, tx).await {
        log::error!("{:?}", e)
    }
}
