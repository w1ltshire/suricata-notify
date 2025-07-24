use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EveEvent {
    pub event_type: Option<String>, // Optional in case some entries don't have it
    pub timestamp: Option<String>,
    pub src_ip: Option<String>,
    pub dest_ip: Option<String>,
    pub src_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub proto: Option<String>,
    pub app_proto: Option<String>,
}
