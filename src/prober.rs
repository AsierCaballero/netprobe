use std::time::Instant;
use tokio::net::TcpStream;
use tokio::time::timeout;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;
use crate::types::ProbeResult;

pub struct Prober;

impl Prober {
    pub async fn probe(target: &str, port: u16, count: u32, timeout_secs: u64) -> ProbeResult {
        let start = Instant::now();

        let (ip, dns_ms) = Self::resolve(target).await;
        let (ping_ms, loss) = Self::ping_series(target, count, timeout_secs).await;
        let (tcp_ms, tcp_err) = Self::tcp_connect(target, port, timeout_secs).await;

        ProbeResult {
            target: target.into(),
            ip,
            dns_time_ms: dns_ms,
            tcp_time_ms: tcp_ms,
            ping_time_ms: ping_ms,
            ping_loss_pct: loss,
            error: tcp_err,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    async fn resolve(host: &str) -> (Option<String>, Option<f64>) {
        let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
        let start = Instant::now();
        match resolver.lookup_ip(host).await {
            Ok(response) => {
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                (response.iter().next().map(|ip| ip.to_string()), Some(elapsed))
            }
            Err(_) => (None, None),
        }
    }

    async fn tcp_connect(host: &str, port: u16, secs: u64) -> (Option<f64>, Option<String>) {
        let addr = format!("{}:{}", host, port);
        let start = Instant::now();
        match timeout(std::time::Duration::from_secs(secs), TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => (Some(start.elapsed().as_secs_f64() * 1000.0), None),
            Ok(Err(e)) => (None, Some(format!("TCP: {}", e))),
            Err(_) => (None, Some("TCP: timeout".into())),
        }
    }

    async fn ping_series(target: &str, count: u32, secs: u64) -> (Option<f64>, f64) {
        let mut ok = 0u32;
        let mut total = 0.0;
        for _ in 0..count {
            let start = Instant::now();
            let dur = std::time::Duration::from_secs(secs);
            match timeout(dur, ping::ping(target, None)).await {
                Ok(Ok(_)) => { ok += 1; total += start.elapsed().as_secs_f64() * 1000.0; }
                _ => {}
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        if ok == 0 { return (None, 100.0); }
        (Some(total / ok as f64), (1.0 - ok as f64 / count as f64) * 100.0)
    }
}
