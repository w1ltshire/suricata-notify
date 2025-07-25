use std::collections::HashMap;
use tokio::sync::broadcast;

use crate::types::EveEvent;

use crate::backends::{AlertBackend, DummyBackend};

type BackendFactory = fn(broadcast::Sender<EveEvent>) -> Box<dyn AlertBackend>;

pub fn get_backend_registry() -> HashMap<&'static str, BackendFactory> {
    let mut registry: HashMap<&str, BackendFactory> = HashMap::new();
    registry.insert("dummy", |tx| Box::new(DummyBackend::new(tx)));
    // Add more backends here
    registry
}
