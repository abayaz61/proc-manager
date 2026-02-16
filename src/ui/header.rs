use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::util::format_duration_short;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let sys = &app.collector.system_data;
    let p = &app.palette;

    let uptime = format_duration_short(sys.uptime);
    let now = chrono_free_time();

    let line = Line::from(vec![
        Span::styled(" proc-manager v0.1.0", Style::default().fg(p.header_fg)),
        Span::raw("   "),
        Span::styled(&sys.hostname, Style::default().fg(p.header_fg)),
        Span::raw("   "),
        Span::styled(format!("Up: {}", uptime), Style::default().fg(p.header_fg)),
        Span::raw("   "),
        Span::styled(now, Style::default().fg(p.header_fg)),
    ]);

    let paragraph = Paragraph::new(line).style(p.header_style());
    frame.render_widget(paragraph, area);
}

fn chrono_free_time() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}:{:02} UTC", hours, minutes, seconds)
}
