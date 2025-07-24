use tokio::sync::broadcast::{Receiver, Sender, error::RecvError};

use crate::types::EveEvent;

use super::AlertBackend;

pub struct DummyBackend {
    pub receiver: Receiver<EveEvent>,
}

impl DummyBackend {
    pub fn new(tx: Sender<EveEvent>) -> Self {
        let receiver = tx.subscribe();

        Self { receiver }
    }
}

#[async_trait::async_trait]
impl AlertBackend for DummyBackend {
    async fn run(&mut self) {
        log::debug!("listening for alerts");
        loop {
            match self.receiver.recv().await {
                Ok(event) => {
                    println!("DUMMY BACKEND ALERT: {:?}", event);
                }
                Err(RecvError::Lagged(skipped)) => {
                    eprintln!("Missed {} messages", skipped);
                }
                Err(RecvError::Closed) => {
                    eprintln!("Channel closed");
                    break;
                }
            }
        }
    }

    async fn send_alert(
        &self,
        alert: EveEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Logging alert: {:?}", alert);
        Ok(())
    }
}
