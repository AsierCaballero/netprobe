use ratatui::{Frame, layout::{Constraint, Layout, Direction}, style::{Style, Color}, widgets::{Block, Borders, Paragraph, Table, Row, Cell, Gauge}, text::Span};
use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)]).split(area);

    let header = Paragraph::new(" netprobe — Multi-target Network Probe ")
        .style(Style::new().fg(Color::Cyan)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    let rows: Vec<Row> = app.targets.iter().map(|t| {
        let last = app.results.get(t).and_then(|r| r.last());
        let (ip, dns, tcp, ping, loss, status) = last.map(|r| (
            r.ip.clone().unwrap_or_default(),
            r.dns_time_ms.map(|v| format!("{:.1}", v)).unwrap_or_default(),
            r.tcp_time_ms.map(|v| format!("{:.1}", v)).unwrap_or_default(),
            r.ping_time_ms.map(|v| format!("{:.1}", v)).unwrap_or_default(),
            format!("{:.0}%", r.ping_loss_pct),
            if r.error.is_some() { "FAIL" } else { "OK" },
        )).unwrap_or_default();

        let style = if status == "FAIL" { Style::new().fg(Color::Red) } else { Style::new().fg(Color::Green) };
        Row::new(vec![
            Cell::from(Span::raw(t)),
            Cell::from(Span::raw(&ip)),
            Cell::from(Span::raw(&dns)),
            Cell::from(Span::raw(&tcp)),
            Cell::from(Span::raw(&ping)),
            Cell::from(Span::raw(&loss)),
            Cell::from(Span::styled(status, style)),
        ])
    }).collect();

    let widths = [Constraint::Length(24), Constraint::Length(16), Constraint::Length(12), Constraint::Length(12), Constraint::Length(12), Constraint::Length(10), Constraint::Length(8)];
    let table = Table::new(rows, widths)
        .header(Row::new(["Target", "IP", "DNS(ms)", "TCP(ms)", "Ping(ms)", "Loss", "Status"].iter().map(|h| Cell::from(*h)).collect::<Vec<_>>()))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(table, chunks[1]);

    let pass = format!("Pass {} | Targets: {}", app.current_pass, app.targets.len());
    let footer = Paragraph::new(format!("{}  [Q]uit  [P]ause", pass))
        .style(Style::new().fg(Color::DarkGray)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}
