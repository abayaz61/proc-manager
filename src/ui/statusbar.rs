use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, AppMode};
use crate::data::process::SortColumn;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let line = match app.mode {
        AppMode::Search => Line::from(vec![
            Span::styled(" Search: ", Style::default().fg(Color::Yellow)),
            Span::raw(&app.search_query),
            Span::styled("_", Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("[Esc]", Style::default().fg(Color::DarkGray)),
            Span::raw(" Cancel"),
        ]),
        AppMode::NewProcess => Line::from(vec![
            Span::styled(" Run: ", Style::default().fg(Color::Yellow)),
            Span::raw(&app.new_process_input),
            Span::styled("_", Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("[Enter]", Style::default().fg(Color::DarkGray)),
            Span::raw(" Run  "),
            Span::styled("[Esc]", Style::default().fg(Color::DarkGray)),
            Span::raw(" Cancel"),
        ]),
        _ => {
            if let Some(msg) = &app.status_message {
                Line::from(vec![
                    Span::styled(" > ", Style::default().fg(Color::Yellow)),
                    Span::styled(msg.as_str(), Style::default().fg(Color::White)),
                ])
            } else {
                let sort_info = sort_indicator(app);
                Line::from(vec![
                    Span::styled(" [q]", Style::default().fg(Color::Cyan)),
                    Span::raw("Quit "),
                    Span::styled("[x]", Style::default().fg(Color::Cyan)),
                    Span::raw("Kill "),
                    Span::styled("[/]", Style::default().fg(Color::Cyan)),
                    Span::raw("Search "),
                    Span::styled("[p]", Style::default().fg(Color::Cyan)),
                    Span::raw("Pin "),
                    Span::styled("[c]", Style::default().fg(Color::Cyan)),
                    Span::raw("Clear "),
                    Span::styled("[h]", Style::default().fg(Color::Cyan)),
                    Span::raw("Hide "),
                    Span::styled("[v]", Style::default().fg(Color::Cyan)),
                    Span::raw("Hidden "),
                    Span::styled("[1-8]", Style::default().fg(Color::Cyan)),
                    Span::raw("Sort "),
                    Span::styled("[n]", Style::default().fg(Color::Cyan)),
                    Span::raw("New "),
                    Span::styled("[s]", Style::default().fg(Color::Cyan)),
                    Span::raw("Settings "),
                    Span::styled("[?]", Style::default().fg(Color::Cyan)),
                    Span::raw("Help "),
                    Span::styled(
                        sort_info,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            }
        }
    };

    let paragraph = Paragraph::new(line).style(Style::default().fg(Color::White));
    frame.render_widget(paragraph, area);
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
    };
    let arrow = if app.process_list.sort_ascending {
        "▲"
    } else {
        "▼"
    };
    format!("Sort: {}{}", col_name, arrow)
}
