pub mod context_menu;
pub mod cpu_panel;
pub mod dialog;
pub mod disk_panel;
pub mod header;
pub mod help;
pub mod hidden_list;
pub mod layout;
pub mod memory_panel;
pub mod network_panel;
pub mod process_table;
pub mod settings;
pub mod statusbar;
pub mod theme;
pub mod theme_picker;
pub mod view_overview;
pub mod view_percpu;
pub mod view_resource_graphs;
pub mod view_mini;
pub mod view_sysinfo;

use ratatui::Frame;

use crate::app::{App, ViewMode};
use layout::AppLayout;

pub fn draw(frame: &mut Frame, app: &App) {
    let layout = AppLayout::new(frame.area(), app.view_mode, app.compact_view);

    header::draw(frame, app, layout.header);

    match app.view_mode {
        ViewMode::Default => {
            if !app.compact_view {
                cpu_panel::draw(frame, app, layout.cpu_panel);
                memory_panel::draw(frame, app, layout.memory_panel);
                network_panel::draw(frame, app, layout.network_panel);

                if let Some(disk_area) = layout.disk_panel {
                    disk_panel::draw(frame, app, disk_area);
                }
            }

            process_table::draw(frame, app, layout.process_table);
        }
        ViewMode::PerCpuChart => {
            view_percpu::draw(frame, app, layout.main_content.unwrap());
        }
        ViewMode::ResourceGraphs => {
            view_resource_graphs::draw(frame, app, layout.main_content.unwrap());
        }
        ViewMode::SystemOverview => {
            view_overview::draw(frame, app, layout.main_content.unwrap());
        }
        ViewMode::SystemInfo => {
            view_sysinfo::draw(frame, app, layout.main_content.unwrap());
        }
        ViewMode::MiniMonitor => {
            view_mini::draw(frame, app, layout.main_content.unwrap());
        }
    }

    statusbar::draw(frame, app, layout.statusbar);

    // Overlay dialogs
    match app.mode {
        crate::app::AppMode::Dialog => dialog::draw_kill_confirm(frame, app),
        crate::app::AppMode::Help => help::draw(frame, app),
        crate::app::AppMode::NewProcess => dialog::draw_new_process(frame, app),
        crate::app::AppMode::Settings => settings::draw(frame, app),
        crate::app::AppMode::ContextMenu => context_menu::draw(frame, app),
        crate::app::AppMode::HiddenList => hidden_list::draw(frame, app),
        crate::app::AppMode::ThemePicker => theme_picker::draw(frame, app),
        _ => {}
    }
}
