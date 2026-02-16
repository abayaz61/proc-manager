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

    // Text
    pub text: Color,
    pub text_secondary: Color,
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
    PALETTE_ONE_DARK,
    PALETTE_EVERFOREST,
    PALETTE_KANAGAWA,
    PALETTE_ROSE_PINE,
    PALETTE_SYNTHWAVE,
    PALETTE_AYU_DARK,
    PALETTE_MATERIAL,
    PALETTE_CYBERPUNK,
];

pub fn by_name(name: &str) -> ColorPalette {
    ALL_PALETTES
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(name))
        .copied()
        .unwrap_or(PALETTE_DEFAULT)
}

// ── Default — classic terminal: green/yellow/cyan/red ───────────────────────

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
    disk_read: Color::Blue,
    disk_write: Color::Red,
    status_running: Color::Green,
    status_sleeping: Color::Gray,
    status_stopped: Color::Red,
    status_zombie: Color::Magenta,
    table_header_fg: Color::Yellow,
    pin: Color::Yellow,
    accent: Color::Cyan,
    text: Color::White,
    text_secondary: Color::Gray,
};

// ── Dracula — purple/pink dominant, green+cyan accents ─────────────────────

pub const PALETTE_DRACULA: ColorPalette = ColorPalette {
    name: "Dracula",
    header_bg: Color::Rgb(68, 71, 90),
    header_fg: Color::Rgb(248, 248, 242),
    selected_bg: Color::Rgb(68, 71, 90),
    selected_fg: Color::Rgb(248, 248, 242),
    border: Color::Rgb(98, 114, 164),
    cpu: Color::Rgb(189, 147, 249),     // purple
    memory: Color::Rgb(255, 121, 198),  // pink
    swap: Color::Rgb(241, 250, 140),    // yellow
    network_rx: Color::Rgb(80, 250, 123),  // green
    network_tx: Color::Rgb(255, 85, 85),   // red
    disk_read: Color::Rgb(139, 233, 253),  // cyan
    disk_write: Color::Rgb(255, 184, 108), // orange
    status_running: Color::Rgb(80, 250, 123),
    status_sleeping: Color::Rgb(98, 114, 164),
    status_stopped: Color::Rgb(255, 85, 85),
    status_zombie: Color::Rgb(255, 121, 198),
    table_header_fg: Color::Rgb(139, 233, 253),
    pin: Color::Rgb(241, 250, 140),
    accent: Color::Rgb(189, 147, 249),
    text: Color::Rgb(248, 248, 242),
    text_secondary: Color::Rgb(158, 154, 180),
};

// ── Nord — cool frost blues, aurora greens, muted warmth ───────────────────

pub const PALETTE_NORD: ColorPalette = ColorPalette {
    name: "Nord",
    header_bg: Color::Rgb(59, 66, 82),
    header_fg: Color::Rgb(236, 239, 244),
    selected_bg: Color::Rgb(67, 76, 94),
    selected_fg: Color::Rgb(236, 239, 244),
    border: Color::Rgb(76, 86, 106),
    cpu: Color::Rgb(136, 192, 208),     // frost blue
    memory: Color::Rgb(180, 142, 173),  // purple
    swap: Color::Rgb(163, 190, 140),    // green
    network_rx: Color::Rgb(235, 203, 139), // yellow
    network_tx: Color::Rgb(191, 97, 106),  // red
    disk_read: Color::Rgb(129, 161, 193),  // steel blue
    disk_write: Color::Rgb(208, 135, 112), // orange
    status_running: Color::Rgb(163, 190, 140),
    status_sleeping: Color::Rgb(76, 86, 106),
    status_stopped: Color::Rgb(191, 97, 106),
    status_zombie: Color::Rgb(180, 142, 173),
    table_header_fg: Color::Rgb(235, 203, 139),
    pin: Color::Rgb(208, 135, 112),
    accent: Color::Rgb(136, 192, 208),
    text: Color::Rgb(236, 239, 244),
    text_secondary: Color::Rgb(216, 222, 233),
};

