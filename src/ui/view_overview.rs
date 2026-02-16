use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Sparkline};
use ratatui::Frame;

use crate::app::App;
use crate::ui::theme;
use crate::util::{format_bytes, format_bytes_rate, format_duration_short};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    draw_dashboard_grid(frame, app, rows[0]);
    draw_system_info(frame, app, rows[1]);
}

fn draw_dashboard_grid(frame: &mut Frame, app: &App, area: Rect) {
    let top_row = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_row[0]);

    let bottom_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_row[1]);

    draw_cpu_panel(frame, app, top_cols[0]);
    draw_memory_panel(frame, app, top_cols[1]);
    draw_network_panel(frame, app, bottom_cols[0]);
    draw_disk_panel(frame, app, bottom_cols[1]);
}

fn draw_cpu_panel(frame: &mut Frame, app: &App, area: Rect) {
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
        .label(format!("CPU {:.1}%", cpu));
    frame.render_widget(gauge, chunks[0]);

    let data = app.cpu_history_sparkline();
    let sparkline = Sparkline::default()
        .data(&data)
        .style(Style::default().fg(theme::CPU_COLOR));
    frame.render_widget(sparkline, chunks[1]);
}

fn draw_memory_panel(frame: &mut Frame, app: &App, area: Rect) {
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

    let ram_pct = if sys.total_memory > 0 {
        ((sys.used_memory as f64 / sys.total_memory as f64) * 100.0) as u16
    } else {
        0
    };
    let ram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme::MEMORY_COLOR))
        .percent(ram_pct.min(100))
        .label(format!(
            "RAM {}/{} ({}%)",
            format_bytes(sys.used_memory),
            format_bytes(sys.total_memory),
            ram_pct
        ));
    frame.render_widget(ram_gauge, chunks[0]);

    let swap_pct = if sys.total_swap > 0 {
        ((sys.used_swap as f64 / sys.total_swap as f64) * 100.0) as u16
    } else {
        0
    };
    let swap_gauge = Gauge::default()
        .gauge_style(Style::default().fg(theme::SWAP_COLOR))
        .percent(swap_pct.min(100))
        .label(format!(
            "SWP {}/{} ({}%)",
            format_bytes(sys.used_swap),
            format_bytes(sys.total_swap),
            swap_pct
        ));
    frame.render_widget(swap_gauge, chunks[1]);

    let data = app.mem_history_sparkline();
    let sparkline = Sparkline::default()
        .data(&data)
        .style(Style::default().fg(theme::MEMORY_COLOR));
    frame.render_widget(sparkline, chunks[2]);
}

fn draw_network_panel(frame: &mut Frame, app: &App, area: Rect) {
    let net = &app.collector.network_data;

    let block = Block::default()
        .title(" Network ")
        .borders(Borders::ALL)
        .border_style(theme::border_style());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner);

    let info = Line::from(vec![
        Span::styled(
            format!("▼ {}", format_bytes_rate(net.bytes_received)),
            Style::default().fg(theme::NETWORK_RX_COLOR),
        ),
        Span::raw("  "),
        Span::styled(
            format!("▲ {}", format_bytes_rate(net.bytes_transmitted)),
            Style::default().fg(theme::NETWORK_TX_COLOR),
        ),
    ]);
    frame.render_widget(Paragraph::new(info), chunks[0]);

    let data = app.net_rx_history_sparkline();
    let sparkline = Sparkline::default()
        .data(&data)
        .style(Style::default().fg(theme::NETWORK_RX_COLOR));
    frame.render_widget(sparkline, chunks[1]);
}

fn draw_disk_panel(frame: &mut Frame, app: &App, area: Rect) {
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

fn draw_system_info(frame: &mut Frame, app: &App, area: Rect) {
    let sys = &app.collector.system_data;

    let block = Block::default()
        .title(" System Information ")
        .borders(Borders::ALL)
        .border_style(theme::border_style());

    let bold = Style::default().add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(Color::Cyan);

    let process_count = app.process_list.all_entries().len();
    let pinned_count = app.process_list.pinned_names().len();
    let hidden_count = app.process_list.hidden_count();
    let cpu_count = sys.cpu_usage.len();

    let os_name = sysinfo::System::long_os_version().unwrap_or_else(|| "Unknown".to_string());

    let text = vec![
        Line::from(vec![
            Span::styled("  Hostname:    ", label_style),
            Span::styled(&sys.hostname, bold),
            Span::raw("       "),
            Span::styled("OS:          ", label_style),
            Span::raw(&os_name),
        ]),
        Line::from(vec![
            Span::styled("  Uptime:      ", label_style),
            Span::raw(format_duration_short(sys.uptime)),
            Span::raw("       "),
            Span::styled("CPU Cores:   ", label_style),
            Span::raw(format!("{}", cpu_count)),
        ]),
        Line::from(vec![
            Span::styled("  Total RAM:   ", label_style),
            Span::raw(format_bytes(sys.total_memory)),
            Span::raw("       "),
            Span::styled("Total Swap:  ", label_style),
            Span::raw(format_bytes(sys.total_swap)),
        ]),
        Line::from(vec![
            Span::styled("  Processes:   ", label_style),
            Span::raw(format!("{}", process_count)),
            Span::raw("       "),
            Span::styled("Pinned:      ", label_style),
            Span::raw(format!("{}", pinned_count)),
            Span::raw("       "),
            Span::styled("Hidden:      ", label_style),
            Span::raw(format!("{}", hidden_count)),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
