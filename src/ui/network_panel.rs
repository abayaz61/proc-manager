use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline};
use ratatui::Frame;

use crate::app::App;
use crate::util::format_bytes_rate;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let net = &app.collector.network_data;
    let p = &app.palette;

    let block = Block::default()
        .title(" Network ")
        .borders(Borders::ALL)
        .border_style(p.border_style());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    let info = Line::from(vec![
        Span::styled(
            format!("▼ {}", format_bytes_rate(net.bytes_received)),
            Style::default().fg(p.network_rx),
        ),
        Span::raw("  "),
        Span::styled(
            format!("▲ {}", format_bytes_rate(net.bytes_transmitted)),
            Style::default().fg(p.network_tx),
        ),
    ]);
    frame.render_widget(Paragraph::new(info), chunks[0]);

    let data = app.net_rx_history_sparkline();
    let sparkline = Sparkline::default()
        .data(&data)
        .style(Style::default().fg(p.network_rx));
    frame.render_widget(sparkline, chunks[1]);
}
