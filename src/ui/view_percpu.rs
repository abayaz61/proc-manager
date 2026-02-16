use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Borders, Gauge};
use ratatui::Frame;

use crate::app::App;
use crate::util::format_bytes;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let sys = &app.collector.system_data;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(4)])
        .split(area);

    draw_global_gauges(frame, app, chunks[0]);
    draw_per_cpu_bars(frame, app, sys.cpu_usage.as_slice(), chunks[1]);
}

fn draw_global_gauges(frame: &mut Frame, app: &App, area: Rect) {
    let sys = &app.collector.system_data;
    let p = &app.palette;

    let block = Block::default()
        .title(" System ")
        .borders(Borders::ALL)
        .border_style(p.border_style());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(inner);

    // CPU gauge
    let cpu = sys.global_cpu_usage;
    let cpu_gauge = Gauge::default()
        .gauge_style(Style::default().fg(p.cpu))
        .percent(cpu.min(100.0) as u16)
        .label(format!("CPU {:.1}%", cpu));
    frame.render_widget(cpu_gauge, cols[0]);

    // RAM gauge
    let ram_pct = if sys.total_memory > 0 {
        ((sys.used_memory as f64 / sys.total_memory as f64) * 100.0) as u16
    } else {
        0
    };
    let ram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(p.memory))
        .percent(ram_pct.min(100))
        .label(format!(
            "RAM {}/{}",
            format_bytes(sys.used_memory),
            format_bytes(sys.total_memory)
        ));
    frame.render_widget(ram_gauge, cols[1]);

    // Swap gauge
    let swap_pct = if sys.total_swap > 0 {
        ((sys.used_swap as f64 / sys.total_swap as f64) * 100.0) as u16
    } else {
        0
    };
    let swap_gauge = Gauge::default()
        .gauge_style(Style::default().fg(p.swap))
        .percent(swap_pct.min(100))
        .label(format!(
            "SWP {}/{}",
            format_bytes(sys.used_swap),
            format_bytes(sys.total_swap)
        ));
    frame.render_widget(swap_gauge, cols[2]);
}

fn draw_per_cpu_bars(frame: &mut Frame, app: &App, cpu_usage: &[f32], area: Rect) {
    let block = Block::default()
        .title(" Per-CPU Usage ")
        .borders(Borders::ALL)
        .border_style(app.palette.border_style());

    let bars: Vec<Bar> = cpu_usage
        .iter()
        .enumerate()
        .map(|(i, &usage)| {
            let color = if usage > 90.0 {
                Color::Red
            } else if usage > 80.0 {
                Color::Yellow
            } else {
                Color::Green
            };
            Bar::default()
                .value(usage as u64)
                .label(format!("CPU{}", i).into())
                .style(Style::default().fg(color))
        })
        .collect();

    let bar_chart = BarChart::default()
        .block(block)
        .data(BarGroup::default().bars(&bars))
        .bar_width(5)
        .bar_gap(1)
        .max(100);

    frame.render_widget(bar_chart, area);
}
