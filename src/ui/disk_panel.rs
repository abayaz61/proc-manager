use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline};
use ratatui::Frame;

use crate::app::App;
use crate::ui::theme;
use crate::util::format_bytes_rate;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Disk I/O ")
        .borders(Borders::ALL)
        .border_style(theme::border_style());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // Aggregate disk I/O from all processes
    let (total_read, total_write) = app
        .process_list
        .all_entries()
        .iter()
        .fold((0u64, 0u64), |(r, w), p| {
            (r + p.disk_read_bytes, w + p.disk_write_bytes)
        });

    let info = Line::from(vec![
        Span::styled(
            format!("R {}", format_bytes_rate(total_read)),
            Style::default().fg(Color::Green),
        ),
        Span::raw("  "),
        Span::styled(
            format!("W {}", format_bytes_rate(total_write)),
            Style::default().fg(Color::Red),
        ),
    ]);
    frame.render_widget(Paragraph::new(info), chunks[0]);

    let data = app.disk_read_history_sparkline();
    let sparkline = Sparkline::default()
        .data(&data)
        .style(Style::default().fg(Color::Green));
    frame.render_widget(sparkline, chunks[1]);
}