// ── Monokai — hot pink/orange dominant, lime+cyan accents ──────────────────

pub const PALETTE_MONOKAI: ColorPalette = ColorPalette {
    name: "Monokai",
    header_bg: Color::Rgb(73, 72, 62),
    header_fg: Color::Rgb(248, 248, 242),
    selected_bg: Color::Rgb(73, 72, 62),
    selected_fg: Color::Rgb(248, 248, 242),
    border: Color::Rgb(117, 113, 94),
    cpu: Color::Rgb(253, 151, 31),      // orange
    memory: Color::Rgb(249, 38, 114),   // hot pink
    swap: Color::Rgb(102, 217, 239),    // cyan
    network_rx: Color::Rgb(166, 226, 46),  // lime
    network_tx: Color::Rgb(174, 129, 255), // purple
    disk_read: Color::Rgb(230, 219, 116),  // yellow
    disk_write: Color::Rgb(249, 38, 114),  // hot pink
    status_running: Color::Rgb(166, 226, 46),
    status_sleeping: Color::Rgb(117, 113, 94),
    status_stopped: Color::Rgb(249, 38, 114),
    status_zombie: Color::Rgb(174, 129, 255),
    table_header_fg: Color::Rgb(102, 217, 239),
    pin: Color::Rgb(253, 151, 31),
    accent: Color::Rgb(166, 226, 46),
    text: Color::Rgb(248, 248, 242),
    text_secondary: Color::Rgb(168, 167, 150),
};

// ── Solarized — teal/blue dominant, warm yellow/orange accents ─────────────

pub const PALETTE_SOLARIZED: ColorPalette = ColorPalette {
    name: "Solarized",
    header_bg: Color::Rgb(7, 54, 66),
    header_fg: Color::Rgb(147, 161, 161),
    selected_bg: Color::Rgb(0, 43, 54),
    selected_fg: Color::Rgb(238, 232, 213),
    border: Color::Rgb(88, 110, 117),
    cpu: Color::Rgb(42, 161, 152),      // teal
    memory: Color::Rgb(38, 139, 210),   // blue
    swap: Color::Rgb(211, 54, 130),     // magenta
    network_rx: Color::Rgb(181, 137, 0),   // yellow
    network_tx: Color::Rgb(220, 50, 47),   // red
    disk_read: Color::Rgb(133, 153, 0),    // olive green
    disk_write: Color::Rgb(203, 75, 22),   // orange
    status_running: Color::Rgb(133, 153, 0),
    status_sleeping: Color::Rgb(88, 110, 117),
    status_stopped: Color::Rgb(220, 50, 47),
    status_zombie: Color::Rgb(211, 54, 130),
    table_header_fg: Color::Rgb(181, 137, 0),
    pin: Color::Rgb(203, 75, 22),
    accent: Color::Rgb(42, 161, 152),
    text: Color::Rgb(131, 148, 150),
    text_secondary: Color::Rgb(101, 123, 131),
};

// ── Gruvbox — warm orange/red dominant, aqua+yellow accents ────────────────

