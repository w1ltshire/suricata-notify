use std::fs::File;

use tokio::sync::broadcast::Sender;

use crate::FILE_SIZE;
use crate::types::EveEvent;
use std::io::BufRead;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::Path;

fn get_reader_from_position<P: AsRef<Path>>(path: P, pos: u64) -> std::io::Result<BufReader<File>> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(pos))?;
    Ok(BufReader::new(file))
}

fn parse_and_filter_alerts<R: BufRead>(reader: R, tx: Sender<EveEvent>, max_severity: u8) {
    for line in reader.lines().map_while(Result::ok) {
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<EveEvent>(&line) {
            Ok(event) if event.event_type.as_deref() == Some("alert") => {
                if event.alert.clone().unwrap().severity.unwrap_or(3) <= max_severity {
                    log::debug!("alert severity is less than or equal than max_severity, sending");
                    // Unwrap is safe here because we know the alert is Some(Alert)
                    if let Err(e) = tx.send(event) {
                        log::warn!("failed to send alert, is anyone listening?: {:?}", e);
                    }
                }
            }
            Ok(_) => {
                log::debug!("not an alert, ignoring");
            }
            Err(e) => {
                eprintln!("Failed to deserialize event: {}", e);
            }
        }
    }
}

pub async fn parse_json(file_path: &str, tx: Sender<EveEvent>, max_severity: u8) {
    log::debug!("parsing {file_path}");

    let file = File::open(file_path).unwrap();
    log::debug!("opened file {:?}", file);

    let new_size = file.metadata().unwrap().len();
    let old_size = FILE_SIZE.load(std::sync::atomic::Ordering::SeqCst);
    FILE_SIZE.store(new_size, std::sync::atomic::Ordering::SeqCst);

    log::debug!("file size change: {old_size} -> {new_size}");

    parse_and_filter_alerts(
        get_reader_from_position(file_path, old_size).unwrap(),
        tx,
        max_severity,
    );

    std::mem::drop(file);
    log::debug!("done with parsing, closed file");
}
