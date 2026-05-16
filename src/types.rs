use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub target: String,
    pub ip: Option<String>,
    pub dns_time_ms: Option<f64>,
    pub ping_time_ms: Option<f64>,
    pub ping_loss_pct: f64,
    pub error: Option<String>,
}
