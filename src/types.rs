use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub target: String,
    pub ip: Option<String>,
    pub dns_time_ms: Option<f64>,
    pub tcp_time_ms: Option<f64>,
    pub ping_time_ms: Option<f64>,
    pub ping_loss_pct: f64,
    pub timestamp: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetConfig {
    pub host: String,
    pub port: Option<u16>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub targets: Vec<TargetConfig>,
    pub port: Option<u16>,
    pub count: Option<u32>,
    pub interval: Option<f64>,
}
EOF

cat > src/cli.rs << 'RUST'
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "netprobe", version)]
pub struct Cli {
    pub targets: Vec<String>,
    #[arg(short = 'p', long, default_value_t = 80)]
    pub port: u16,
    #[arg(short = 'n', long, default_value_t = 3)]
    pub count: u32,
    #[arg(short = 'i', long, default_value_t = 2.0)]
    pub interval: f64,
    #[arg(long)]
    pub json: bool,
    #[arg(short = 'c', long)]
    pub config: Option<PathBuf>,
}
EOF

cat > src/main.rs << 'RUST'
mod app;
mod cli;
mod prober;
mod types;
mod ui;

use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::app::App;
use crate::cli::Cli;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let (targets, port, count, interval) = if let Some(ref cfg_path) = cli.config {
        let cfg: crate::types::Config = toml::from_str(&std::fs::read_to_string(cfg_path)?)?;
        let targets: Vec<String> = cfg.targets.iter().map(|t| t.host.clone()).collect();
        if targets.is_empty() { return Err(anyhow::anyhow!("no targets in config")); }
        (targets, cfg.port.unwrap_or(cli.port), cfg.count.unwrap_or(cli.count), cfg.interval.unwrap_or(cli.interval))
    } else {
        let t = if cli.targets.is_empty() { vec!["8.8.8.8".into(), "1.1.1.1".into()] } else { cli.targets.clone() };
        (t, cli.port, cli.count, cli.interval)
    };

    if cli.json {
        let mut results = Vec::new();
        for t in &targets {
            results.push(prober::Prober::probe(t, port, count, 5).await);
        }
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(targets, port, count);

    loop {
        app.run_pass().await;
        terminal.draw(|f| ui::draw(f, &app))?;
        tokio::time::sleep(std::time::Duration::from_secs_f64(interval)).await;
    }
}