pub const PALETTE_GRUVBOX: ColorPalette = ColorPalette {
    name: "Gruvbox",
    header_bg: Color::Rgb(60, 56, 54),
    header_fg: Color::Rgb(235, 219, 178),
    selected_bg: Color::Rgb(80, 73, 69),
    selected_fg: Color::Rgb(235, 219, 178),
    border: Color::Rgb(146, 131, 116),
    cpu: Color::Rgb(254, 128, 25),      // bright orange
    memory: Color::Rgb(250, 189, 47),   // bright yellow
    swap: Color::Rgb(211, 134, 155),    // pink
    network_rx: Color::Rgb(131, 165, 152), // aqua
    network_tx: Color::Rgb(251, 73, 52),   // bright red
    disk_read: Color::Rgb(184, 187, 38),   // yellow-green
    disk_write: Color::Rgb(204, 36, 29),   // dark red
    status_running: Color::Rgb(184, 187, 38),
    status_sleeping: Color::Rgb(146, 131, 116),
    status_stopped: Color::Rgb(251, 73, 52),
    status_zombie: Color::Rgb(211, 134, 155),
    table_header_fg: Color::Rgb(131, 165, 152),
    pin: Color::Rgb(250, 189, 47),
    accent: Color::Rgb(254, 128, 25),
    text: Color::Rgb(235, 219, 178),
    text_secondary: Color::Rgb(189, 174, 147),
};

// ── Tokyo Night — deep indigo/blue dominant, bright contrasts ──────────────

pub const PALETTE_TOKYO_NIGHT: ColorPalette = ColorPalette {
    name: "Tokyo Night",
    header_bg: Color::Rgb(36, 40, 59),
    header_fg: Color::Rgb(192, 202, 245),
    selected_bg: Color::Rgb(41, 46, 66),
    selected_fg: Color::Rgb(192, 202, 245),
    border: Color::Rgb(59, 66, 97),
    cpu: Color::Rgb(122, 162, 247),     // bright blue
    memory: Color::Rgb(187, 154, 247),  // purple
    swap: Color::Rgb(247, 118, 142),    // pink
    network_rx: Color::Rgb(158, 206, 106), // green
    network_tx: Color::Rgb(255, 158, 100), // orange
    disk_read: Color::Rgb(125, 207, 255),  // sky blue
    disk_write: Color::Rgb(247, 118, 142), // pink
    status_running: Color::Rgb(158, 206, 106),
    status_sleeping: Color::Rgb(59, 66, 97),
    status_stopped: Color::Rgb(247, 118, 142),
    status_zombie: Color::Rgb(187, 154, 247),
    table_header_fg: Color::Rgb(224, 175, 104),
    pin: Color::Rgb(224, 175, 104),
    accent: Color::Rgb(122, 162, 247),
    text: Color::Rgb(192, 202, 245),
    text_secondary: Color::Rgb(130, 140, 186),
};

// ── Catppuccin — pastel pink/lavender, teal/peach accents ──────────────────

pub const PALETTE_CATPPUCCIN: ColorPalette = ColorPalette {
    name: "Catppuccin",
    header_bg: Color::Rgb(49, 50, 68),
    header_fg: Color::Rgb(205, 214, 244),
    selected_bg: Color::Rgb(69, 71, 90),
    selected_fg: Color::Rgb(205, 214, 244),
    border: Color::Rgb(108, 112, 134),
    cpu: Color::Rgb(243, 139, 168),     // pink
    memory: Color::Rgb(203, 166, 247),  // lavender
    swap: Color::Rgb(166, 227, 161),    // green
    network_rx: Color::Rgb(148, 226, 213), // teal
    network_tx: Color::Rgb(250, 179, 135), // peach
    disk_read: Color::Rgb(137, 180, 250),  // blue
    disk_write: Color::Rgb(245, 194, 231), // flamingo
    status_running: Color::Rgb(166, 227, 161),
    status_sleeping: Color::Rgb(108, 112, 134),
    status_stopped: Color::Rgb(243, 139, 168),
    status_zombie: Color::Rgb(203, 166, 247),
    table_header_fg: Color::Rgb(148, 226, 213),
    pin: Color::Rgb(250, 179, 135),
    accent: Color::Rgb(203, 166, 247),
    text: Color::Rgb(205, 214, 244),
    text_secondary: Color::Rgb(166, 173, 200),
};

// ── One Dark — blue/cyan dominant, warm orange/red accents ─────────────────

