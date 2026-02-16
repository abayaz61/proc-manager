use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct AppLayout {
    pub header: Rect,
    pub cpu_panel: Rect,
    pub memory_panel: Rect,
    pub network_panel: Rect,
    pub disk_panel: Option<Rect>,
    pub process_table: Rect,
    pub statusbar: Rect,
}

impl AppLayout {
    pub fn new(area: Rect) -> Self {
        let is_narrow = area.width < 80;
        let is_short = area.height < 20;

        let dashboard_height = if is_short { 4 } else { 5 };

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),               // header
                Constraint::Length(dashboard_height), // dashboard
                Constraint::Min(8),                  // process table
                Constraint::Length(1),                // status bar
            ])
            .split(area);

        let (cpu_panel, memory_panel, network_panel, disk_panel) = if is_narrow {
            // 2-column layout for narrow terminals
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_chunks[1]);
            (cols[0], cols[1], cols[1], None)
        } else {
            // 4-column layout for wide terminals (>=120)
            if area.width >= 120 {
                let cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ])
                    .split(main_chunks[1]);
                (cols[0], cols[1], cols[2], Some(cols[3]))
            } else {
                // 3-column layout (default)
                let cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(34),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ])
                    .split(main_chunks[1]);
                (cols[0], cols[1], cols[2], None)
            }
        };

        Self {
            header: main_chunks[0],
            cpu_panel,
            memory_panel,
            network_panel,
            disk_panel,
            process_table: main_chunks[2],
            statusbar: main_chunks[3],
        }
    }
}
