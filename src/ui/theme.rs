use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub name: &'static str,

    // Header bar
    pub header_bg: Color,
    pub header_fg: Color,

    // Selection highlight
    pub selected_bg: Color,
    pub selected_fg: Color,

    // Borders
    pub border: Color,

    // Dashboard panels
    pub cpu: Color,
    pub memory: Color,
    pub swap: Color,
    pub network_rx: Color,
    pub network_tx: Color,
    pub disk_read: Color,
    pub disk_write: Color,

    // Process status
    pub status_running: Color,
    pub status_sleeping: Color,
    pub status_stopped: Color,
    pub status_zombie: Color,

    // Table
    pub table_header_fg: Color,

    // Pinned process
    pub pin: Color,

    // General accent
    pub accent: Color,
}

impl ColorPalette {
    pub fn header_style(&self) -> Style {
        Style::default().fg(self.header_fg).bg(self.header_bg)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.selected_fg)
            .bg(self.selected_bg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    pub fn table_header_style(&self) -> Style {
        Style::default()
            .fg(self.table_header_fg)
            .add_modifier(Modifier::BOLD)
    }

    /// Key colors for the theme preview swatches.
    pub fn swatch_colors(&self) -> [Color; 6] {
        [self.cpu, self.memory, self.network_rx, self.network_tx, self.accent, self.pin]
    }
}

pub const ALL_PALETTES: &[ColorPalette] = &[
    PALETTE_DEFAULT,
    PALETTE_DRACULA,
    PALETTE_NORD,
    PALETTE_MONOKAI,
    PALETTE_SOLARIZED,
    PALETTE_GRUVBOX,
    PALETTE_TOKYO_NIGHT,
    PALETTE_CATPPUCCIN,
];

pub fn by_name(name: &str) -> ColorPalette {
    ALL_PALETTES
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(name))
        .copied()
        .unwrap_or(PALETTE_DEFAULT)
}

// ── Default ─────────────────────────────────────────────────────────────────

pub const PALETTE_DEFAULT: ColorPalette = ColorPalette {
    name: "Default",
    header_bg: Color::Blue,
    header_fg: Color::White,
    selected_bg: Color::DarkGray,
    selected_fg: Color::White,
    border: Color::Gray,
    cpu: Color::Green,
    memory: Color::Yellow,
    swap: Color::Magenta,
    network_rx: Color::Cyan,
    network_tx: Color::Red,
    disk_read: Color::Green,
    disk_write: Color::Red,
    status_running: Color::Green,
    status_sleeping: Color::Gray,
    status_stopped: Color::Red,
    status_zombie: Color::Magenta,
    table_header_fg: Color::Yellow,
    pin: Color::Yellow,
    accent: Color::Cyan,
};

// ── Dracula ─────────────────────────────────────────────────────────────────

pub const PALETTE_DRACULA: ColorPalette = ColorPalette {
    name: "Dracula",
    header_bg: Color::Rgb(68, 71, 90),
    header_fg: Color::Rgb(248, 248, 242),
    selected_bg: Color::Rgb(68, 71, 90),
    selected_fg: Color::Rgb(248, 248, 242),
    border: Color::Rgb(98, 114, 164),
    cpu: Color::Rgb(80, 250, 123),
    memory: Color::Rgb(189, 147, 249),
    swap: Color::Rgb(255, 121, 198),
    network_rx: Color::Rgb(139, 233, 253),
    network_tx: Color::Rgb(255, 85, 85),
    disk_read: Color::Rgb(80, 250, 123),
    disk_write: Color::Rgb(255, 85, 85),
    status_running: Color::Rgb(80, 250, 123),
    status_sleeping: Color::Rgb(98, 114, 164),
    status_stopped: Color::Rgb(255, 85, 85),
    status_zombie: Color::Rgb(255, 121, 198),
    table_header_fg: Color::Rgb(255, 184, 108),
    pin: Color::Rgb(241, 250, 140),
    accent: Color::Rgb(189, 147, 249),
};

