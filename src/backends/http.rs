use std::{collections::HashMap, str::FromStr, time::Duration};

use reqwest::{Client, Method};
use tokio::sync::broadcast::{Receiver, Sender, error::RecvError};

use crate::{TEMPLATE, config::reader::SettingsReader, templates::render_alert, types::EveEvent};

use super::AlertBackend;

pub struct HttpBackend {
    pub receiver: Receiver<EveEvent>,
    pub endpoint: String,
    pub method: String,
    pub auth_token: Option<String>,
    pub timeout: u64,
    pub headers: HashMap<String, String>,
    client: Client,
}

impl HttpBackend {
    pub fn new(tx: Sender<EveEvent>, settings: HashMap<String, String>) -> Self {
        let receiver = tx.subscribe();
        let reader = SettingsReader::new(&settings);

        Self {
            receiver,
            endpoint: reader.required("endpoint").unwrap(),
            auth_token: reader.optional("auth_token"),
            method: reader.optional("method").unwrap_or_else(|| "POST".into()),
            timeout: reader.parse_or("timeout", 5),
            headers: reader.headers(),
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl AlertBackend for HttpBackend {
    async fn run(&mut self) {
        log::debug!("listening for alerts");
        loop {
            match self.receiver.recv().await {
                Ok(event) => {
                    log::debug!("received event, sending to {}", self.endpoint);

                    let mut req = self
                        .client
                        .request(
                            Method::from_str(&self.method).unwrap(),
                            self.endpoint.clone(),
                        )
                        .timeout(Duration::from_secs(self.timeout))
                        .body(render_alert(&event, TEMPLATE.get().unwrap()).unwrap());

                    // Add headers
                    for (k, v) in &self.headers {
                        req = req.header(k, v);
                    }

                    // Optional bearer token
                    if let Some(token) = &self.auth_token {
                        req = req.bearer_auth(token);
                    }

                    let resp = req.send().await.unwrap();
                    if !resp.status().is_success() {
                        log::warn!(
                            "HTTP alert sent, but got non-2xx response: {}",
                            resp.status()
                        );
                    }
                }
                Err(RecvError::Lagged(skipped)) => {
                    log::warn!("missed {skipped} messages");
                }
                Err(RecvError::Closed) => {
                    log::error!("Channel closed");
                    break;
                }
            }
        }
    }
}
