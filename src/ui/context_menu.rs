use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let menu = match &app.context_menu {
        Some(m) => m,
        None => return,
    };

    let item_count = menu.items.len() as u16;
    let width = 22u16;
    let height = item_count + 2; // borders

    // Position menu at click location, clamped to screen
    let screen = frame.area();
    let x = menu.x.min(screen.width.saturating_sub(width));
    let y = menu.y.min(screen.height.saturating_sub(height));

    let area = Rect::new(x, y, width, height);
    frame.render_widget(Clear, area);

    let mut lines: Vec<Line> = Vec::new();
    for (i, item) in menu.items.iter().enumerate() {
        let selected = i == menu.selected;
        let style = if selected {
            app.palette.selected_style()
        } else {
            Style::default().fg(Color::White)
        };
        let indicator = if selected { " > " } else { "   " };
        lines.push(Line::from(vec![
            Span::styled(indicator.to_string(), style),
            Span::styled(item.label.clone(), style),
        ]));
    }

    let title = format!(" {} ", truncate_name(&menu.target_name, 16));
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.palette.accent));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn truncate_name(name: &str, max: usize) -> String {
    if name.len() <= max {
        name.to_string()
    } else {
        format!("{}...", &name[..max.saturating_sub(3)])
    }
}
