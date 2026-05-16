use std::time::Instant;
use crate::types::ProbeResult;

pub struct Prober;

impl Prober {
    pub async fn probe(target: &str, port: u16, count: u32) -> ProbeResult {
        let ip = Self::resolve(target).await;
        let (ping_time, loss) = Self::ping_series(target, count).await;
        ProbeResult {
            target: target.into(),
            ip,
            dns_time_ms: None,
            ping_time_ms: ping_time,
            ping_loss_pct: loss,
            error: None,
        }
    }

    async fn resolve(host: &str) -> Option<String> {
        match tokio::net::lookup_host((host, 0)).await {
            Ok(mut addrs) => addrs.next().map(|a| a.ip().to_string()),
            Err(_) => None,
        }
    }

    async fn ping_series(target: &str, count: u32) -> (Option<f64>, f64) {
        let mut ok = 0u32;
        for _ in 0..count {
            if (ping::ping(target, None)).is_ok() {
                ok += 1;
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        if ok == 0 { return (None, 100.0); }
        (Some(0.0), (1.0 - ok as f64 / count as f64) * 100.0)
    }
}
