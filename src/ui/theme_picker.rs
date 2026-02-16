use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::ui::theme::ALL_PALETTES;

pub fn draw(frame: &mut Frame, app: &App) {
    let count = ALL_PALETTES.len() as u16;
    // 1 blank + items + 1 blank + 1 footer + 2 border = items + 5
    let height = count + 5;
    let area = centered_rect(50, height, frame.area());
    frame.render_widget(Clear, area);

    let active_name = app.palette.name;

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for (i, palette) in ALL_PALETTES.iter().enumerate() {
        let selected = i == app.theme_picker_selected;
        let is_active = palette.name == active_name;

        let indicator = if selected { " > " } else { "   " };
        let check = if is_active { " *" } else { "" };

        let name_style = if selected {
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Build color swatch: ██ blocks for each key color
        let swatches = palette.swatch_colors();
        let mut spans = vec![
            Span::styled(indicator.to_string(), name_style),
            Span::styled(format!("{:<14}", palette.name), name_style),
            Span::styled(
                check.to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ];

        // Padding before swatches
        let pad = 16usize.saturating_sub(palette.name.len() + check.len());
        spans.push(Span::raw(" ".repeat(pad)));

        for color in &swatches {
            spans.push(Span::styled(
                "\u{2588}\u{2588}",
                Style::default().fg(*color),
            ));
            spans.push(Span::raw(" "));
        }

        lines.push(Line::from(spans));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  [Esc]", Style::default().fg(Color::DarkGray)),
        Span::raw(" Cancel  "),
        Span::styled("[Enter]", Style::default().fg(Color::DarkGray)),
        Span::raw(" Apply  "),
        Span::styled("*", Style::default().fg(Color::Green)),
        Span::raw(" = active"),
    ]));

    let block = Block::default()
        .title(" Theme ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.palette.accent));

    let paragraph = Paragraph::new(lines).block(block);
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
