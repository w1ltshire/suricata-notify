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
    let mut buffer = String::new();

    for line in reader.lines().map_while(Result::ok) {
        buffer.push_str(&line);

        match serde_json::from_str::<EveEvent>(&buffer) {
            Ok(event) => {
                buffer.clear(); // reset for next event
                if event.event_type.as_deref() == Some("alert") {
                    if event.alert.clone().unwrap().severity.unwrap_or(3) <= max_severity {
                        if let Err(e) = tx.send(event) {
                            log::warn!("failed to send alert, is anyone listening?: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                // If error is EOF or incomplete JSON, continue reading lines
                // Otherwise print error and reset buffer to avoid infinite loop

                if e.is_eof() {
                    // keep reading more lines until JSON completes
                    continue;
                } else {
                    eprintln!("Failed to deserialize event: {}", e);
                    buffer.clear(); // discard bad data
                }
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
