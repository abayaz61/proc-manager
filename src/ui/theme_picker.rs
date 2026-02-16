use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::ui::theme::ALL_PALETTES;

/// Fixed width for theme name column (longest name is "Tokyo Night" = 11 chars).
const NAME_WIDTH: usize = 14;

pub fn draw(frame: &mut Frame, app: &App) {
    let count = ALL_PALETTES.len() as u16;
    let height = (count + 5).min(frame.area().height.saturating_sub(4));
    let area = centered_rect(60, height, frame.area());
    frame.render_widget(Clear, area);

    let active_name = app.palette.name;

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    // Scrolling support
    let max_visible = (height.saturating_sub(5)) as usize; // borders(2) + blank(1) + blank(1) + footer(1)
    let selected = app.theme_picker_selected;
    let offset = if selected >= max_visible {
        selected - max_visible + 1
    } else {
        0
    };

    for (i, palette) in ALL_PALETTES.iter().enumerate().skip(offset).take(max_visible) {
        let is_selected = i == selected;
        let is_active = palette.name == active_name;

        // Indicator: " > " or "   "
        let indicator = if is_selected { " > " } else { "   " };

        // Active mark: " *" or "  "
        let check = if is_active { " *" } else { "  " };

        let name_style = if is_selected {
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.palette.text)
        };

        let check_style = Style::default()
            .fg(app.palette.cpu)
            .add_modifier(Modifier::BOLD);

        // Pad name to fixed width
        let padded_name = format!("{:<width$}", palette.name, width = NAME_WIDTH);

        let mut spans = vec![
            Span::styled(indicator.to_string(), name_style),
            Span::styled(padded_name, name_style),
            Span::styled(check.to_string(), check_style),
            Span::raw("  "),
        ];

        // Color swatches — always at same column
        let swatches = palette.swatch_colors();
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

    let key_style = Style::default().fg(app.palette.accent);
    lines.push(Line::from(vec![
        Span::styled("  [Esc]", key_style),
        Span::raw(" Cancel  "),
        Span::styled("[Enter]", key_style),
        Span::raw(" Apply  "),
        Span::styled("*", Style::default().fg(app.palette.cpu).add_modifier(Modifier::BOLD)),
        Span::raw(" = active"),
    ]));

    let block = Block::default()
        .title(format!(" Theme ({}/{}) ", selected + 1, ALL_PALETTES.len()))
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