pub const PALETTE_ONE_DARK: ColorPalette = ColorPalette {
    name: "One Dark",
    header_bg: Color::Rgb(40, 44, 52),
    header_fg: Color::Rgb(171, 178, 191),
    selected_bg: Color::Rgb(50, 56, 66),
    selected_fg: Color::Rgb(220, 223, 228),
    border: Color::Rgb(76, 82, 99),
    cpu: Color::Rgb(97, 175, 239),      // blue
    memory: Color::Rgb(198, 120, 221),  // purple
    swap: Color::Rgb(229, 192, 123),    // yellow
    network_rx: Color::Rgb(86, 182, 194),  // cyan
    network_tx: Color::Rgb(224, 108, 117), // red
    disk_read: Color::Rgb(152, 195, 121),  // green
    disk_write: Color::Rgb(190, 80, 70),   // dark red
    status_running: Color::Rgb(152, 195, 121),
    status_sleeping: Color::Rgb(76, 82, 99),
    status_stopped: Color::Rgb(224, 108, 117),
    status_zombie: Color::Rgb(198, 120, 221),
    table_header_fg: Color::Rgb(229, 192, 123),
    pin: Color::Rgb(86, 182, 194),
    accent: Color::Rgb(97, 175, 239),
    text: Color::Rgb(171, 178, 191),
    text_secondary: Color::Rgb(127, 132, 142),
};

// ── Everforest — earthy greens, warm amber/coral accents ───────────────────

pub const PALETTE_EVERFOREST: ColorPalette = ColorPalette {
    name: "Everforest",
    header_bg: Color::Rgb(45, 51, 48),
    header_fg: Color::Rgb(211, 198, 170),
    selected_bg: Color::Rgb(55, 63, 58),
    selected_fg: Color::Rgb(211, 198, 170),
    border: Color::Rgb(90, 101, 94),
    cpu: Color::Rgb(167, 192, 128),     // sage green
    memory: Color::Rgb(219, 188, 127),  // warm amber
    swap: Color::Rgb(214, 153, 182),    // rose
    network_rx: Color::Rgb(230, 126, 128), // coral
    network_tx: Color::Rgb(131, 192, 179), // aqua
    disk_read: Color::Rgb(210, 174, 110),  // gold
    disk_write: Color::Rgb(178, 100, 100), // muted red
    status_running: Color::Rgb(167, 192, 128),
    status_sleeping: Color::Rgb(90, 101, 94),
    status_stopped: Color::Rgb(230, 126, 128),
    status_zombie: Color::Rgb(214, 153, 182),
    table_header_fg: Color::Rgb(131, 192, 179),
    pin: Color::Rgb(230, 126, 128),
    accent: Color::Rgb(167, 192, 128),
    text: Color::Rgb(211, 198, 170),
    text_secondary: Color::Rgb(157, 149, 131),
};

// ── Kanagawa — deep indigo/wave blue, cherry red/gold accents ──────────────

pub const PALETTE_KANAGAWA: ColorPalette = ColorPalette {
    name: "Kanagawa",
    header_bg: Color::Rgb(34, 34, 50),
    header_fg: Color::Rgb(220, 215, 186),
    selected_bg: Color::Rgb(54, 54, 74),
    selected_fg: Color::Rgb(220, 215, 186),
    border: Color::Rgb(84, 84, 109),
    cpu: Color::Rgb(127, 160, 223),     // wave blue
    memory: Color::Rgb(226, 194, 124),  // gold
    swap: Color::Rgb(212, 140, 181),    // sakura pink
    network_rx: Color::Rgb(152, 187, 108), // spring green
    network_tx: Color::Rgb(195, 64, 67),   // cherry red
    disk_read: Color::Rgb(126, 196, 193),  // teal
    disk_write: Color::Rgb(255, 94, 94),   // bright red
    status_running: Color::Rgb(152, 187, 108),
    status_sleeping: Color::Rgb(84, 84, 109),
    status_stopped: Color::Rgb(195, 64, 67),
    status_zombie: Color::Rgb(212, 140, 181),
    table_header_fg: Color::Rgb(212, 140, 181),
    pin: Color::Rgb(255, 166, 77),
    accent: Color::Rgb(127, 160, 223),
    text: Color::Rgb(220, 215, 186),
    text_secondary: Color::Rgb(149, 146, 131),
};

