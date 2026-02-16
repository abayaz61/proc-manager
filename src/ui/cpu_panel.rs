use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Gauge, Sparkline};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::Frame;

use crate::app::App;
use crate::ui::theme;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" CPU ")
        .borders(Borders::ALL)
        .border_style(theme::border_style());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    let cpu = app.collector.system_data.global_cpu_usage;
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme::CPU_COLOR))
        .percent(cpu.min(100.0) as u16)
        .label(format!("{:.1}%", cpu));
    frame.render_widget(gauge, chunks[0]);

    let data = app.cpu_history_sparkline();
    let sparkline = Sparkline::default()
        .data(&data)
        .style(Style::default().fg(theme::CPU_COLOR));
    frame.render_widget(sparkline, chunks[1]);
}
