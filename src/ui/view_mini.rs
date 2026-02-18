use ratatui::layout::{Constraint, Direction, Layout, Rect, Alignment};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;

use crate::app::App;
use crate::util::{format_bytes, format_cpu_percent};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // system stats
            Constraint::Min(3),   // pinned process table
        ])
        .split(area);

    draw_system_stats(frame, app, chunks[0]);
    draw_pinned_table(frame, app, chunks[1]);
}

fn draw_system_stats(frame: &mut Frame, app: &App, area: Rect) {
    let p = &app.palette;
    let sys = &app.collector.system_data;

    let cpu_pct = sys.global_cpu_usage;
    let mem_used = sys.used_memory;
    let mem_total = sys.total_memory;
    let mem_pct = if mem_total > 0 {
        (mem_used as f64 / mem_total as f64) * 100.0
    } else {
        0.0
    };

    // Aggregate disk I/O from processes
    let (disk_read, disk_write) = app
        .process_list
        .all_entries()
        .iter()
        .fold((0u64, 0u64), |(r, w), proc_entry| {
            (r + proc_entry.disk_read_bytes, w + proc_entry.disk_write_bytes)
        });

    let label = Style::default().fg(p.text_secondary);
    let cpu_style = Style::default().fg(p.cpu).add_modifier(Modifier::BOLD);
    let mem_style = Style::default().fg(p.memory).add_modifier(Modifier::BOLD);
    let disk_r_style = Style::default().fg(p.disk_read);
    let disk_w_style = Style::default().fg(p.disk_write);

    let lines = vec![
        Line::from(vec![
            Span::styled("  CPU: ", label),
            Span::styled(format!("{:.1}%", cpu_pct), cpu_style),
            Span::styled("    RAM: ", label),
            Span::styled(
                format!("{}/{} ({:.0}%)", format_bytes(mem_used), format_bytes(mem_total), mem_pct),
                mem_style,
            ),
        ]),
        Line::from(vec![
            Span::styled("  Disk R: ", label),
            Span::styled(format_bytes(disk_read), disk_r_style),
            Span::styled("    Disk W: ", label),
            Span::styled(format_bytes(disk_write), disk_w_style),
        ]),
    ];

    let block = Block::default()
        .title(" System ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(p.border));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_pinned_table(frame: &mut Frame, app: &App, area: Rect) {
    let p = &app.palette;
    let pinned = app.process_list.pinned_visible();

    if pinned.is_empty() {
        let block = Block::default()
            .title(" Pinned Processes ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(p.border));

        let msg = Paragraph::new(Line::from(Span::styled(
            "No pinned processes — press P in process view",
            Style::default().fg(p.text_secondary),
        )))
        .alignment(Alignment::Center)
        .block(block);

        frame.render_widget(msg, area);
        return;
    }

    let header_style = p.table_header_style();
    let header = Row::new(vec![
        Cell::from("PID"),
        Cell::from("Name"),
        Cell::from("CPU%"),
        Cell::from("Memory"),
    ])
    .style(header_style);

    let widths = [
        Constraint::Length(8),
        Constraint::Min(12),
        Constraint::Length(8),
        Constraint::Length(10),
    ];

    let rows: Vec<Row> = pinned
        .iter()
        .map(|proc_entry| {
            if proc_entry.is_dead {
                let dead_style = Style::default().fg(p.border);
                Row::new(vec![
                    Cell::from(proc_entry.pid.to_string()).style(dead_style),
                    Cell::from(proc_entry.name.clone()).style(dead_style),
                    Cell::from("-").style(dead_style),
                    Cell::from(format_bytes(proc_entry.memory_bytes)).style(dead_style),
                ])
            } else {
                Row::new(vec![
                    Cell::from(proc_entry.pid.to_string()).style(Style::default().fg(p.pin)),
                    Cell::from(proc_entry.name.clone()).style(Style::default().fg(p.pin)),
                    Cell::from(format_cpu_percent(proc_entry.cpu_percent))
                        .style(Style::default().fg(p.cpu)),
                    Cell::from(format_bytes(proc_entry.memory_bytes))
                        .style(Style::default().fg(p.memory)),
                ])
            }
        })
        .collect();

    let title = format!(" Pinned Processes: {} ", pinned.len());
    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(p.border)),
        )
        .row_highlight_style(p.selected_style());

    frame.render_widget(table, area);
}
