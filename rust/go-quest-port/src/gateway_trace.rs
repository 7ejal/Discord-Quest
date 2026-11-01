//! Offline parser for *saved, synthetic* Discord Gateway JSON frames.
//! No WebSocket, user-token Identify, authentication, or heartbeat is sent.
use crate::MAX_JSON_BYTES;
use serde::Serialize;
use serde_json::Value;

pub const MAX_GATEWAY_FRAMES: usize = 10_000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GatewayTraceSummary {
    pub input_trust: &'static str,
    pub frames: usize,
    pub hello_frames: usize,
    pub dispatch_frames: usize,
    pub ready_events_in_untrusted_trace: usize,
    pub heartbeat_requests: usize,
    pub heartbeat_acks_without_outbound_proof: usize,
    pub reconnects: usize,
    pub invalid_sessions: usize,
    pub other_frames: usize,
    pub last_sequence: Option<u64>,
    pub heartbeat_interval_ms: Option<u64>,
}

/// Parse a bounded newline-delimited Gateway trace. Individual frame payloads,
/// including sensitive user objects, are never retained or returned.
pub fn summarize_gateway_ndjson(input: &[u8]) -> Result<GatewayTraceSummary, String> {
    if input.len() > MAX_JSON_BYTES {
        return Err("Gateway trace exceeds size limit".to_owned());
    }
    let mut summary = GatewayTraceSummary {
        input_trust: "unverified_local_gateway_trace",
        frames: 0,
        hello_frames: 0,
        dispatch_frames: 0,
        ready_events_in_untrusted_trace: 0,
        heartbeat_requests: 0,
        heartbeat_acks_without_outbound_proof: 0,
        reconnects: 0,
        invalid_sessions: 0,
        other_frames: 0,
        last_sequence: None,
        heartbeat_interval_ms: None,
    };
    for line in input.split(|&b| b == b'\n') {
        if line.iter().all(u8::is_ascii_whitespace) {
            continue;
        }
        if summary.frames == MAX_GATEWAY_FRAMES {
            return Err("Too many Gateway trace frames".to_owned());
        }
        let frame: Value = serde_json::from_slice(line)
            .map_err(|_| format!("Invalid Gateway JSON at frame {}", summary.frames + 1))?;
        let object = frame
            .as_object()
            .ok_or_else(|| format!("Gateway frame {} must be an object", summary.frames + 1))?;
        let op = object
            .get("op")
            .and_then(Value::as_u64)
            .filter(|op| *op <= u8::MAX as u64)
            .ok_or_else(|| format!("Invalid Gateway opcode at frame {}", summary.frames + 1))?;
        summary.frames += 1;
        match op {
            0 => {
                summary.dispatch_frames += 1;
                let sequence = object.get("s").and_then(Value::as_u64).ok_or_else(|| {
                    format!("Missing dispatch sequence at frame {}", summary.frames)
                })?;
                if summary
                    .last_sequence
                    .is_some_and(|previous| sequence <= previous)
                {
                    return Err(format!(
                        "Non-increasing Gateway sequence at frame {}",
                        summary.frames
                    ));
                }
                summary.last_sequence = Some(sequence);
                let event = object
                    .get("t")
                    .and_then(Value::as_str)
                    .filter(|name| !name.is_empty())
                    .ok_or_else(|| format!("Missing dispatch event at frame {}", summary.frames))?;
                if event == "READY" {
                    summary.ready_events_in_untrusted_trace += 1;
                }
            }
            1 => summary.heartbeat_requests += 1,
            7 => {
                summary.reconnects += 1;
                summary.last_sequence = None;
            }
            9 => {
                summary.invalid_sessions += 1;
                summary.last_sequence = None;
            }
            10 => {
                summary.hello_frames += 1;
                let interval = object
                    .get("d")
                    .and_then(|data| data.get("heartbeat_interval"))
                    .and_then(Value::as_u64)
                    .filter(|value| (1..=3_600_000).contains(value))
                    .ok_or_else(|| format!("Invalid HELLO interval at frame {}", summary.frames))?;
                summary.heartbeat_interval_ms = Some(interval);
            }
            // An ACK seen in an inbound-only trace is not proof that an outbound
            // heartbeat was sent, or that a session is authenticated.
            11 => summary.heartbeat_acks_without_outbound_proof += 1,
            _ => summary.other_frames += 1,
        }
    }
    Ok(summary)
}