// ── Nord ────────────────────────────────────────────────────────────────────

pub const PALETTE_NORD: ColorPalette = ColorPalette {
    name: "Nord",
    header_bg: Color::Rgb(59, 66, 82),
    header_fg: Color::Rgb(236, 239, 244),
    selected_bg: Color::Rgb(67, 76, 94),
    selected_fg: Color::Rgb(236, 239, 244),
    border: Color::Rgb(76, 86, 106),
    cpu: Color::Rgb(163, 190, 140),
    memory: Color::Rgb(235, 203, 139),
    swap: Color::Rgb(180, 142, 173),
    network_rx: Color::Rgb(136, 192, 208),
    network_tx: Color::Rgb(191, 97, 106),
    disk_read: Color::Rgb(163, 190, 140),
    disk_write: Color::Rgb(191, 97, 106),
    status_running: Color::Rgb(163, 190, 140),
    status_sleeping: Color::Rgb(76, 86, 106),
    status_stopped: Color::Rgb(191, 97, 106),
    status_zombie: Color::Rgb(180, 142, 173),
    table_header_fg: Color::Rgb(129, 161, 193),
    pin: Color::Rgb(235, 203, 139),
    accent: Color::Rgb(136, 192, 208),
};

// ── Monokai ─────────────────────────────────────────────────────────────────

pub const PALETTE_MONOKAI: ColorPalette = ColorPalette {
    name: "Monokai",
    header_bg: Color::Rgb(73, 72, 62),
    header_fg: Color::Rgb(248, 248, 242),
    selected_bg: Color::Rgb(73, 72, 62),
    selected_fg: Color::Rgb(248, 248, 242),
    border: Color::Rgb(117, 113, 94),
    cpu: Color::Rgb(166, 226, 46),
    memory: Color::Rgb(230, 219, 116),
    swap: Color::Rgb(174, 129, 255),
    network_rx: Color::Rgb(102, 217, 239),
    network_tx: Color::Rgb(249, 38, 114),
    disk_read: Color::Rgb(166, 226, 46),
    disk_write: Color::Rgb(249, 38, 114),
    status_running: Color::Rgb(166, 226, 46),
    status_sleeping: Color::Rgb(117, 113, 94),
    status_stopped: Color::Rgb(249, 38, 114),
    status_zombie: Color::Rgb(174, 129, 255),
    table_header_fg: Color::Rgb(253, 151, 31),
    pin: Color::Rgb(230, 219, 116),
    accent: Color::Rgb(102, 217, 239),
};

// ── Solarized Dark ──────────────────────────────────────────────────────────

pub const PALETTE_SOLARIZED: ColorPalette = ColorPalette {
    name: "Solarized",
    header_bg: Color::Rgb(7, 54, 66),
    header_fg: Color::Rgb(147, 161, 161),
    selected_bg: Color::Rgb(0, 43, 54),
    selected_fg: Color::Rgb(238, 232, 213),
    border: Color::Rgb(88, 110, 117),
    cpu: Color::Rgb(133, 153, 0),
    memory: Color::Rgb(181, 137, 0),
    swap: Color::Rgb(211, 54, 130),
    network_rx: Color::Rgb(42, 161, 152),
    network_tx: Color::Rgb(220, 50, 47),
    disk_read: Color::Rgb(133, 153, 0),
    disk_write: Color::Rgb(220, 50, 47),
    status_running: Color::Rgb(133, 153, 0),
    status_sleeping: Color::Rgb(88, 110, 117),
    status_stopped: Color::Rgb(220, 50, 47),
    status_zombie: Color::Rgb(211, 54, 130),
    table_header_fg: Color::Rgb(38, 139, 210),
    pin: Color::Rgb(181, 137, 0),
    accent: Color::Rgb(38, 139, 210),
};

