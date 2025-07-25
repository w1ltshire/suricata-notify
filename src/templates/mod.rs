use crate::types::EveEvent;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
pub struct AlertTemplate<'a> {
    timestamp: &'a str,
    flow_id: String,
    in_iface: &'a str,
    event_type: &'a str,
    src_ip: &'a str,
    src_port: String,
    dest_ip: &'a str,
    dest_port: String,
    proto: &'a str,
    pkt_src: &'a str,
    community_id: &'a str,
    app_proto: &'a str,
    direction: &'a str,
    payload: &'a str,
    payload_printable: &'a str,
    stream: String,
    packet: &'a str,
    capture_file: &'a str,
    host: &'a str,

    // Alert fields
    alert_action: &'a str,
    alert_gid: String,
    alert_signature_id: String,
    alert_rev: String,
    alert_signature: &'a str,
    alert_category: &'a str,
    alert_severity: String,

    // Flow fields
    flow_pkts_toserver: String,
    flow_pkts_toclient: String,
    flow_bytes_toserver: String,
    flow_bytes_toclient: String,
    flow_start: &'a str,
    flow_src_ip: &'a str,
    flow_dest_ip: &'a str,
    flow_src_port: String,
    flow_dest_port: String,

    // Packet info
    packet_linktype: String,
}

pub fn render_alert(event: &EveEvent, template: &str) -> Result<String, handlebars::RenderError> {
    let data = AlertTemplate {
        timestamp: &event.timestamp,
        flow_id: event.flow_id.map_or("".into(), |v| v.to_string()),
        in_iface: event.in_iface.as_deref().unwrap_or(""),
        event_type: event.event_type.as_deref().unwrap_or(""),
        src_ip: event.src_ip.as_deref().unwrap_or(""),
        src_port: event.src_port.map_or("".into(), |v| v.to_string()),
        dest_ip: event.dest_ip.as_deref().unwrap_or(""),
        dest_port: event.dest_port.map_or("".into(), |v| v.to_string()),
        proto: event.proto.as_deref().unwrap_or(""),
        pkt_src: event.pkt_src.as_deref().unwrap_or(""),
        community_id: event.community_id.as_deref().unwrap_or(""),
        app_proto: event.app_proto.as_deref().unwrap_or(""),
        direction: event.direction.as_deref().unwrap_or(""),
        payload: event.payload.as_deref().unwrap_or(""),
        payload_printable: event.payload_printable.as_deref().unwrap_or(""),
        stream: event.stream.map_or("".into(), |v| v.to_string()),
        packet: event.packet.as_deref().unwrap_or(""),
        capture_file: event.capture_file.as_deref().unwrap_or(""),
        host: event.host.as_deref().unwrap_or(""),

        alert_action: event
            .alert
            .as_ref()
            .and_then(|a| a.action.as_deref())
            .unwrap_or(""),
        alert_gid: event
            .alert
            .as_ref()
            .and_then(|a| a.gid)
            .map_or("".into(), |v| v.to_string()),
        alert_signature_id: event
            .alert
            .as_ref()
            .and_then(|a| a.signature_id)
            .map_or("".into(), |v| v.to_string()),
        alert_rev: event
            .alert
            .as_ref()
            .and_then(|a| a.rev)
            .map_or("".into(), |v| v.to_string()),
        alert_signature: event
            .alert
            .as_ref()
            .and_then(|a| a.signature.as_deref())
            .unwrap_or(""),
        alert_category: event
            .alert
            .as_ref()
            .and_then(|a| a.category.as_deref())
            .unwrap_or(""),
        alert_severity: event
            .alert
            .as_ref()
            .and_then(|a| a.severity)
            .map_or("".into(), |v| v.to_string()),

        flow_pkts_toserver: event
            .flow
            .as_ref()
            .and_then(|f| f.pkts_toserver)
            .map_or("".into(), |v| v.to_string()),
        flow_pkts_toclient: event
            .flow
            .as_ref()
            .and_then(|f| f.pkts_toclient)
            .map_or("".into(), |v| v.to_string()),
        flow_bytes_toserver: event
            .flow
            .as_ref()
            .and_then(|f| f.bytes_toserver)
            .map_or("".into(), |v| v.to_string()),
        flow_bytes_toclient: event
            .flow
            .as_ref()
            .and_then(|f| f.bytes_toclient)
            .map_or("".into(), |v| v.to_string()),
        flow_start: event
            .flow
            .as_ref()
            .and_then(|f| f.start.as_deref())
            .unwrap_or(""),
        flow_src_ip: event
            .flow
            .as_ref()
            .and_then(|f| f.src_ip.as_deref())
            .unwrap_or(""),
        flow_dest_ip: event
            .flow
            .as_ref()
            .and_then(|f| f.dest_ip.as_deref())
            .unwrap_or(""),
        flow_src_port: event
            .flow
            .as_ref()
            .and_then(|f| f.src_port)
            .map_or("".into(), |v| v.to_string()),
        flow_dest_port: event
            .flow
            .as_ref()
            .and_then(|f| f.dest_port)
            .map_or("".into(), |v| v.to_string()),

        packet_linktype: event
            .packet_info
            .as_ref()
            .and_then(|p| p.linktype)
            .map_or("".into(), |v| v.to_string()),
    };

    let handlebars = Handlebars::new();
    handlebars.render_template(template, &data)
}
