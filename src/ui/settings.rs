use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::{App, SettingsItem};
use crate::ui::theme::ColorPalette;

pub fn draw(frame: &mut Frame, app: &App) {
    let state = match &app.settings_state {
        Some(s) => s,
        None => return,
    };

    let p = &app.palette;
    let item_count = state.items.len() as u16;
    let height = item_count + 9;
    let area = centered_rect(60, height, frame.area());
    frame.render_widget(Clear, area);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for (i, item) in state.items.iter().enumerate() {
        let selected = i == state.selected;
        let line = render_item(item, selected, p);
        lines.push(line);
    }

    lines.push(Line::from(""));

    // Description for selected item
    let desc = settings_description(&state.items[state.selected]);
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(desc, Style::default().fg(p.text_secondary)),
    ]));

    lines.push(Line::from(""));

    let dim_style = Style::default().fg(p.border);
    if let Some(status) = &state.status {
        lines.push(Line::from(vec![
            Span::styled("  > ", Style::default().fg(p.pin)),
            Span::styled(status.as_str(), Style::default().fg(Color::White)),
        ]));
    } else {
        lines.push(Line::from(vec![
            Span::styled("  [Esc]", dim_style),
            Span::raw(" Close  "),
            Span::styled("[Ctrl+S]", dim_style),
            Span::raw(" Save & Close  "),
            Span::styled("[←→]", dim_style),
            Span::raw(" Change  "),
            Span::styled("[Enter]", dim_style),
            Span::raw(" Select"),
        ]));
    }

    let block = Block::default()
        .title(" Settings ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(p.accent));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn render_item(item: &SettingsItem, selected: bool, palette: &ColorPalette) -> Line<'static> {
    let indicator = if selected { " > " } else { "   " };
    let sel_style = if selected {
        Style::default()
            .fg(palette.pin)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let val_style = if selected {
        Style::default().fg(palette.accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette.border)
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

fn settings_description(item: &SettingsItem) -> String {
    let label = match item {
        SettingsItem::Slider { label, .. } => label.as_str(),
        SettingsItem::Toggle { label, .. } => label.as_str(),
        SettingsItem::Cycle { label, .. } => label.as_str(),
        SettingsItem::Action { label, .. } => label.as_str(),
    };
    match label {
        "Refresh Rate" => "How often process data is refreshed (lower = more CPU usage)".into(),
        "Default Sort Column" => "Column used to sort the process list by default".into(),
        "Sort Ascending" => "Sort processes in ascending order instead of descending".into(),
        "History Length" => "Number of data points kept for sparkline charts".into(),
        "Colors" => "Enable or disable colored output in the UI".into(),
        "Bold Headers" => "Use bold text for table column headers".into(),
        "Standalone Window" => "Launch in a separate window instead of current terminal".into(),
        "Keep Dead Pins" => "Keep pinned processes visible after they exit".into(),
        "Compact View" => "Hide the dashboard panels to show more processes".into(),
        "Always on Top" => "Keep the window above all other windows".into(),
        "Font" => "Console font face (monospace fonts recommended)".into(),
        "Font Size" => "Console font size in pixels".into(),
        "Change Theme" => "Open the theme picker to change color scheme".into(),
        "Register to PATH" => "Add prc command to system PATH for easy access".into(),
        "Create Desktop Shortcut" => "Create a shortcut on the desktop".into(),
        "Save & Close" => "Save all settings to config file and close".into(),
        _ if label.starts_with("Column: ") => {
            format!("Show or hide the {} column in the process table", &label[8..])
        }
        _ => String::new(),
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
