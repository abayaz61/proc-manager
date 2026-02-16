use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use ratatui::Frame;

use crate::app::App;
use crate::data::process::{ProcessEntry, ProcessStatus, SortColumn};
use crate::ui::theme;
use crate::util::{format_bytes, format_cpu_percent, format_duration_short};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Column {
    Pid,
    Name,
    User,
    Cpu,
    Memory,
    Status,
    Threads,
    Time,
    DiskRead,
    DiskWrite,
    Ppid,
    Command,
}

impl Column {
    pub fn all() -> &'static [Column] {
        &[
            Column::Pid,
            Column::Name,
            Column::User,
            Column::Cpu,
            Column::Memory,
            Column::Status,
            Column::Threads,
            Column::Time,
            Column::DiskRead,
            Column::DiskWrite,
            Column::Ppid,
            Column::Command,
        ]
    }

    pub fn header_name(&self) -> &'static str {
        match self {
            Column::Pid => "PID",
            Column::Name => "Name",
            Column::User => "User",
            Column::Cpu => "CPU%",
            Column::Memory => "Memory",
            Column::Status => "Status",
            Column::Threads => "Threads",
            Column::Time => "Time",
            Column::DiskRead => "Disk R",
            Column::DiskWrite => "Disk W",
            Column::Ppid => "PPID",
            Column::Command => "Command",
        }
    }

    pub fn width(&self) -> Constraint {
        match self {
            Column::Pid => Constraint::Length(8),
            Column::Name => Constraint::Min(15),
            Column::User => Constraint::Length(12),
            Column::Cpu => Constraint::Length(8),
            Column::Memory => Constraint::Length(10),
            Column::Status => Constraint::Length(7),
            Column::Threads => Constraint::Length(8),
            Column::Time => Constraint::Length(10),
            Column::DiskRead => Constraint::Length(10),
            Column::DiskWrite => Constraint::Length(10),
            Column::Ppid => Constraint::Length(8),
            Column::Command => Constraint::Min(20),
        }
    }

    pub fn is_required(&self) -> bool {
        matches!(self, Column::Pid | Column::Name)
    }

    pub fn to_id(&self) -> &'static str {
        match self {
            Column::Pid => "pid",
            Column::Name => "name",
            Column::User => "user",
            Column::Cpu => "cpu",
            Column::Memory => "memory",
            Column::Status => "status",
            Column::Threads => "threads",
            Column::Time => "time",
            Column::DiskRead => "disk_read",
            Column::DiskWrite => "disk_write",
            Column::Ppid => "ppid",
            Column::Command => "command",
        }
    }

    pub fn from_id(s: &str) -> Option<Column> {
        match s {
            "pid" => Some(Column::Pid),
            "name" => Some(Column::Name),
            "user" => Some(Column::User),
            "cpu" => Some(Column::Cpu),
            "memory" => Some(Column::Memory),
            "status" => Some(Column::Status),
            "threads" => Some(Column::Threads),
            "time" => Some(Column::Time),
            "disk_read" => Some(Column::DiskRead),
            "disk_write" => Some(Column::DiskWrite),
            "ppid" => Some(Column::Ppid),
            "command" => Some(Column::Command),
            _ => None,
        }
    }

    pub fn sort_column(&self) -> Option<SortColumn> {
        match self {
            Column::Pid => Some(SortColumn::Pid),
            Column::Name => Some(SortColumn::Name),
            Column::User => Some(SortColumn::User),
            Column::Cpu => Some(SortColumn::Cpu),
            Column::Memory => Some(SortColumn::Memory),
            Column::Status => Some(SortColumn::Status),
            Column::Threads => Some(SortColumn::Threads),
            Column::Time => Some(SortColumn::StartTime),
            Column::DiskRead => Some(SortColumn::DiskRead),
            Column::DiskWrite => Some(SortColumn::DiskWrite),
            Column::Ppid => Some(SortColumn::Ppid),
            Column::Command => None,
        }
    }

    /// Parse a list of column IDs, ensuring required columns are always present and first.
    pub fn parse_list(ids: &[String]) -> Vec<Column> {
        let mut cols: Vec<Column> = Vec::new();
        // Always start with required columns
        cols.push(Column::Pid);
        cols.push(Column::Name);
        for id in ids {
            if let Some(col) = Column::from_id(id.as_str()) {
                if !col.is_required() && !cols.contains(&col) {
                    cols.push(col);
                }
            }
        }
        cols
    }

    fn cell_value(&self, p: &ProcessEntry, now: u64, is_pinned: bool) -> (String, Option<Style>) {
        match self {
            Column::Pid => {
                let text = if is_pinned {
                    format!("* {}", p.pid)
                } else {
                    format!("  {}", p.pid)
                };
                (text, None)
            }
            Column::Name => (p.name.clone(), None),
            Column::User => (p.user.clone(), Some(Style::default())),
            Column::Cpu => (format_cpu_percent(p.cpu_percent), Some(Style::default())),
            Column::Memory => (format_bytes(p.memory_bytes), Some(Style::default())),
            Column::Status => {
                let style = match p.status {
                    ProcessStatus::Running => Style::default().fg(theme::STATUS_RUNNING),
                    ProcessStatus::Sleeping => Style::default().fg(theme::STATUS_SLEEPING),
                    ProcessStatus::Stopped => Style::default().fg(theme::STATUS_STOPPED),
                    ProcessStatus::Zombie => Style::default().fg(theme::STATUS_ZOMBIE),
                    ProcessStatus::Dead => Style::default().fg(Color::DarkGray),
                    ProcessStatus::Unknown => Style::default(),
                };
                (p.status.as_str().to_string(), Some(style))
            }
            Column::Threads => (p.thread_count.to_string(), Some(Style::default())),
            Column::Time => {
                let running_time = now.saturating_sub(p.start_time);
                (format_duration_short(running_time), Some(Style::default()))
            }
            Column::DiskRead => (format_bytes(p.disk_read_bytes), Some(Style::default())),
            Column::DiskWrite => (format_bytes(p.disk_write_bytes), Some(Style::default())),
            Column::Ppid => {
                let text = p.parent_pid.map(|pid| pid.to_string()).unwrap_or_else(|| "-".to_string());
                (text, Some(Style::default()))
            }
            Column::Command => (p.command.clone(), Some(Style::default())),
        }
    }

    fn dead_cell_value(&self, p: &ProcessEntry) -> String {
        match self {
            Column::Pid => format!("* {}", p.pid),
            Column::Name => p.name.clone(),
            Column::User => p.user.clone(),
            Column::Cpu => "-".to_string(),
            Column::Memory => format_bytes(p.memory_bytes),
            Column::Status => "Dead".to_string(),
            Column::Threads => "-".to_string(),
            Column::Time => "-".to_string(),
            Column::DiskRead => "-".to_string(),
            Column::DiskWrite => "-".to_string(),
            Column::Ppid => p.parent_pid.map(|pid| pid.to_string()).unwrap_or_else(|| "-".to_string()),
            Column::Command => p.command.clone(),
        }
    }
}

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    app.process_table_area.set(area);

    let columns = &app.visible_columns;
    let has_pins = app.process_list.has_pins();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let (header, widths) = build_header(columns, app);

    // Build rows: pinned first, separator, then unpinned
    let mut rows: Vec<Row> = Vec::new();

    if has_pins {
        let pinned = app.process_list.pinned_visible();
        for p in &pinned {
            rows.push(build_row(p, now, columns, true));
        }
        // Separator row
        let sep_text = format!("── pinned: {} ──", pinned.len());
        let mut sep_cells: Vec<Cell> = Vec::new();
        for (i, _) in columns.iter().enumerate() {
            if i == 1 {
                sep_cells.push(Cell::from(sep_text.clone()).style(Style::default().fg(Color::DarkGray)));
            } else {
                sep_cells.push(Cell::from(""));
            }
        }
        rows.push(Row::new(sep_cells).style(Style::default().fg(Color::DarkGray)));
    }

    let unpinned = if has_pins {
        app.process_list.unpinned_visible()
    } else {
        app.process_list.visible()
    };
    for p in &unpinned {
        rows.push(build_row(p, now, columns, false));
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

fn build_header(columns: &[Column], app: &App) -> (Row<'static>, Vec<Constraint>) {
    let cells: Vec<Cell> = columns
        .iter()
        .map(|col| {
            if let Some(sort_col) = col.sort_column() {
                column_header(col.header_name(), sort_col, app)
            } else {
                Cell::from(col.header_name().to_string())
            }
        })
        .collect();
    let widths: Vec<Constraint> = columns.iter().map(|col| col.width()).collect();
    (Row::new(cells).style(theme::table_header_style()), widths)
}

fn build_row(p: &ProcessEntry, now: u64, columns: &[Column], is_pinned: bool) -> Row<'static> {
    // Dead pinned processes get a faded gray style
    if p.is_dead {
        let dead_style = Style::default().fg(Color::DarkGray);
        let cells: Vec<Cell> = columns
            .iter()
            .map(|col| Cell::from(col.dead_cell_value(p)).style(dead_style))
            .collect();
        return Row::new(cells);
    }

    let row_style = if is_pinned {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let cells: Vec<Cell> = columns
        .iter()
        .map(|col| {
            let (text, style_override) = col.cell_value(p, now, is_pinned);
            match col {
                Column::Pid | Column::Name => Cell::from(text).style(row_style),
                Column::Status => {
                    Cell::from(text).style(style_override.unwrap_or_default())
                }
                _ => Cell::from(text),
            }
        })
        .collect();

    Row::new(cells)
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
