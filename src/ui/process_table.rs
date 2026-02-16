use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use ratatui::Frame;

use crate::app::App;
use crate::data::process::{ProcessEntry, ProcessStatus, SortColumn};
use crate::ui::theme;
use crate::util::{format_bytes, format_cpu_percent, format_duration_short};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    app.process_table_area.set(area);

    let is_wide = area.width >= 100;
    let has_pins = app.process_list.has_pins();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let (header, widths) = build_header(is_wide, app);

    // Build rows: pinned first, separator, then unpinned
    let mut rows: Vec<Row> = Vec::new();

    if has_pins {
        let pinned = app.process_list.pinned_visible();
        for p in &pinned {
            rows.push(build_row(p, now, is_wide, true));
        }
        // Separator row
        let sep_text = format!("── pinned: {} ──", pinned.len());
        let sep_cells = if is_wide {
            vec![
                Cell::from(""),
                Cell::from(sep_text).style(Style::default().fg(Color::DarkGray)),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ]
        } else {
            vec![
                Cell::from(""),
                Cell::from(sep_text).style(Style::default().fg(Color::DarkGray)),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
            ]
        };
        rows.push(Row::new(sep_cells).style(Style::default().fg(Color::DarkGray)));
    }

    let unpinned = if has_pins {
        app.process_list.unpinned_visible()
    } else {
        app.process_list.visible()
    };
    for p in &unpinned {
        rows.push(build_row(p, now, is_wide, false));
    }

    // Title with process count and pin info
    let title = build_title(app, has_pins);

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(theme::border_style()),
        )
        .row_highlight_style(theme::selected_style());

    frame.render_stateful_widget(table, area, &mut app.table_state.clone());
}

fn build_header(is_wide: bool, app: &App) -> (Row<'static>, Vec<Constraint>) {
    if is_wide {
        let h = Row::new([
            column_header("PID", SortColumn::Pid, app),
            column_header("Name", SortColumn::Name, app),
            column_header("User", SortColumn::User, app),
            column_header("CPU%", SortColumn::Cpu, app),
            column_header("Memory", SortColumn::Memory, app),
            column_header("Status", SortColumn::Status, app),
            column_header("Threads", SortColumn::Threads, app),
            column_header("Time", SortColumn::StartTime, app),
        ])
        .style(theme::table_header_style());
        let w = vec![
            Constraint::Length(8),
            Constraint::Min(15),
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(7),
            Constraint::Length(8),
            Constraint::Length(10),
        ];
        (h, w)
    } else {
        let h = Row::new([
            column_header("PID", SortColumn::Pid, app),
            column_header("Name", SortColumn::Name, app),
            column_header("CPU%", SortColumn::Cpu, app),
            column_header("Memory", SortColumn::Memory, app),
            column_header("Status", SortColumn::Status, app),
        ])
        .style(theme::table_header_style());
        let w = vec![
            Constraint::Length(8),
            Constraint::Min(12),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(7),
        ];
        (h, w)
    }
}

fn build_row(p: &ProcessEntry, now: u64, is_wide: bool, is_pinned: bool) -> Row<'static> {
    let running_time = now.saturating_sub(p.start_time);

    // Dead pinned processes get a faded gray style
    if p.is_dead {
        let dead_style = Style::default().fg(Color::DarkGray);
        let pid_prefix = format!("* {}", p.pid);

        return if is_wide {
            Row::new(vec![
                Cell::from(pid_prefix).style(dead_style),
                Cell::from(p.name.clone()).style(dead_style),
                Cell::from(p.user.clone()).style(dead_style),
                Cell::from("-").style(dead_style),
                Cell::from(format_bytes(p.memory_bytes)).style(dead_style),
                Cell::from("Dead").style(dead_style),
                Cell::from("-").style(dead_style),
                Cell::from("-").style(dead_style),
            ])
        } else {
            Row::new(vec![
                Cell::from(pid_prefix).style(dead_style),
                Cell::from(p.name.clone()).style(dead_style),
                Cell::from("-").style(dead_style),
                Cell::from(format_bytes(p.memory_bytes)).style(dead_style),
                Cell::from("Dead").style(dead_style),
            ])
        };
    }

    let status_style = match p.status {
        ProcessStatus::Running => Style::default().fg(theme::STATUS_RUNNING),
        ProcessStatus::Sleeping => Style::default().fg(theme::STATUS_SLEEPING),
        ProcessStatus::Stopped => Style::default().fg(theme::STATUS_STOPPED),
        ProcessStatus::Zombie => Style::default().fg(theme::STATUS_ZOMBIE),
        ProcessStatus::Dead => Style::default().fg(Color::DarkGray),
        ProcessStatus::Unknown => Style::default(),
    };

    let pid_prefix = if is_pinned {
        format!("* {}", p.pid)
    } else {
        format!("  {}", p.pid)
    };

    let row_style = if is_pinned {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    if is_wide {
        Row::new(vec![
            Cell::from(pid_prefix).style(row_style),
            Cell::from(p.name.clone()).style(row_style),
            Cell::from(p.user.clone()),
            Cell::from(format_cpu_percent(p.cpu_percent)),
            Cell::from(format_bytes(p.memory_bytes)),
            Cell::from(p.status.as_str()).style(status_style),
            Cell::from(p.thread_count.to_string()),
            Cell::from(format_duration_short(running_time)),
        ])
    } else {
        Row::new(vec![
            Cell::from(pid_prefix).style(row_style),
            Cell::from(p.name.clone()).style(row_style),
            Cell::from(format_cpu_percent(p.cpu_percent)),
            Cell::from(format_bytes(p.memory_bytes)),
            Cell::from(p.status.as_str()).style(status_style),
        ])
    }
}

fn build_title(app: &App, has_pins: bool) -> String {
    let mut parts = vec![format!(
        " Processes: {}/{}",
        app.process_list.visible_count(),
        app.process_list.total_count()
    )];

    if has_pins {
        let dead_count = app.process_list.dead_pin_count();
        if dead_count > 0 {
            parts.push(format!(
                " pinned: {} ({}dead)",
                app.process_list.pin_count(),
                dead_count
            ));
        } else {
            parts.push(format!(" pinned: {}", app.process_list.pin_count()));
        }
    }

    let hidden_count = app.process_list.hidden_count();
    if hidden_count > 0 {
        parts.push(format!(" hidden: {}", hidden_count));
    }

    if !app.search_query.is_empty() {
        parts.push(format!(" filter: \"{}\"", app.search_query));
    }

    parts.push(" ".to_string());
    parts.join(" |")
}

fn column_header(name: &str, col: SortColumn, app: &App) -> Cell<'static> {
    if app.process_list.sort_column == col {
        let arrow = if app.process_list.sort_ascending {
            "▲"
        } else {
            "▼"
        };
        Cell::from(format!("{}{}", name, arrow)).style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Cell::from(name.to_string())
    }
}
