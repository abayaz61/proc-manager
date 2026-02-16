use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let hidden_names = {
        let mut names: Vec<String> = app.process_list.hidden_names().iter().cloned().collect();
        names.sort();
        names
    };

    let item_count = hidden_names.len();
    let height = (item_count as u16 + 6).min(frame.area().height.saturating_sub(4));
    let area = centered_rect(50, height, frame.area());
    frame.render_widget(Clear, area);

    let inner = Block::default()
        .title(format!(" Hidden Processes ({}) ", item_count))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.palette.swap));

    let inner_area = inner.inner(area);
    frame.render_widget(inner, area);

    // Split inner into list + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(inner_area);

    // List items
    let max_visible = chunks[0].height as usize;
    let selected = app.hidden_list_selected;

    // Scroll offset
    let offset = if selected >= max_visible {
        selected - max_visible + 1
    } else {
        0
    };

    let mut lines: Vec<Line> = Vec::new();
    for (i, name) in hidden_names.iter().enumerate().skip(offset).take(max_visible) {
        let is_selected = i == selected;
        let style = if is_selected {
            app.palette.selected_style()
        } else {
            Style::default().fg(Color::White)
        };
        let indicator = if is_selected { " > " } else { "   " };
        lines.push(Line::from(vec![
            Span::styled(indicator.to_string(), style),
            Span::styled(name.clone(), style),
        ]));
    }

    let list_paragraph = Paragraph::new(lines);
    frame.render_widget(list_paragraph, chunks[0]);

    // Footer
    let key_style = Style::default().fg(app.palette.accent);
    let footer = Line::from(vec![
        Span::styled(" [Enter/Del]", key_style),
        Span::raw(" Unhide  "),
        Span::styled("[a]", key_style),
        Span::raw(" Unhide All  "),
        Span::styled("[Esc/H]", key_style),
        Span::raw(" Close"),
    ]);
    let footer_paragraph = Paragraph::new(vec![Line::from(""), footer]);
    frame.render_widget(footer_paragraph, chunks[1]);
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
