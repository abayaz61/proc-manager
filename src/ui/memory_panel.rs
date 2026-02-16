use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Gauge, Sparkline};
use ratatui::Frame;

use crate::app::App;
use crate::ui::theme;
use crate::util::format_bytes;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let sys = &app.collector.system_data;

    let block = Block::default()
        .title(" Memory ")
        .borders(Borders::ALL)
        .border_style(theme::border_style());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    // RAM gauge
    let ram_pct = if sys.total_memory > 0 {
        ((sys.used_memory as f64 / sys.total_memory as f64) * 100.0) as u16
    } else {
        0
    };
    let ram_label = format!(
        "RAM {}/{} ({}%)",
        format_bytes(sys.used_memory),
        format_bytes(sys.total_memory),
        ram_pct
    );
    let ram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme::MEMORY_COLOR))
        .percent(ram_pct.min(100))
        .label(ram_label);
    frame.render_widget(ram_gauge, chunks[0]);

    // Swap gauge
    let swap_pct = if sys.total_swap > 0 {
        ((sys.used_swap as f64 / sys.total_swap as f64) * 100.0) as u16
    } else {
        0
    };
    let swap_label = format!(
        "SWP {}/{} ({}%)",
        format_bytes(sys.used_swap),
        format_bytes(sys.total_swap),
        swap_pct
    );
    let swap_gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme::SWAP_COLOR))
        .percent(swap_pct.min(100))
        .label(swap_label);
    frame.render_widget(swap_gauge, chunks[1]);

    // Memory sparkline
    let data = app.mem_history_sparkline();
    let sparkline = Sparkline::default()
        .data(&data)
        .style(Style::default().fg(theme::MEMORY_COLOR));
    frame.render_widget(sparkline, chunks[2]);
}
