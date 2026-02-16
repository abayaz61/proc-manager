use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::ui::theme::ColorPalette;
use crate::util::{format_bytes, format_duration_short};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),  // OS
            Constraint::Length(9),  // CPU
            Constraint::Min(6),    // Memory
        ])
        .split(cols[0]);

    let right_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // BIOS
            Constraint::Length(9),  // Motherboard / System
            Constraint::Min(6),    // Runtime stats
        ])
        .split(cols[1]);

    draw_os_section(frame, app, left_rows[0]);
    draw_cpu_section(frame, app, left_rows[1]);
    draw_memory_section(frame, app, left_rows[2]);
    draw_bios_section(frame, app, right_rows[0]);
    draw_board_section(frame, app, right_rows[1]);
    draw_runtime_section(frame, app, right_rows[2]);
}

fn label_value_line(label: &str, value: &str) -> Line<'static> {
    let label_style = Style::default().fg(Color::Cyan);
    let val_style = Style::default().fg(Color::White);
    Line::from(vec![
        Span::styled(format!("  {:<20}", label), label_style),
        Span::styled(value.to_string(), val_style),
    ])
}

fn section_block(title: &str, palette: &ColorPalette) -> Block<'static> {
    Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(palette.border_style())
}

fn draw_os_section(frame: &mut Frame, app: &App, area: Rect) {
    let info = &app.system_info_detail;
    let sys = &app.collector.system_data;

    let text = vec![
        label_value_line("Hostname:", &sys.hostname),
        label_value_line("OS Name:", &info.os_name),
        label_value_line("OS:", &info.os_long_version),
        label_value_line("OS Version:", &info.os_version),
        label_value_line("Kernel:", &info.kernel_version),
        label_value_line("Architecture:", &info.cpu_arch),
        label_value_line("Uptime:", &format_duration_short(sys.uptime)),
    ];

    let block = section_block("Operating System", &app.palette);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_cpu_section(frame: &mut Frame, app: &App, area: Rect) {
    let info = &app.system_info_detail;
    let sys = &app.collector.system_data;

    let logical_cores = sys.cpu_usage.len();

    let text = vec![
        label_value_line("Model:", &info.cpu_brand),
        label_value_line("Vendor:", &info.cpu_vendor),
        label_value_line("Physical Cores:", &info.physical_core_count.to_string()),
        label_value_line("Logical Cores:", &logical_cores.to_string()),
        label_value_line("Architecture:", &info.cpu_arch),
        label_value_line("Global Usage:", &format!("{:.1}%", sys.global_cpu_usage)),
        cpu_usage_bar(sys.global_cpu_usage),
    ];

    let block = section_block("CPU", &app.palette);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn cpu_usage_bar(usage: f32) -> Line<'static> {
    let width: usize = 40;
    let filled = ((usage / 100.0) * width as f32).round() as usize;
    let empty = width.saturating_sub(filled);

    let color = if usage > 90.0 {
        Color::Red
    } else if usage > 70.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    Line::from(vec![
        Span::raw("  "),
        Span::styled(
            "\u{2588}".repeat(filled),
            Style::default().fg(color),
        ),
        Span::styled(
            "\u{2591}".repeat(empty),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            format!(" {:.1}%", usage),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
    ])
}

fn draw_memory_section(frame: &mut Frame, app: &App, area: Rect) {
    let sys = &app.collector.system_data;

    let ram_pct = if sys.total_memory > 0 {
        (sys.used_memory as f64 / sys.total_memory as f64) * 100.0
    } else {
        0.0
    };
    let swap_pct = if sys.total_swap > 0 {
        (sys.used_swap as f64 / sys.total_swap as f64) * 100.0
    } else {
        0.0
    };
    let available = sys.total_memory.saturating_sub(sys.used_memory);

    let text = vec![
        label_value_line("Total RAM:", &format_bytes(sys.total_memory)),
        label_value_line("Used RAM:", &format!("{} ({:.1}%)", format_bytes(sys.used_memory), ram_pct)),
        label_value_line("Available RAM:", &format_bytes(available)),
        label_value_line("Total Swap:", &format_bytes(sys.total_swap)),
        label_value_line("Used Swap:", &format!("{} ({:.1}%)", format_bytes(sys.used_swap), swap_pct)),
    ];

    let block = section_block("Memory", &app.palette);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_bios_section(frame: &mut Frame, app: &App, area: Rect) {
    let info = &app.system_info_detail;

    let text = vec![
        label_value_line("Vendor:", &info.bios_vendor),
        label_value_line("Version:", &info.bios_version),
        label_value_line("Release Date:", &info.bios_release_date),
        Line::from(""),
        label_value_line("System Manufacturer:", &info.system_manufacturer),
        label_value_line("System Product:", &info.system_product),
    ];

    let block = section_block("BIOS / Firmware", &app.palette);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_board_section(frame: &mut Frame, app: &App, area: Rect) {
    let info = &app.system_info_detail;

    let text = vec![
        label_value_line("Manufacturer:", &info.board_manufacturer),
        label_value_line("Product:", &info.board_product),
        Line::from(""),
        label_value_line("System SKU:", &info.system_sku),
        label_value_line("System Family:", &info.system_family),
    ];

    let block = section_block("Motherboard", &app.palette);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_runtime_section(frame: &mut Frame, app: &App, area: Rect) {
    let process_count = app.process_list.all_entries().len();
    let pinned_count = app.process_list.pinned_names().len();
    let hidden_count = app.process_list.hidden_count();

    let text = vec![
        label_value_line("Total Processes:", &process_count.to_string()),
        label_value_line("Pinned:", &pinned_count.to_string()),
        label_value_line("Hidden:", &hidden_count.to_string()),
    ];

    let block = section_block("Runtime", &app.palette);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}
