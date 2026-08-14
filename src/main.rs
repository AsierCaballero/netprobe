mod app;
mod cli;
mod prober;
mod types;
mod ui;

use crate::app::App;
use crate::cli::Cli;
use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::{Terminal, backend::CrosstermBackend};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let targets = if cli.targets.is_empty() {
        vec!["8.8.8.8".into(), "1.1.1.1".into()]
    } else {
        cli.targets
    };

    if cli.json {
        let mut results = Vec::new();
        for t in &targets {
            results.push(prober::Prober::probe(t, cli.port, cli.count, 5).await);
        }
        println!("{}", serde_json::to_string_pretty(&results)?);
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(targets, cli.port, cli.count);

    loop {
        app.run_pass().await;
        terminal.draw(|f| ui::draw(f, &app))?;
        tokio::time::sleep(std::time::Duration::from_secs_f64(cli.interval)).await;
    }
}
