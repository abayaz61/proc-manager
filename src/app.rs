use anyhow::Result;
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;
use ratatui::widgets::TableState;

use crate::action::Action;
use crate::config::Config;
use crate::data::collector::DataCollector;
use crate::data::history::RingBuffer;
use crate::data::persistence::ProcessState;
use crate::data::sysinfo_detail::SystemInfoDetail;
use crate::data::process::{ProcessList, SortColumn};
use crate::event::{Event, EventHandler};
use crate::input;
use crate::ui::process_table::Column;
use crate::ui::theme::ColorPalette;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Default,
    PerCpuChart,
    ResourceGraphs,
    SystemOverview,
    SystemInfo,
    MiniMonitor,
}

impl ViewMode {
    pub fn next(self) -> Self {
        match self {
            ViewMode::Default => ViewMode::PerCpuChart,
            ViewMode::PerCpuChart => ViewMode::ResourceGraphs,
            ViewMode::ResourceGraphs => ViewMode::SystemOverview,
            ViewMode::SystemOverview => ViewMode::SystemInfo,
            ViewMode::SystemInfo => ViewMode::MiniMonitor,
            ViewMode::MiniMonitor => ViewMode::Default,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ViewMode::Default => "Processes",
            ViewMode::PerCpuChart => "Per-CPU",
            ViewMode::ResourceGraphs => "Graphs",
            ViewMode::SystemOverview => "Overview",
            ViewMode::SystemInfo => "System Info",
            ViewMode::MiniMonitor => "Mini Monitor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    Search,
    Dialog,
    Help,
    NewProcess,
    Settings,
    ContextMenu,
    HiddenList,
    ThemePicker,
    PidFilter,
}

#[derive(Debug, Clone)]
pub enum SettingsItem {
    Slider {
        label: String,
        value: u64,
        min: u64,
        max: u64,
        step: u64,
        unit: String,
    },
    Toggle {
        label: String,
        value: bool,
    },
    Cycle {
        label: String,
        options: Vec<String>,
        selected: usize,
    },
    Action {
        label: String,
        description: String,
    },
}

#[derive(Debug, Clone)]
pub struct SettingsState {
    pub selected: usize,
    pub items: Vec<SettingsItem>,
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContextMenuState {
    pub x: u16,
    pub y: u16,
    pub selected: usize,
    pub items: Vec<ContextMenuItem>,
    pub target_pid: u32,
    pub target_name: String,
}

#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub label: String,
    pub action: ContextMenuAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContextMenuAction {
    Pin,
    Unpin,
    PinByName,
    UnpinByName,
    Hide,
    Unhide,
    Kill,
}

pub struct App {
    pub mode: AppMode,
    pub view_mode: ViewMode,
    pub compact_view: bool,
    pub palette: ColorPalette,
    pub theme_picker_selected: usize,
    pub theme_picker_previous: Option<ColorPalette>,
    pub collector: DataCollector,
    pub process_list: ProcessList,
    pub table_state: TableState,
    pub search_query: String,
    pub new_process_input: String,
    pub pid_filter_input: String,
    pub config: Config,
    pub visible_columns: Vec<Column>,
    pub status_message: Option<String>,
    status_message_ttl: u8,
    pub settings_state: Option<SettingsState>,
    pub context_menu: Option<ContextMenuState>,
    pub hidden_list_selected: usize,
    pub process_table_area: std::cell::Cell<Rect>,
    pub tick_rate_changed: Option<u64>,
    pub system_info_detail: SystemInfoDetail,

    cpu_history: RingBuffer<f64>,
    mem_history: RingBuffer<f64>,
    net_rx_history: RingBuffer<f64>,
    net_tx_history: RingBuffer<f64>,
    disk_read_history: RingBuffer<f64>,
    disk_write_history: RingBuffer<f64>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let history_len = config.history_len;
        let sort_column = config.parse_sort_column();
        let sort_ascending = config.sort_ascending;

        let visible_columns = Column::parse_list(&config.visible_columns);

        let mut process_list = ProcessList::new();
        process_list.set_sort(sort_column, sort_ascending);

        // Load persisted pin/hidden state
        let persisted = ProcessState::load();
        process_list.load_persisted_state(persisted.pinned_set(), persisted.hidden_set());

        let compact_view = config.compact_view;
        let palette = crate::ui::theme::by_name(&config.theme.theme_name);
        let theme_picker_selected = crate::ui::theme::ALL_PALETTES
            .iter()
            .position(|p| p.name == palette.name)
            .unwrap_or(0);

        Self {
            mode: AppMode::Normal,
            view_mode: ViewMode::Default,
            compact_view,
            palette,
            theme_picker_selected,
            theme_picker_previous: None,
            collector: DataCollector::new(),
            process_list,
            table_state: TableState::default().with_selected(0),
            search_query: String::new(),
            new_process_input: String::new(),
            pid_filter_input: String::new(),
            visible_columns,
            status_message: None,
            status_message_ttl: 0,
            settings_state: None,
            context_menu: None,
            hidden_list_selected: 0,
            process_table_area: std::cell::Cell::new(Rect::default()),
            tick_rate_changed: None,
            system_info_detail: SystemInfoDetail::collect(),
            cpu_history: RingBuffer::new(history_len),
            mem_history: RingBuffer::new(history_len),
            net_rx_history: RingBuffer::new(history_len),
            net_tx_history: RingBuffer::new(history_len),
            disk_read_history: RingBuffer::new(history_len),
            disk_write_history: RingBuffer::new(history_len),
            config,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut events = EventHandler::new(self.config.tick_rate_ms);

        // Apply always-on-top from config on startup
        if self.config.always_on_top {
            apply_always_on_top(true);
        }

        // Initial data collection
        self.collector.refresh();
        self.update_data();

        loop {
            terminal.draw(|frame| crate::ui::draw(frame, self))?;

            match events.next().await? {
                Event::Tick => {
                    self.collector.refresh();
                    self.update_data();
                    // Auto-clear status message after TTL
                    if self.status_message.is_some() {
                        if self.status_message_ttl == 0 {
                            self.status_message = None;
                        } else {
                            self.status_message_ttl -= 1;
                        }
                    }
                }
                Event::Key(key) if key.kind == crossterm::event::KeyEventKind::Press => {
                    let action = input::map_key(key, self.mode);
                    if action == Action::Quit {
                        break;
                    }
                    self.dispatch(action);
                    if let Some(new_rate) = self.tick_rate_changed.take() {
                        events.set_tick_rate(new_rate);
                    }
                }
                Event::Key(_) => {}
                Event::Resize(_, _) => {}
                Event::Mouse(mouse) => {
                    self.handle_mouse(mouse);
                }
            }
        }

        Ok(())
    }

    fn update_data(&mut self) {
        // Remember selected PID before refresh
        let selected_pid = self.selected_process().map(|p| p.pid);

        let processes = self.collector.processes();
        self.process_list.update(processes);
        self.process_list.resolve_pinned_pids();

        // Restore selection to same PID (using display index for pinned-first ordering)
        if let Some(pid) = selected_pid {
            if let Some(new_idx) = self.process_list.find_display_index_by_pid(pid) {
                self.table_state.select(Some(new_idx));
            }
        }

        // Prune dead pinned processes (5 second timeout if not keeping them)
        if !self.config.keep_dead_pins {
            let pruned = self
                .process_list
                .prune_dead_pins(std::time::Duration::from_secs(5));
            if !pruned.is_empty() {
                self.save_process_state();
            }
        }

        // CPU history
        self.cpu_history
            .push(self.collector.system_data.global_cpu_usage as f64);

        // Memory history
        let mem_pct = if self.collector.system_data.total_memory > 0 {
            (self.collector.system_data.used_memory as f64
                / self.collector.system_data.total_memory as f64)
                * 100.0
        } else {
            0.0
        };
        self.mem_history.push(mem_pct);

        // Network history
        self.net_rx_history
            .push(self.collector.network_data.bytes_received as f64);
        self.net_tx_history
            .push(self.collector.network_data.bytes_transmitted as f64);

        // Disk I/O history (aggregate from all processes)
        let (total_read, total_write) = self
            .process_list
            .all_entries()
            .iter()
            .fold((0u64, 0u64), |(r, w), p| {
                (r + p.disk_read_bytes, w + p.disk_write_bytes)
            });
        self.disk_read_history.push(total_read as f64);
        self.disk_write_history.push(total_write as f64);
    }

    fn dispatch(&mut self, action: Action) {
        // In non-Default views, ignore process-specific actions
        if self.view_mode != ViewMode::Default {
            match action {
                Action::Quit => unreachable!(),
                Action::CycleViewMode => {
                    self.view_mode = self.view_mode.next();
                    return;
                }
                Action::ToggleHelp => {
                    self.toggle_help();
                    return;
                }
                Action::OpenSettings => {
                    self.open_settings();
                    return;
                }
                Action::ToggleCompactView => {
                    self.compact_view = !self.compact_view;
                    self.config.compact_view = self.compact_view;
                    let label = if self.compact_view { "ON" } else { "OFF" };
                    self.set_status(format!("Compact view: {}", label));
                    return;
                }
                Action::OpenThemePicker => {
                    self.open_theme_picker();
                    return;
                }
                Action::ToggleAlwaysOnTop => {
                    self.toggle_always_on_top();
                    return;
                }
                _ => return,
            }
        }

        match action {
            Action::Quit => unreachable!(),
            Action::ScrollUp => self.scroll_up(),
            Action::ScrollDown => self.scroll_down(),
            Action::PageUp => self.page_up(),
            Action::PageDown => self.page_down(),
            Action::Home => self.scroll_home(),
            Action::End => self.scroll_end(),
            Action::SortBy(col) => self.sort_by(col),
            Action::ReverseSortOrder => self.reverse_sort(),
            Action::ToggleSearch => self.toggle_search(),
            Action::SearchInput(c) => self.search_input(c),
            Action::SearchBackspace => self.search_backspace(),
            Action::SearchSubmit => self.mode = AppMode::Normal,
            Action::KillProcess => self.start_kill(),
            Action::KillProcessConfirm => self.confirm_kill(),
            Action::CancelDialog => {
                if self.mode == AppMode::Settings {
                    self.settings_state = None;
                }
                if self.mode == AppMode::ContextMenu {
                    self.context_menu = None;
                }
                if self.mode == AppMode::ThemePicker {
                    self.theme_picker_cancel();
                    self.status_message = None;
                    return;
                }
                if self.mode == AppMode::PidFilter {
                    self.pid_filter_input.clear();
                    self.process_list.set_pid_filter(None);
                }
                self.mode = AppMode::Normal;
                self.status_message = None;
            }
            Action::ToggleHelp => self.toggle_help(),
            Action::StartNewProcess => self.start_new_process(),
            Action::SubmitNewProcess => self.submit_new_process(),
            Action::NewProcessInput(c) => self.new_process_input.push(c),
            Action::NewProcessBackspace => {
                self.new_process_input.pop();
            }
            Action::TogglePin => self.toggle_pin(),
            Action::ClearPins => {
                self.process_list.clear_pins();
                self.save_process_state();
                self.set_status("All pins cleared".to_string());
            }
            Action::HideProcess => self.hide_selected(),
            Action::OpenHiddenList => self.open_hidden_list(),
            Action::HiddenListUp => self.hidden_list_navigate(-1),
            Action::HiddenListDown => self.hidden_list_navigate(1),
            Action::HiddenListUnhide => self.hidden_list_unhide(),
            Action::HiddenListUnhideAll => self.hidden_list_unhide_all(),
            Action::ContextMenuUp => self.context_menu_navigate(-1),
            Action::ContextMenuDown => self.context_menu_navigate(1),
            Action::ContextMenuSelect => self.context_menu_select(),
            Action::OpenSettings => self.open_settings(),
            Action::SettingsUp => self.settings_navigate(-1),
            Action::SettingsDown => self.settings_navigate(1),
            Action::SettingsLeft => self.settings_adjust(-1),
            Action::SettingsRight => self.settings_adjust(1),
            Action::SettingsSelect => self.settings_select(),
            Action::SettingsSave => self.settings_save(),
            Action::CycleViewMode => {
                self.view_mode = self.view_mode.next();
            }
            Action::ToggleCompactView => {
                self.compact_view = !self.compact_view;
                self.config.compact_view = self.compact_view;
                let label = if self.compact_view { "ON" } else { "OFF" };
                self.set_status(format!("Compact view: {}", label));
            }
            Action::OpenThemePicker => self.open_theme_picker(),
            Action::ThemePickerUp => self.theme_picker_navigate(-1),
            Action::ThemePickerDown => self.theme_picker_navigate(1),
            Action::ThemePickerSelect => self.theme_picker_select(),
            Action::OpenPidFilter => self.open_pid_filter(),
            Action::PidFilterInput(c) => self.pid_filter_input(c),
            Action::PidFilterBackspace => self.pid_filter_backspace(),
            Action::PidFilterSubmit => self.pid_filter_submit(),
            Action::ToggleAlwaysOnTop => self.toggle_always_on_top(),
            Action::Noop => {}
        }
    }

    fn scroll_up(&mut self) {
        let selected = self.table_state.selected().unwrap_or(0);
        if selected > 0 {
            let prev = selected - 1;
            // Skip separator row
            if self.process_list.get_by_display_index(prev).is_none() && prev > 0 {
                self.table_state.select(Some(prev - 1));
            } else if self.process_list.get_by_display_index(prev).is_some() {
                self.table_state.select(Some(prev));
            }
        }
    }

    fn scroll_down(&mut self) {
        let selected = self.table_state.selected().unwrap_or(0);
        let max = self.process_list.display_row_count().saturating_sub(1);
        if selected < max {
            let next = selected + 1;
            // Skip separator row
            if self.process_list.get_by_display_index(next).is_none() && next < max {
                self.table_state.select(Some(next + 1));
            } else {
                self.table_state.select(Some(next));
            }
        }
    }

    fn page_up(&mut self) {
        let selected = self.table_state.selected().unwrap_or(0);
        let mut target = selected.saturating_sub(20);
        // Skip separator row
        if self.process_list.get_by_display_index(target).is_none() && target > 0 {
            target -= 1;
        }
        self.table_state.select(Some(target));
    }

    fn page_down(&mut self) {
        let selected = self.table_state.selected().unwrap_or(0);
        let max = self.process_list.display_row_count().saturating_sub(1);
        let mut target = (selected + 20).min(max);
        // Skip separator row
        if self.process_list.get_by_display_index(target).is_none() && target < max {
            target += 1;
        }
        self.table_state.select(Some(target));
    }

    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        use crossterm::event::{MouseButton, MouseEventKind};

        let col = mouse.column;
        let row = mouse.row;

        // If context menu is open, handle clicks on it
        if self.mode == AppMode::ContextMenu {
            match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if let Some(menu) = &self.context_menu {
                        let menu_x = menu.x;
                        let menu_y = menu.y;
                        let menu_h = menu.items.len() as u16 + 2; // +2 for borders
                        let menu_w = 20u16;
                        if col >= menu_x
                            && col < menu_x + menu_w
                            && row > menu_y
                            && row < menu_y + menu_h - 1
                        {
                            let clicked = (row - menu_y - 1) as usize;
                            if let Some(menu) = &mut self.context_menu {
                                if clicked < menu.items.len() {
                                    menu.selected = clicked;
                                }
                            }
                            self.context_menu_select();
                        } else {
                            self.context_menu = None;
                            self.mode = AppMode::Normal;
                        }
                    }
                }
                _ => {}
            }
            return;
        }

        if self.mode != AppMode::Normal || self.view_mode != ViewMode::Default {
            return;
        }

        let area = self.process_table_area.get();

        // Table has 1 row border top + 1 header row = content starts at area.y + 2
        let content_start_y = area.y + 2;
        let content_end_y = area.y + area.height.saturating_sub(1);

        let in_table = col >= area.x
            && col < area.x + area.width
            && row >= content_start_y
            && row < content_end_y;

        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if in_table {
                    let clicked_row = (row - content_start_y) as usize;
                    let offset = self.table_state.offset();
                    let index = offset + clicked_row;
                    let max = self.process_list.display_row_count().saturating_sub(1);
                    if index <= max {
                        // Don't select separator row
                        if self.process_list.get_by_display_index(index).is_some() {
                            self.table_state.select(Some(index));
                        }
                    }
                }
            }
            MouseEventKind::Down(MouseButton::Right) => {
                if in_table {
                    let clicked_row = (row - content_start_y) as usize;
                    let offset = self.table_state.offset();
                    let index = offset + clicked_row;
                    let max = self.process_list.display_row_count().saturating_sub(1);
                    if index <= max {
                        // Don't open context menu on separator row
                        if self.process_list.get_by_display_index(index).is_some() {
                            self.table_state.select(Some(index));
                            self.open_context_menu(col, row);
                        }
                    }
                }
            }
            MouseEventKind::ScrollUp => {
                if col >= area.x
                    && col < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
                {
                    self.scroll_up();
                }
            }
            MouseEventKind::ScrollDown => {
                if col >= area.x
                    && col < area.x + area.width
                    && row >= area.y
                    && row < area.y + area.height
                {
                    self.scroll_down();
                }
            }
            _ => {}
        }
    }

    fn scroll_home(&mut self) {
        self.table_state.select(Some(0));
    }

    fn scroll_end(&mut self) {
        let max = self.process_list.display_row_count().saturating_sub(1);
        self.table_state.select(Some(max));
    }

    fn sort_by(&mut self, col: SortColumn) {
        if self.process_list.sort_column == col {
            let asc = !self.process_list.sort_ascending;
            self.process_list.set_sort(col, asc);
        } else {
            let ascending = matches!(col, SortColumn::Pid | SortColumn::Name | SortColumn::User);
            self.process_list.set_sort(col, ascending);
        }
    }

    fn reverse_sort(&mut self) {
        let col = self.process_list.sort_column;
        let asc = !self.process_list.sort_ascending;
        self.process_list.set_sort(col, asc);
    }

    fn toggle_search(&mut self) {
        match self.mode {
            AppMode::Search => {
                self.mode = AppMode::Normal;
            }
            AppMode::Normal => {
                self.search_query.clear();
                self.mode = AppMode::Search;
            }
            _ => {}
        }
    }

    fn search_input(&mut self, c: char) {
        self.search_query.push(c);
        self.process_list.set_search(self.search_query.clone());
        self.table_state.select(Some(0));
    }

    fn search_backspace(&mut self) {
        self.search_query.pop();
        self.process_list.set_search(self.search_query.clone());
        self.table_state.select(Some(0));
    }

    fn toggle_help(&mut self) {
        self.mode = if self.mode == AppMode::Help {
            AppMode::Normal
        } else {
            AppMode::Help
        };
    }

    fn start_kill(&mut self) {
        if self.selected_process().is_some() {
            self.mode = AppMode::Dialog;
        }
    }

    fn confirm_kill(&mut self) {
        if let Some(p) = self.selected_process() {
            let pid = p.pid;
            let name = p.name.clone();
            if self.collector.kill_process(pid) {
                self.set_status(format!("Killed {} (PID: {})", name, pid));
            } else {
                self.set_status(format!("Failed to kill {} (PID: {})", name, pid));
            }
        }
        self.mode = AppMode::Normal;
    }

    fn start_new_process(&mut self) {
        self.new_process_input.clear();
        self.mode = AppMode::NewProcess;
    }

    fn submit_new_process(&mut self) {
        let cmd = self.new_process_input.trim().to_string();
        if !cmd.is_empty() {
            let result = {
                #[cfg(target_os = "windows")]
                {
                    std::process::Command::new("cmd")
                        .args(["/C", &cmd])
                        .spawn()
                }
                #[cfg(not(target_os = "windows"))]
                {
                    std::process::Command::new("sh")
                        .args(["-c", &cmd])
                        .spawn()
                }
            };
            match result {
                Ok(child) => {
                    self.set_status(format!(
                        "Started '{}' (PID: {})",
                        cmd,
                        child.id()
                    ));
                }
                Err(e) => {
                    self.set_status(format!("Failed to start '{}': {}", cmd, e));
                }
            }
        }
        self.new_process_input.clear();
        self.mode = AppMode::Normal;
    }

    fn set_status(&mut self, msg: String) {
        self.status_message = Some(msg);
        self.status_message_ttl = 3; // 3 ticks = ~3 seconds
    }

    fn toggle_pin(&mut self) {
        if let Some(p) = self.selected_process() {
            let pid = p.pid;
            let name = p.name.clone();
            self.process_list.toggle_pin(pid);
            if self.process_list.is_pinned(pid) {
                self.set_status(format!("Pinned {} (PID: {})", name, pid));
            } else {
                self.set_status(format!("Unpinned {} (PID: {})", name, pid));
            }
            self.save_process_state();
        }
    }

    fn hide_selected(&mut self) {
        if let Some(p) = self.selected_process() {
            let pid = p.pid;
            let name = p.name.clone();
            let was_hidden = self.process_list.is_hidden_name(&name);
            self.process_list.toggle_hidden(pid);
            if was_hidden {
                self.set_status(format!("Unhidden: {}", name));
            } else {
                self.set_status(format!("Hidden: {} (press H to show hidden)", name));
            }
            self.save_process_state();
        }
    }

    fn open_hidden_list(&mut self) {
        if self.process_list.hidden_count() == 0 {
            self.set_status("No hidden processes".to_string());
            return;
        }
        self.hidden_list_selected = 0;
        self.mode = AppMode::HiddenList;
    }

    fn hidden_list_navigate(&mut self, delta: i32) {
        let count = self.process_list.hidden_count();
        if count == 0 {
            return;
        }
        if delta < 0 {
            self.hidden_list_selected = self.hidden_list_selected.saturating_sub(1);
        } else {
            self.hidden_list_selected = (self.hidden_list_selected + 1).min(count - 1);
        }
    }

    fn hidden_list_unhide(&mut self) {
        let names: Vec<String> = self.process_list.hidden_names().iter().cloned().collect();
        let mut names_sorted = names;
        names_sorted.sort();
        if let Some(name) = names_sorted.get(self.hidden_list_selected).cloned() {
            self.process_list.unhide_by_name(&name);
            self.save_process_state();
            self.set_status(format!("Unhidden: {}", name));
            // Adjust selection if needed
            let new_count = self.process_list.hidden_count();
            if new_count == 0 {
                self.mode = AppMode::Normal;
            } else if self.hidden_list_selected >= new_count {
                self.hidden_list_selected = new_count - 1;
            }
        }
    }

    fn hidden_list_unhide_all(&mut self) {
        let count = self.process_list.hidden_count();
        self.process_list.clear_hidden();
        self.save_process_state();
        self.set_status(format!("Unhidden all {} process(es)", count));
        self.mode = AppMode::Normal;
    }

    fn save_process_state(&self) {
        let state = ProcessState {
            pinned: self.process_list.pinned_names().iter().cloned().collect(),
            hidden: self.process_list.hidden_names().iter().cloned().collect(),
        };
        state.save();
    }

    fn open_context_menu(&mut self, x: u16, y: u16) {
        if let Some(p) = self.selected_process() {
            let pid = p.pid;
            let name = p.name.clone();
            let is_pinned = self.process_list.is_pinned(pid);
            let is_name_pinned = self.process_list.is_name_pinned(&name);
            let is_hidden = self.process_list.is_hidden_name(&name);
            let name_count = self.process_list.count_by_name(&name);

            let mut items = Vec::new();
            if is_pinned {
                items.push(ContextMenuItem {
                    label: "Unpin".to_string(),
                    action: ContextMenuAction::Unpin,
                });
            } else {
                items.push(ContextMenuItem {
                    label: "Pin".to_string(),
                    action: ContextMenuAction::Pin,
                });
            }
            // Show "Pin by Name" / "Unpin by Name" when multiple processes share the name
            if name_count > 1 {
                if is_name_pinned {
                    items.push(ContextMenuItem {
                        label: format!("Unpin by Name ({})", name_count),
                        action: ContextMenuAction::UnpinByName,
                    });
                } else {
                    items.push(ContextMenuItem {
                        label: format!("Pin by Name ({})", name_count),
                        action: ContextMenuAction::PinByName,
                    });
                }
            }
            if is_hidden {
                items.push(ContextMenuItem {
                    label: "Unhide".to_string(),
                    action: ContextMenuAction::Unhide,
                });
            } else {
                items.push(ContextMenuItem {
                    label: "Hide".to_string(),
                    action: ContextMenuAction::Hide,
                });
            }
            items.push(ContextMenuItem {
                label: "Kill".to_string(),
                action: ContextMenuAction::Kill,
            });

            self.context_menu = Some(ContextMenuState {
                x,
                y,
                selected: 0,
                items,
                target_pid: pid,
                target_name: name,
            });
            self.mode = AppMode::ContextMenu;
        }
    }

    fn context_menu_navigate(&mut self, delta: i32) {
        if let Some(menu) = &mut self.context_menu {
            let count = menu.items.len();
            if delta < 0 {
                menu.selected = menu.selected.saturating_sub(1);
            } else {
                menu.selected = (menu.selected + 1).min(count - 1);
            }
        }
    }

    fn context_menu_select(&mut self) {
        if let Some(menu) = self.context_menu.take() {
            let pid = menu.target_pid;
            let name = menu.target_name;
            match menu.items[menu.selected].action {
                ContextMenuAction::Pin => {
                    self.process_list.toggle_pin(pid);
                    self.set_status(format!("Pinned {}", name));
                    self.save_process_state();
                }
                ContextMenuAction::Unpin => {
                    self.process_list.toggle_pin(pid);
                    self.set_status(format!("Unpinned {}", name));
                    self.save_process_state();
                }
                ContextMenuAction::PinByName => {
                    let count = self.process_list.count_by_name(&name);
                    self.process_list.pin_by_name(&name);
                    self.set_status(format!("Pinned all '{}' ({} processes)", name, count));
                    self.save_process_state();
                }
                ContextMenuAction::UnpinByName => {
                    let count = self.process_list.count_by_name(&name);
                    self.process_list.unpin_by_name(&name);
                    self.set_status(format!("Unpinned all '{}' ({} processes)", name, count));
                    self.save_process_state();
                }
                ContextMenuAction::Hide => {
                    self.process_list.toggle_hidden(pid);
                    self.set_status(format!("Hidden: {}", name));
                    self.save_process_state();
                }
                ContextMenuAction::Unhide => {
                    self.process_list.toggle_hidden(pid);
                    self.set_status(format!("Unhidden: {}", name));
                    self.save_process_state();
                }
                ContextMenuAction::Kill => {
                    if self.collector.kill_process(pid) {
                        self.set_status(format!("Killed {} (PID: {})", name, pid));
                    } else {
                        self.set_status(format!("Failed to kill {} (PID: {})", name, pid));
                    }
                }
            }
        }
        self.mode = AppMode::Normal;
    }

    fn open_settings(&mut self) {
        let sort_options = vec![
            "pid".to_string(),
            "name".to_string(),
            "user".to_string(),
            "cpu".to_string(),
            "memory".to_string(),
            "status".to_string(),
            "threads".to_string(),
            "time".to_string(),
            "disk_read".to_string(),
            "disk_write".to_string(),
            "ppid".to_string(),
        ];
        let sort_idx = sort_options
            .iter()
            .position(|s| s == &self.config.sort_column)
            .unwrap_or(3);

        let mut items = vec![
            SettingsItem::Slider {
                label: "Refresh Rate".to_string(),
                value: self.config.tick_rate_ms,
                min: 200,
                max: 5000,
                step: 200,
                unit: "ms".to_string(),
            },
            SettingsItem::Cycle {
                label: "Default Sort Column".to_string(),
                options: sort_options,
                selected: sort_idx,
            },
            SettingsItem::Toggle {
                label: "Sort Ascending".to_string(),
                value: self.config.sort_ascending,
            },
            SettingsItem::Slider {
                label: "History Length".to_string(),
                value: self.config.history_len as u64,
                min: 10,
                max: 300,
                step: 10,
                unit: "".to_string(),
            },
            SettingsItem::Toggle {
                label: "Colors".to_string(),
                value: self.config.theme.color_enabled,
            },
            SettingsItem::Toggle {
                label: "Bold Headers".to_string(),
                value: self.config.theme.bold_headers,
            },
            SettingsItem::Toggle {
                label: "Standalone Window".to_string(),
                value: self.config.standalone_window,
            },
            SettingsItem::Toggle {
                label: "Keep Dead Pins".to_string(),
                value: self.config.keep_dead_pins,
            },
            SettingsItem::Toggle {
                label: "Compact View".to_string(),
                value: self.compact_view,
            },
            SettingsItem::Toggle {
                label: "Always on Top".to_string(),
                value: self.config.always_on_top,
            },
        ];

        // Column toggles for optional columns
        for col in Column::all() {
            if col.is_required() {
                continue;
            }
            let enabled = self.visible_columns.contains(col);
            items.push(SettingsItem::Toggle {
                label: format!("Column: {}", col.header_name()),
                value: enabled,
            });
        }

        items.push(SettingsItem::Action {
            label: "Change Theme".to_string(),
            description: format!("Current: {}", self.palette.name),
        });
        items.push(SettingsItem::Action {
            label: "Register to PATH".to_string(),
            description: "Add prc to system PATH".to_string(),
        });
        items.push(SettingsItem::Action {
            label: "Create Desktop Shortcut".to_string(),
            description: "Create shortcut on desktop".to_string(),
        });
        items.push(SettingsItem::Action {
            label: "Save & Close".to_string(),
            description: "Save settings to config file".to_string(),
        });

        self.settings_state = Some(SettingsState {
            selected: 0,
            items,
            status: None,
        });
        self.mode = AppMode::Settings;
    }

    fn settings_navigate(&mut self, delta: i32) {
        if let Some(state) = &mut self.settings_state {
            let count = state.items.len();
            if count == 0 {
                return;
            }
            if delta < 0 {
                state.selected = state.selected.saturating_sub(1);
            } else {
                state.selected = (state.selected + 1).min(count - 1);
            }
        }
    }

    fn settings_adjust(&mut self, direction: i32) {
        if let Some(state) = &mut self.settings_state {
            let idx = state.selected;
            match &mut state.items[idx] {
                SettingsItem::Slider {
                    value,
                    min,
                    max,
                    step,
                    ..
                } => {
                    if direction < 0 {
                        *value = (*value).saturating_sub(*step).max(*min);
                    } else {
                        *value = (*value + *step).min(*max);
                    }
                }
                SettingsItem::Toggle { value, .. } => {
                    *value = !*value;
                }
                SettingsItem::Cycle {
                    options, selected, ..
                } => {
                    let len = options.len();
                    if direction < 0 {
                        *selected = if *selected == 0 { len - 1 } else { *selected - 1 };
                    } else {
                        *selected = (*selected + 1) % len;
                    }
                }
                SettingsItem::Action { .. } => {
                    // Actions use Enter, not Left/Right
                }
            }
        }
    }

    fn settings_select(&mut self) {
        if let Some(state) = &mut self.settings_state {
            let idx = state.selected;
            match &state.items[idx] {
                SettingsItem::Toggle { .. } => {
                    // Toggle on Enter too
                    if let SettingsItem::Toggle { value, .. } = &mut state.items[idx] {
                        *value = !*value;
                    }
                }
                SettingsItem::Action { label, .. } => {
                    let label = label.clone();
                    match label.as_str() {
                        "Change Theme" => {
                            // Close settings and open theme picker
                            self.settings_state = None;
                            self.open_theme_picker();
                            return;
                        }
                        "Register to PATH" => {
                            let result = register_to_path();
                            state.status = Some(result);
                        }
                        "Create Desktop Shortcut" => {
                            let result = create_desktop_shortcut();
                            state.status = Some(result);
                        }
                        "Save & Close" => {
                            self.apply_settings();
                            return;
                        }
                        _ => {}
                    }
                }
                _ => {
                    // Slider/Cycle: Enter does nothing special
                }
            }
        }
    }

    fn settings_save(&mut self) {
        self.apply_settings();
    }

    fn apply_settings(&mut self) {
        if let Some(state) = self.settings_state.take() {
            let old_tick_rate = self.config.tick_rate_ms;
            // Collect column toggles
            let mut new_columns: Vec<Column> = vec![Column::Pid, Column::Name];
            for item in &state.items {
                match item {
                    SettingsItem::Slider { label, value, .. } => match label.as_str() {
                        "Refresh Rate" => self.config.tick_rate_ms = *value,
                        "History Length" => self.config.history_len = *value as usize,
                        _ => {}
                    },
                    SettingsItem::Toggle { label, value } => match label.as_str() {
                        "Sort Ascending" => self.config.sort_ascending = *value,
                        "Colors" => self.config.theme.color_enabled = *value,
                        "Bold Headers" => self.config.theme.bold_headers = *value,
                        "Standalone Window" => self.config.standalone_window = *value,
                        "Keep Dead Pins" => self.config.keep_dead_pins = *value,
                        "Compact View" => {
                            self.compact_view = *value;
                            self.config.compact_view = *value;
                        }
                        "Always on Top" => {
                            self.config.always_on_top = *value;
                            apply_always_on_top(*value);
                        }
                        _ => {
                            // Handle "Column: XYZ" toggles
                            if let Some(col_name) = label.strip_prefix("Column: ") {
                                if *value {
                                    // Find the Column by header name
                                    for col in Column::all() {
                                        if col.header_name() == col_name && !col.is_required() {
                                            new_columns.push(*col);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    },
                    SettingsItem::Cycle {
                        label,
                        options,
                        selected,
                    } => {
                        if label == "Default Sort Column" {
                            self.config.sort_column = options[*selected].clone();
                        }
                    }
                    SettingsItem::Action { .. } => {}
                }
            }

            // Update visible columns
            self.visible_columns = new_columns;
            self.config.visible_columns = self.visible_columns.iter().map(|c| c.to_id().to_string()).collect();

            // Apply tick rate change at runtime
            if self.config.tick_rate_ms != old_tick_rate {
                self.tick_rate_changed = Some(self.config.tick_rate_ms);
            }

            // Apply sort settings at runtime
            let sort_col = self.config.parse_sort_column();
            self.process_list.set_sort(sort_col, self.config.sort_ascending);

            match self.config.save() {
                Ok(_) => self.set_status("Settings saved".to_string()),
                Err(e) => self.set_status(format!("Failed to save: {}", e)),
            }
        }
        self.mode = AppMode::Normal;
    }

    fn open_theme_picker(&mut self) {
        self.theme_picker_selected = crate::ui::theme::ALL_PALETTES
            .iter()
            .position(|p| p.name == self.palette.name)
            .unwrap_or(0);
        self.theme_picker_previous = Some(self.palette);
        self.mode = AppMode::ThemePicker;
    }

    fn theme_picker_navigate(&mut self, delta: i32) {
        let count = crate::ui::theme::ALL_PALETTES.len();
        if delta < 0 {
            self.theme_picker_selected = self.theme_picker_selected.saturating_sub(1);
        } else {
            self.theme_picker_selected = (self.theme_picker_selected + 1).min(count - 1);
        }
        // Live preview
        self.palette = crate::ui::theme::ALL_PALETTES[self.theme_picker_selected];
    }

    fn theme_picker_select(&mut self) {
        let palette = crate::ui::theme::ALL_PALETTES[self.theme_picker_selected];
        self.palette = palette;
        self.config.theme.theme_name = palette.name.to_string();
        self.theme_picker_previous = None;
        self.mode = AppMode::Normal;
        match self.config.save() {
            Ok(_) => self.set_status(format!("Theme: {}", palette.name)),
            Err(e) => self.set_status(format!("Theme set but save failed: {}", e)),
        }
    }

    fn theme_picker_cancel(&mut self) {
        if let Some(prev) = self.theme_picker_previous.take() {
            self.palette = prev;
        }
        self.mode = AppMode::Normal;
    }

    fn open_pid_filter(&mut self) {
        self.pid_filter_input = String::new();
        self.process_list.set_pid_filter(None);
        self.mode = AppMode::PidFilter;
    }

    fn pid_filter_input(&mut self, c: char) {
        if c.is_ascii_digit() {
            self.pid_filter_input.push(c);
            let pid = self.pid_filter_input.parse::<u32>().ok();
            self.process_list.set_pid_filter(pid);
            self.table_state.select(Some(0));
        }
    }

    fn pid_filter_backspace(&mut self) {
        self.pid_filter_input.pop();
        let pid = if self.pid_filter_input.is_empty() {
            None
        } else {
            self.pid_filter_input.parse::<u32>().ok()
        };
        self.process_list.set_pid_filter(pid);
        self.table_state.select(Some(0));
    }

    fn pid_filter_submit(&mut self) {
        // Keep filter active, return to normal mode
        self.mode = AppMode::Normal;
        if self.pid_filter_input.is_empty() {
            self.process_list.set_pid_filter(None);
        } else {
            self.set_status(format!("PID filter: {}", self.pid_filter_input));
        }
    }

    fn toggle_always_on_top(&mut self) {
        self.config.always_on_top = !self.config.always_on_top;
        let enabled = self.config.always_on_top;
        apply_always_on_top(enabled);
        let label = if enabled { "ON" } else { "OFF" };
        self.set_status(format!("Always on Top: {}", label));
        let _ = self.config.save();
    }

    pub fn selected_process(&self) -> Option<&crate::data::process::ProcessEntry> {
        let idx = self.table_state.selected()?;
        self.process_list.get_by_display_index(idx)
    }

    pub fn cpu_history_sparkline(&self) -> Vec<u64> {
        self.cpu_history.as_sparkline_data()
    }

    pub fn mem_history_sparkline(&self) -> Vec<u64> {
        self.mem_history.as_sparkline_data()
    }

    pub fn net_rx_history_sparkline(&self) -> Vec<u64> {
        self.net_rx_history.as_sparkline_data()
    }

    #[allow(dead_code)]
    pub fn net_tx_history_sparkline(&self) -> Vec<u64> {
        self.net_tx_history.as_sparkline_data()
    }

    pub fn disk_read_history_sparkline(&self) -> Vec<u64> {
        self.disk_read_history.as_sparkline_data()
    }

    #[allow(dead_code)]
    pub fn disk_write_history_sparkline(&self) -> Vec<u64> {
        self.disk_write_history.as_sparkline_data()
    }
}

#[cfg(target_os = "windows")]
fn apply_always_on_top(enabled: bool) {
    use windows::Win32::System::Console::GetConsoleWindow;
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, HWND_TOPMOST, HWND_NOTOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_NOACTIVATE,
    };

    unsafe {
        let hwnd = GetConsoleWindow();
        if hwnd.0 == 0 {
            return;
        }
        let insert_after = if enabled { HWND_TOPMOST } else { HWND_NOTOPMOST };
        if let Err(error) = SetWindowPos(
            hwnd,
            insert_after,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        ) {
            eprintln!("Failed to set window position: {}", error);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn apply_always_on_top(_enabled: bool) {
    // No-op on non-Windows platforms
}

fn register_to_path() -> String {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => return format!("Failed: {}", e),
    };
    let exe_dir = match exe_path.parent() {
        Some(d) => d.to_string_lossy().to_string(),
        None => return "Failed: cannot find exe directory".to_string(),
    };

    #[cfg(target_os = "windows")]
    {
        // Read current user PATH and append if not already present
        let current_path = std::env::var("PATH").unwrap_or_default();
        if current_path
            .split(';')
            .any(|p| p.eq_ignore_ascii_case(&exe_dir))
        {
            return "Already registered in PATH".to_string();
        }
        // Use PowerShell to read user PATH from registry and append
        let ps_script = format!(
            "$old = [Environment]::GetEnvironmentVariable('Path', 'User'); \
             if ($old -split ';' | Where-Object {{ $_ -eq '{}' }}) {{ exit 0 }}; \
             $new = if ($old) {{ \"$old;{}\" }} else {{ '{}' }}; \
             [Environment]::SetEnvironmentVariable('Path', $new, 'User')",
            exe_dir, exe_dir, exe_dir
        );
        match std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_script])
            .output()
        {
            Ok(output) if output.status.success() => {
                "Registered! Restart terminal to use 'prc'".to_string()
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Failed: {}", stderr.trim())
            }
            Err(e) => format!("Failed: {}", e),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Create symlink in ~/.local/bin
        let home = std::env::var("HOME").unwrap_or_default();
        let bin_dir = std::path::PathBuf::from(&home).join(".local/bin");
        let _ = std::fs::create_dir_all(&bin_dir);
        let link_path = bin_dir.join("prc");
        if link_path.exists() {
            let _ = std::fs::remove_file(&link_path);
        }
        match std::os::unix::fs::symlink(&exe_path, &link_path) {
            Ok(_) => format!("Symlink created at {}", link_path.display()),
            Err(e) => format!("Failed: {}", e),
        }
    }
}

fn create_desktop_shortcut() -> String {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => return format!("Failed: {}", e),
    };

    #[cfg(target_os = "windows")]
    {
        let desktop = match std::env::var("USERPROFILE") {
            Ok(home) => format!("{}\\Desktop\\Process Manager.lnk", home),
            Err(_) => return "Failed: USERPROFILE not found".to_string(),
        };
        let ps_script = format!(
            "$ws = New-Object -ComObject WScript.Shell; \
             $s = $ws.CreateShortcut('{}'); \
             $s.TargetPath = '{}'; \
             $s.WorkingDirectory = '{}'; \
             $s.Description = 'Process Manager TUI'; \
             $s.Save()",
            desktop,
            exe_path.display(),
            exe_path.parent().map(|p| p.display().to_string()).unwrap_or_default()
        );
        match std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_script])
            .output()
        {
            Ok(output) if output.status.success() => "Desktop shortcut created!".to_string(),
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Failed: {}", stderr.trim())
            }
            Err(e) => format!("Failed: {}", e),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let desktop_entry = format!(
            "[Desktop Entry]\nType=Application\nName=Process Manager\nExec={}\nTerminal=true\nComment=Process Manager TUI\n",
            exe_path.display()
        );
        let home = std::env::var("HOME").unwrap_or_default();
        let desktop_file = std::path::PathBuf::from(&home).join("Desktop/prc.desktop");
        match std::fs::write(&desktop_file, desktop_entry) {
            Ok(_) => format!("Shortcut created at {}", desktop_file.display()),
            Err(e) => format!("Failed: {}", e),
        }
    }
}
