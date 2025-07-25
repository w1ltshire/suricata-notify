use std::collections::HashMap;
use tokio::sync::broadcast;

use crate::types::EveEvent;

use crate::backends::{AlertBackend, DummyBackend};

use super::http::HttpBackend;

type BackendFactory =
    fn(broadcast::Sender<EveEvent>, HashMap<String, String>) -> Box<dyn AlertBackend>;

pub fn get_backend_registry() -> HashMap<&'static str, BackendFactory> {
    let mut registry: HashMap<&str, BackendFactory> = HashMap::new();
    registry.insert("dummy", |tx, _settings| Box::new(DummyBackend::new(tx)));
    registry.insert("http", |tx, settings| {
        Box::new(HttpBackend::new(tx, settings))
    });
    // Add more backends here
    registry
}
