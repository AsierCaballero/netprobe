mod cli;
mod prober;
mod types;
mod ui;

use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::cli::Cli;
use crate::app::App;

mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let targets = if cli.targets.is_empty() {
        vec!["8.8.8.8".into(), "1.1.1.1".into()]
    } else { cli.targets };

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(targets, cli.port, cli.count);
    app.run_pass().await?;

    terminal.draw(|f| ui::draw(f, &app))?;
    std::thread::sleep(std::time::Duration::from_secs(5));

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