// ── Gruvbox ─────────────────────────────────────────────────────────────────

pub const PALETTE_GRUVBOX: ColorPalette = ColorPalette {
    name: "Gruvbox",
    header_bg: Color::Rgb(60, 56, 54),
    header_fg: Color::Rgb(235, 219, 178),
    selected_bg: Color::Rgb(80, 73, 69),
    selected_fg: Color::Rgb(235, 219, 178),
    border: Color::Rgb(146, 131, 116),
    cpu: Color::Rgb(184, 187, 38),
    memory: Color::Rgb(250, 189, 47),
    swap: Color::Rgb(211, 134, 155),
    network_rx: Color::Rgb(131, 165, 152),
    network_tx: Color::Rgb(251, 73, 52),
    disk_read: Color::Rgb(184, 187, 38),
    disk_write: Color::Rgb(251, 73, 52),
    status_running: Color::Rgb(184, 187, 38),
    status_sleeping: Color::Rgb(146, 131, 116),
    status_stopped: Color::Rgb(251, 73, 52),
    status_zombie: Color::Rgb(211, 134, 155),
    table_header_fg: Color::Rgb(254, 128, 25),
    pin: Color::Rgb(250, 189, 47),
    accent: Color::Rgb(142, 192, 124),
};

// ── Tokyo Night ─────────────────────────────────────────────────────────────

pub const PALETTE_TOKYO_NIGHT: ColorPalette = ColorPalette {
    name: "Tokyo Night",
    header_bg: Color::Rgb(36, 40, 59),
    header_fg: Color::Rgb(192, 202, 245),
    selected_bg: Color::Rgb(41, 46, 66),
    selected_fg: Color::Rgb(192, 202, 245),
    border: Color::Rgb(59, 66, 97),
    cpu: Color::Rgb(158, 206, 106),
    memory: Color::Rgb(224, 175, 104),
    swap: Color::Rgb(187, 154, 247),
    network_rx: Color::Rgb(125, 207, 255),
    network_tx: Color::Rgb(247, 118, 142),
    disk_read: Color::Rgb(158, 206, 106),
    disk_write: Color::Rgb(247, 118, 142),
    status_running: Color::Rgb(158, 206, 106),
    status_sleeping: Color::Rgb(59, 66, 97),
    status_stopped: Color::Rgb(247, 118, 142),
    status_zombie: Color::Rgb(187, 154, 247),
    table_header_fg: Color::Rgb(122, 162, 247),
    pin: Color::Rgb(224, 175, 104),
    accent: Color::Rgb(122, 162, 247),
};

// ── Catppuccin Mocha ────────────────────────────────────────────────────────

pub const PALETTE_CATPPUCCIN: ColorPalette = ColorPalette {
    name: "Catppuccin",
    header_bg: Color::Rgb(49, 50, 68),
    header_fg: Color::Rgb(205, 214, 244),
    selected_bg: Color::Rgb(69, 71, 90),
    selected_fg: Color::Rgb(205, 214, 244),
    border: Color::Rgb(108, 112, 134),
    cpu: Color::Rgb(166, 227, 161),
    memory: Color::Rgb(249, 226, 175),
    swap: Color::Rgb(203, 166, 247),
    network_rx: Color::Rgb(148, 226, 213),
    network_tx: Color::Rgb(243, 139, 168),
    disk_read: Color::Rgb(166, 227, 161),
    disk_write: Color::Rgb(243, 139, 168),
    status_running: Color::Rgb(166, 227, 161),
    status_sleeping: Color::Rgb(108, 112, 134),
    status_stopped: Color::Rgb(243, 139, 168),
    status_zombie: Color::Rgb(203, 166, 247),
    table_header_fg: Color::Rgb(137, 180, 250),
    pin: Color::Rgb(249, 226, 175),
    accent: Color::Rgb(137, 180, 250),
};
