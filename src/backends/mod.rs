use tokio::sync::broadcast::Receiver;

use crate::types::EveEvent;

pub mod dummy;

// # Define the trait that backends must implement
#[async_trait::async_trait]
pub trait AlertBackend {
    async fn run(&mut self);

    async fn send_alert(
        &self,
        alert: EveEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
