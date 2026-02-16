use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, AppMode};
use crate::data::process::SortColumn;


pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let p = &app.palette;
    let key_style = Style::default().fg(p.accent);
    let dim_style = Style::default().fg(p.border);

    let line = match app.mode {
        AppMode::Search => Line::from(vec![
            Span::styled(" Search: ", Style::default().fg(p.pin)),
            Span::raw(&app.search_query),
            Span::styled("_", Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("[Esc]", dim_style),
            Span::raw(" Cancel"),
        ]),
        AppMode::NewProcess => Line::from(vec![
            Span::styled(" Run: ", Style::default().fg(p.pin)),
            Span::raw(&app.new_process_input),
            Span::styled("_", Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("[Enter]", dim_style),
            Span::raw(" Run  "),
            Span::styled("[Esc]", dim_style),
            Span::raw(" Cancel"),
        ]),
        _ => {
            if let Some(msg) = &app.status_message {
                Line::from(vec![
                    Span::styled(" > ", Style::default().fg(p.pin)),
                    Span::styled(msg.as_str(), Style::default().fg(Color::White)),
                ])
            } else {
                let sort_info = sort_indicator(app);
                Line::from(vec![
                    Span::styled(" [q]", key_style),
                    Span::raw("Quit "),
                    Span::styled("[x]", key_style),
                    Span::raw("Kill "),
                    Span::styled("[/]", key_style),
                    Span::raw("Search "),
                    Span::styled("[p]", key_style),
                    Span::raw("Pin "),
                    Span::styled("[c]", key_style),
                    Span::raw("Clear "),
                    Span::styled("[h]", key_style),
                    Span::raw("Hide "),
                    Span::styled("[v]", key_style),
                    Span::raw("Hidden "),
                    Span::styled("[1-0]", key_style),
                    Span::raw("Sort "),
                    Span::styled("[n]", key_style),
                    Span::raw("New "),
                    Span::styled("[d]", key_style),
                    Span::raw("Compact "),
                    Span::styled("[t]", key_style),
                    Span::raw("Theme "),
                    Span::styled("[s]", key_style),
                    Span::raw("Settings "),
                    Span::styled("[?]", key_style),
                    Span::raw("Help "),
                    Span::styled("[Tab]", key_style),
                    Span::raw("View "),
                    Span::styled(
                        view_mode_indicator(app),
                        Style::default()
                            .fg(p.swap)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        sort_info,
                        Style::default()
                            .fg(p.table_header_fg)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            }
        }
    };

    let paragraph = Paragraph::new(line).style(Style::default().fg(Color::White));
    frame.render_widget(paragraph, area);
}

fn view_mode_indicator(app: &App) -> String {
    format!("[{}]", app.view_mode.label())
}

fn sort_indicator(app: &App) -> String {
    let col_name = match app.process_list.sort_column {
        SortColumn::Pid => "PID",
        SortColumn::Name => "Name",
        SortColumn::User => "User",
        SortColumn::Cpu => "CPU",
        SortColumn::Memory => "Mem",
        SortColumn::Status => "Status",
        SortColumn::Threads => "Threads",
        SortColumn::StartTime => "Time",
        SortColumn::DiskRead => "Disk R",
        SortColumn::DiskWrite => "Disk W",
        SortColumn::Ppid => "PPID",
    };
    let arrow = if app.process_list.sort_ascending {
        "▲"
    } else {
        "▼"
    };
    format!("Sort: {}{}", col_name, arrow)
}
