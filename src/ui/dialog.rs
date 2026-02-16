use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn draw_kill_confirm(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 7, frame.area());
    frame.render_widget(Clear, area);

    let pid_info = if let Some(p) = app.selected_process() {
        format!("{} (PID: {})", p.name, p.pid)
    } else {
        "unknown".to_string()
    };

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Kill process "),
            Span::styled(&pid_info, Style::default().fg(app.palette.status_stopped).add_modifier(Modifier::BOLD)),
            Span::raw("?"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [y]", Style::default().fg(app.palette.status_running)),
            Span::raw(" Yes  "),
            Span::styled("[n/Esc]", Style::default().fg(app.palette.status_stopped)),
            Span::raw(" No"),
        ]),
    ];

    let block = Block::default()
        .title(" Confirm Kill ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.palette.status_stopped));

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

pub fn draw_new_process(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 7, frame.area());
    frame.render_widget(Clear, area);

    let text = vec![
        Line::from(""),
        Line::from("  Enter command to run:"),
        Line::from(""),
        Line::from(vec![
            Span::raw("  > "),
            Span::raw(&app.new_process_input),
            Span::styled("_", Style::default().fg(Color::White)),
        ]),
    ];

    let block = Block::default()
        .title(" Start New Process ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.palette.accent));

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}
