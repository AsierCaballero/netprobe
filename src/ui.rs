use ratatui::{Frame, layout::{Constraint, Layout, Direction}, style::{Style, Color}, widgets::{Block, Borders, Paragraph, Table, Row, Cell}, text::Span};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)]).split(area);

    let header = Paragraph::new(" netprobe — Multi-target Network Probe ")
        .style(Style::new().fg(Color::Cyan)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    let rows: Vec<Row> = app.targets.iter().map(|t| {
        let last = app.results.get(t).and_then(|r| r.last());
        let (ip, ping, loss) = last.map(|r| (
            r.ip.clone().unwrap_or_default(),
            r.ping_time_ms.map(|v| format!("{:.1}", v)).unwrap_or_default(),
            format!("{:.0}%", r.ping_loss_pct),
        )).unwrap_or_default();
        Row::new(vec![
            Cell::from(Span::raw(t)),
            Cell::from(Span::raw(&ip)),
            Cell::from(Span::raw(&ping)),
            Cell::from(Span::raw(&loss)),
        ])
    }).collect();

    let table = Table::new(rows, [Constraint::Length(20), Constraint::Length(16), Constraint::Length(12), Constraint::Length(8)])
        .header(Row::new(["Target", "IP", "Ping", "Loss"].iter().map(|h| Cell::from(*h))))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(table, chunks[1]);
}
