use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

pub fn draw(frame: &mut Frame) {
    let area = centered_rect(60, 33, frame.area());
    frame.render_widget(Clear, area);

    let bold = Style::default().add_modifier(Modifier::BOLD);
    let key_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);

    let text = vec![
        Line::from(Span::styled("Navigation", bold)),
        Line::from(vec![
            Span::styled("  ↑/k ↓/j    ", key_style),
            Span::raw("Scroll up/down"),
        ]),
        Line::from(vec![
            Span::styled("  PgUp/PgDn  ", key_style),
            Span::raw("Page scroll"),
        ]),
        Line::from(vec![
            Span::styled("  Home/End   ", key_style),
            Span::raw("Jump to top/bottom"),
        ]),
        Line::from(""),
        Line::from(Span::styled("Sorting", bold)),
        Line::from(vec![
            Span::styled("  1-0 / F1-F11", key_style),
            Span::raw("Sort by column (0=Disk W, F11=PPID)"),
        ]),
        Line::from(vec![
            Span::styled("  r          ", key_style),
            Span::raw("Reverse sort order"),
        ]),
        Line::from(""),
        Line::from(Span::styled("Pinning & Hiding", bold)),
        Line::from(vec![
            Span::styled("  p          ", key_style),
            Span::raw("Pin/unpin selected process"),
        ]),
        Line::from(vec![
            Span::styled("  c          ", key_style),
            Span::raw("Clear all pins"),
        ]),
        Line::from(vec![
            Span::styled("  h          ", key_style),
            Span::raw("Hide/unhide selected process"),
        ]),
        Line::from(vec![
            Span::styled("  v          ", key_style),
            Span::raw("View hidden processes list"),
        ]),
        Line::from(vec![
            Span::styled("  Right-click", key_style),
            Span::raw("Context menu (Pin/Hide/Kill)"),
        ]),
        Line::from(""),
        Line::from(Span::styled("Actions", bold)),
        Line::from(vec![
            Span::styled("  /          ", key_style),
            Span::raw("Search processes"),
        ]),
        Line::from(vec![
            Span::styled("  x          ", key_style),
            Span::raw("Kill selected process"),
        ]),
        Line::from(vec![
            Span::styled("  n          ", key_style),
            Span::raw("Start new process"),
        ]),
        Line::from(vec![
            Span::styled("  q / Ctrl+C ", key_style),
            Span::raw("Quit"),
        ]),
        Line::from(vec![
            Span::styled("  s          ", key_style),
            Span::raw("Open settings"),
        ]),
        Line::from(vec![
            Span::styled("  ?          ", key_style),
            Span::raw("Toggle this help"),
        ]),
        Line::from(vec![
            Span::styled("  d          ", key_style),
            Span::raw("Toggle compact view (hide dashboard)"),
        ]),
        Line::from(""),
        Line::from(Span::styled("View Modes", bold)),
        Line::from(vec![
            Span::styled("  Tab        ", key_style),
            Span::raw("Cycle view: Processes > Per-CPU > Graphs > Overview > SysInfo"),
        ]),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

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
