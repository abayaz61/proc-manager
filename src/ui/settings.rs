use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::{App, SettingsItem};

pub fn draw(frame: &mut Frame, app: &App) {
    let state = match &app.settings_state {
        Some(s) => s,
        None => return,
    };

    let item_count = state.items.len() as u16;
    // 3 lines header + items + 2 lines footer + 2 border = items + 7
    let height = item_count + 7;
    let area = centered_rect(60, height, frame.area());
    frame.render_widget(Clear, area);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for (i, item) in state.items.iter().enumerate() {
        let selected = i == state.selected;
        let line = render_item(item, selected);
        lines.push(line);
    }

    lines.push(Line::from(""));

    // Status message if any
    if let Some(status) = &state.status {
        lines.push(Line::from(vec![
            Span::styled("  > ", Style::default().fg(Color::Yellow)),
            Span::styled(status.as_str(), Style::default().fg(Color::White)),
        ]));
    } else {
        lines.push(Line::from(vec![
            Span::styled("  [Esc]", Style::default().fg(Color::DarkGray)),
            Span::raw(" Close  "),
            Span::styled("[Ctrl+S]", Style::default().fg(Color::DarkGray)),
            Span::raw(" Save & Close  "),
            Span::styled("[←→]", Style::default().fg(Color::DarkGray)),
            Span::raw(" Change  "),
            Span::styled("[Enter]", Style::default().fg(Color::DarkGray)),
            Span::raw(" Select"),
        ]));
    }

    let block = Block::default()
        .title(" Settings ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn render_item(item: &SettingsItem, selected: bool) -> Line<'static> {
    let indicator = if selected { " > " } else { "   " };
    let sel_style = if selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let val_style = if selected {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    match item {
        SettingsItem::Slider {
            label,
            value,
            unit,
            ..
        } => {
            let value_str = if unit.is_empty() {
                format!("< {} >", value)
            } else {
                format!("< {}{} >", value, unit)
            };
            let padding = 40usize.saturating_sub(label.len() + indicator.len());
            Line::from(vec![
                Span::styled(indicator.to_string(), sel_style),
                Span::styled(label.clone(), sel_style),
                Span::raw(" ".repeat(padding)),
                Span::styled(value_str, val_style),
            ])
        }
        SettingsItem::Toggle { label, value } => {
            let value_str = if *value {
                "< ON >"
            } else {
                "< OFF >"
            };
            let padding = 40usize.saturating_sub(label.len() + indicator.len());
            Line::from(vec![
                Span::styled(indicator.to_string(), sel_style),
                Span::styled(label.clone(), sel_style),
                Span::raw(" ".repeat(padding)),
                Span::styled(
                    value_str.to_string(),
                    if *value {
                        val_style.fg(Color::Green)
                    } else {
                        val_style.fg(Color::Red)
                    },
                ),
            ])
        }
        SettingsItem::Cycle {
            label,
            options,
            selected: sel_idx,
        } => {
            let value_str = format!("< {} >", options[*sel_idx]);
            let padding = 40usize.saturating_sub(label.len() + indicator.len());
            Line::from(vec![
                Span::styled(indicator.to_string(), sel_style),
                Span::styled(label.clone(), sel_style),
                Span::raw(" ".repeat(padding)),
                Span::styled(value_str, val_style),
            ])
        }
        SettingsItem::Action {
            label,
            description,
        } => {
            let padding = 40usize.saturating_sub(label.len() + indicator.len());
            Line::from(vec![
                Span::styled(indicator.to_string(), sel_style),
                Span::styled(label.clone(), sel_style),
                Span::raw(" ".repeat(padding)),
                Span::styled(
                    description.clone(),
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        }
    }
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
