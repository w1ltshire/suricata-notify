use crate::backends::AlertBackend;
use backends::dummy::DummyBackend;
use std::fs::File;
use tokio::sync::broadcast;
use types::EveEvent;
use watcher::async_watch;

mod backends;
mod parser;
mod types;
mod watcher;

use std::sync::atomic::{AtomicU64, Ordering};

static FILE_SIZE: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() {
    env_logger::init();

    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    log::info!("watching {}", path);

    let file = File::open(&path).unwrap();
    let size = file.metadata().unwrap().len();
    log::debug!("real file size: {size}");

    FILE_SIZE.store(size, Ordering::SeqCst);

    std::mem::drop(file);
    log::debug!("closed file");

    log::debug!("creating broadcast channel");
    let (tx, _) = broadcast::channel::<EveEvent>(100); // buffer size 100

    let mut dummy_backend = DummyBackend::new(tx.clone());
    tokio::spawn(async move {
        dummy_backend.run().await;
    });

    log::debug!("starting async watcher");
    futures::executor::block_on(async {
        if let Err(e) = async_watch(path, tx).await {
            log::error!("{:?}", e)
        }
    });
}
