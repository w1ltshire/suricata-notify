use futures::{
    SinkExt, StreamExt,
    channel::mpsc::{Receiver, channel},
};
use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event::DataChange,
};
use std::{path::Path, time::Duration};
use tokio::sync::broadcast::Sender;

use crate::{parser::parse_json, types::EveEvent};

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let config = Config::default()
        .with_poll_interval(Duration::from_secs(2))
        .with_compare_contents(true);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        config,
    )?;

    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(
    path: P,
    tx: Sender<EveEvent>,
    max_severity: u8,
) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                if let EventKind::Modify(notify::event::ModifyKind::Data(DataChange::Any)) =
                    event.kind
                {
                    log::debug!(
                        "received a file change event: {:?}, sending to parser",
                        event
                    );
                    parse_json(
                        event.paths.first().unwrap().to_str().unwrap(),
                        tx.clone(),
                        max_severity,
                    )
                    .await; // `first().unwrap()` should be safe because we know there is at least one path
                }
            }
            Err(e) => log::error!("watch error: {:?}", e),
        }
    }

    Ok(())
}