// ── Rosé Pine — dusty rose/iris purple, gold/foam accents ──────────────────

pub const PALETTE_ROSE_PINE: ColorPalette = ColorPalette {
    name: "Rose Pine",
    header_bg: Color::Rgb(31, 29, 46),
    header_fg: Color::Rgb(224, 222, 244),
    selected_bg: Color::Rgb(38, 35, 58),
    selected_fg: Color::Rgb(224, 222, 244),
    border: Color::Rgb(110, 106, 134),
    cpu: Color::Rgb(235, 111, 146),     // love (rose)
    memory: Color::Rgb(196, 167, 231),  // iris (purple)
    swap: Color::Rgb(246, 193, 119),    // gold
    network_rx: Color::Rgb(156, 207, 216), // foam (teal)
    network_tx: Color::Rgb(234, 154, 151), // rose red
    disk_read: Color::Rgb(62, 143, 176),   // pine (deep teal)
    disk_write: Color::Rgb(235, 111, 146), // love
    status_running: Color::Rgb(156, 207, 216),
    status_sleeping: Color::Rgb(110, 106, 134),
    status_stopped: Color::Rgb(235, 111, 146),
    status_zombie: Color::Rgb(196, 167, 231),
    table_header_fg: Color::Rgb(246, 193, 119),
    pin: Color::Rgb(156, 207, 216),
    accent: Color::Rgb(235, 111, 146),
    text: Color::Rgb(224, 222, 244),
    text_secondary: Color::Rgb(144, 140, 170),
};

// ── Synthwave — neon magenta/cyan, hot pink/electric yellow ────────────────

pub const PALETTE_SYNTHWAVE: ColorPalette = ColorPalette {
    name: "Synthwave",
    header_bg: Color::Rgb(36, 23, 52),
    header_fg: Color::Rgb(255, 255, 255),
    selected_bg: Color::Rgb(52, 34, 75),
    selected_fg: Color::Rgb(255, 255, 255),
    border: Color::Rgb(97, 67, 133),
    cpu: Color::Rgb(255, 0, 255),       // magenta
    memory: Color::Rgb(54, 243, 243),   // cyan
    swap: Color::Rgb(255, 230, 109),    // yellow
    network_rx: Color::Rgb(114, 242, 114), // neon green
    network_tx: Color::Rgb(254, 78, 149),  // hot pink
    disk_read: Color::Rgb(230, 158, 255),  // lavender
    disk_write: Color::Rgb(255, 106, 60),  // neon orange
    status_running: Color::Rgb(114, 242, 114),
    status_sleeping: Color::Rgb(97, 67, 133),
    status_stopped: Color::Rgb(254, 78, 149),
    status_zombie: Color::Rgb(255, 0, 255),
    table_header_fg: Color::Rgb(54, 243, 243),
    pin: Color::Rgb(255, 230, 109),
    accent: Color::Rgb(230, 158, 255),
    text: Color::Rgb(255, 255, 255),
    text_secondary: Color::Rgb(179, 157, 204),
};

// ── Ayu Dark — amber/orange dominant, bright cyan/blue contrasts ───────────

