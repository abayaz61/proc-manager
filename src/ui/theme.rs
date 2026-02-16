use ratatui::style::{Color, Modifier, Style};

pub const HEADER_BG: Color = Color::Blue;
pub const HEADER_FG: Color = Color::White;

pub const SELECTED_BG: Color = Color::DarkGray;
pub const SELECTED_FG: Color = Color::White;

pub const BORDER_COLOR: Color = Color::Gray;

pub const CPU_COLOR: Color = Color::Green;
pub const MEMORY_COLOR: Color = Color::Yellow;
pub const SWAP_COLOR: Color = Color::Magenta;
pub const NETWORK_RX_COLOR: Color = Color::Cyan;
pub const NETWORK_TX_COLOR: Color = Color::Red;

pub const STATUS_RUNNING: Color = Color::Green;
pub const STATUS_SLEEPING: Color = Color::Gray;
pub const STATUS_STOPPED: Color = Color::Red;
pub const STATUS_ZOMBIE: Color = Color::Magenta;

pub fn header_style() -> Style {
    Style::default().fg(HEADER_FG).bg(HEADER_BG)
}

pub fn selected_style() -> Style {
    Style::default()
        .fg(SELECTED_FG)
        .bg(SELECTED_BG)
        .add_modifier(Modifier::BOLD)
}

pub fn border_style() -> Style {
    Style::default().fg(BORDER_COLOR)
}

pub fn table_header_style() -> Style {
    Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD)
}
