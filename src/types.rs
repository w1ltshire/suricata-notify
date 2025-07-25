use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EveEvent {
    pub timestamp: String,
    pub flow_id: Option<u64>,
    pub in_iface: Option<String>,
    pub event_type: Option<String>,
    pub src_ip: Option<String>,
    pub src_port: Option<u16>,
    pub dest_ip: Option<String>,
    pub dest_port: Option<u16>,
    pub proto: Option<String>,
    pub pkt_src: Option<String>,
    pub community_id: Option<String>,
    pub alert: Option<Alert>,
    pub app_proto: Option<String>,
    pub direction: Option<String>,
    pub flow: Option<Flow>,
    pub payload: Option<String>,
    pub payload_printable: Option<String>,
    pub stream: Option<u32>,
    pub packet: Option<String>,
    pub packet_info: Option<PacketInfo>,
    pub capture_file: Option<String>,
    pub host: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Alert {
    pub action: Option<String>,
    pub gid: Option<u32>,
    pub signature_id: Option<u32>,
    pub rev: Option<u32>,
    pub signature: Option<String>,
    pub category: Option<String>,
    pub severity: Option<u8>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Flow {
    pub pkts_toserver: Option<u64>,
    pub pkts_toclient: Option<u64>,
    pub bytes_toserver: Option<u64>,
    pub bytes_toclient: Option<u64>,
    pub start: Option<String>,
    pub src_ip: Option<String>,
    pub dest_ip: Option<String>,
    pub src_port: Option<u16>,
    pub dest_port: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PacketInfo {
    pub linktype: Option<u8>,
}