pub const PALETTE_AYU_DARK: ColorPalette = ColorPalette {
    name: "Ayu Dark",
    header_bg: Color::Rgb(15, 20, 25),
    header_fg: Color::Rgb(203, 204, 198),
    selected_bg: Color::Rgb(27, 35, 43),
    selected_fg: Color::Rgb(230, 231, 226),
    border: Color::Rgb(59, 73, 86),
    cpu: Color::Rgb(255, 180, 84),      // orange
    memory: Color::Rgb(90, 207, 227),   // bright cyan
    swap: Color::Rgb(217, 120, 252),    // purple
    network_rx: Color::Rgb(170, 217, 76),  // lime
    network_tx: Color::Rgb(255, 51, 51),   // red
    disk_read: Color::Rgb(57, 186, 230),   // blue
    disk_write: Color::Rgb(242, 151, 39),  // dark orange
    status_running: Color::Rgb(170, 217, 76),
    status_sleeping: Color::Rgb(59, 73, 86),
    status_stopped: Color::Rgb(255, 51, 51),
    status_zombie: Color::Rgb(217, 120, 252),
    table_header_fg: Color::Rgb(255, 180, 84),
    pin: Color::Rgb(90, 207, 227),
    accent: Color::Rgb(57, 186, 230),
    text: Color::Rgb(203, 204, 198),
    text_secondary: Color::Rgb(127, 133, 138),
};

// ── Material — teal/coral dominant, deep purple/lime accents ───────────────

pub const PALETTE_MATERIAL: ColorPalette = ColorPalette {
    name: "Material",
    header_bg: Color::Rgb(38, 50, 56),
    header_fg: Color::Rgb(238, 255, 255),
    selected_bg: Color::Rgb(55, 71, 79),
    selected_fg: Color::Rgb(238, 255, 255),
    border: Color::Rgb(84, 110, 122),
    cpu: Color::Rgb(0, 188, 212),       // teal
    memory: Color::Rgb(240, 113, 120),  // coral
    swap: Color::Rgb(199, 146, 234),    // purple
    network_rx: Color::Rgb(195, 232, 141), // lime
    network_tx: Color::Rgb(255, 83, 112),  // red
    disk_read: Color::Rgb(130, 170, 255),  // blue
    disk_write: Color::Rgb(255, 203, 107), // amber
    status_running: Color::Rgb(195, 232, 141),
    status_sleeping: Color::Rgb(84, 110, 122),
    status_stopped: Color::Rgb(240, 113, 120),
    status_zombie: Color::Rgb(199, 146, 234),
    table_header_fg: Color::Rgb(0, 188, 212),
    pin: Color::Rgb(255, 203, 107),
    accent: Color::Rgb(130, 170, 255),
    text: Color::Rgb(238, 255, 255),
    text_secondary: Color::Rgb(144, 164, 174),
};

// ── Cyberpunk — neon cyan/magenta, electric green/yellow ───────────────────

pub const PALETTE_CYBERPUNK: ColorPalette = ColorPalette {
    name: "Cyberpunk",
    header_bg: Color::Rgb(20, 4, 40),
    header_fg: Color::Rgb(0, 255, 255),
    selected_bg: Color::Rgb(40, 10, 70),
    selected_fg: Color::Rgb(0, 255, 255),
    border: Color::Rgb(80, 40, 120),
    cpu: Color::Rgb(0, 255, 255),       // electric cyan
    memory: Color::Rgb(255, 0, 255),    // magenta
    swap: Color::Rgb(255, 213, 0),      // yellow
    network_rx: Color::Rgb(0, 255, 136),   // electric green
    network_tx: Color::Rgb(255, 0, 102),   // hot pink
    disk_read: Color::Rgb(189, 0, 255),    // violet
    disk_write: Color::Rgb(255, 106, 0),   // orange
    status_running: Color::Rgb(0, 255, 136),
    status_sleeping: Color::Rgb(80, 40, 120),
    status_stopped: Color::Rgb(255, 0, 102),
    status_zombie: Color::Rgb(189, 0, 255),
    table_header_fg: Color::Rgb(255, 213, 0),
    pin: Color::Rgb(0, 255, 255),
    accent: Color::Rgb(255, 0, 255),
    text: Color::Rgb(230, 230, 255),
    text_secondary: Color::Rgb(140, 120, 180),
};
