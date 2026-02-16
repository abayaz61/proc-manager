use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::Action;
use crate::app::AppMode;
use crate::data::process::SortColumn;

/// Normalize key to lowercase for case-insensitive matching
fn normalize_key(key: KeyEvent) -> KeyEvent {
    let code = match key.code {
        KeyCode::Char(c) => KeyCode::Char(c.to_ascii_lowercase()),
        other => other,
    };
    KeyEvent::new(code, key.modifiers)
}

pub fn map_key(key: KeyEvent, mode: AppMode) -> Action {
    let key = normalize_key(key);
    match mode {
        AppMode::Normal => map_normal(key),
        AppMode::Search => map_search(key),
        AppMode::Dialog => map_dialog(key),
        AppMode::Help => map_help(key),
        AppMode::NewProcess => map_new_process(key),
        AppMode::Settings => map_settings(key),
        AppMode::ContextMenu => map_context_menu(key),
        AppMode::HiddenList => map_hidden_list(key),
        AppMode::ThemePicker => map_theme_picker(key),
        AppMode::PidFilter => map_pid_filter(key),
    }
}

fn map_normal(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,
        KeyCode::Up | KeyCode::Char('k') => Action::ScrollUp,
        KeyCode::Down | KeyCode::Char('j') => Action::ScrollDown,
        KeyCode::PageUp => Action::PageUp,
        KeyCode::PageDown => Action::PageDown,
        KeyCode::Home => Action::Home,
        KeyCode::End => Action::End,
        KeyCode::Char('/') => Action::ToggleSearch,
        KeyCode::Char('r') => Action::ReverseSortOrder,
        KeyCode::Char('x') => Action::KillProcess,
        KeyCode::Char('n') => Action::StartNewProcess,
        KeyCode::Char('p') => Action::TogglePin,
        KeyCode::Char('c') => Action::ClearPins,
        KeyCode::Char('?') => Action::ToggleHelp,
        KeyCode::Char('s') => Action::OpenSettings,
        KeyCode::Char('h') => Action::HideProcess,
        KeyCode::Char('v') => Action::OpenHiddenList,
        KeyCode::F(1) | KeyCode::Char('1') => Action::SortBy(SortColumn::Pid),
        KeyCode::F(2) | KeyCode::Char('2') => Action::SortBy(SortColumn::Name),
        KeyCode::F(3) | KeyCode::Char('3') => Action::SortBy(SortColumn::User),
        KeyCode::F(4) | KeyCode::Char('4') => Action::SortBy(SortColumn::Cpu),
        KeyCode::F(5) | KeyCode::Char('5') => Action::SortBy(SortColumn::Memory),
        KeyCode::F(6) | KeyCode::Char('6') => Action::SortBy(SortColumn::Status),
        KeyCode::F(7) | KeyCode::Char('7') => Action::SortBy(SortColumn::Threads),
        KeyCode::F(8) | KeyCode::Char('8') => Action::SortBy(SortColumn::StartTime),
        KeyCode::F(9) | KeyCode::Char('9') => Action::SortBy(SortColumn::DiskRead),
        KeyCode::F(10) | KeyCode::Char('0') => Action::SortBy(SortColumn::DiskWrite),
        KeyCode::F(11) => Action::SortBy(SortColumn::Ppid),
        KeyCode::Tab => Action::CycleViewMode,
        KeyCode::Char('d') => Action::ToggleCompactView,
        KeyCode::Char('t') => Action::OpenThemePicker,
        KeyCode::Char('f') => Action::OpenPidFilter,
        _ => Action::Noop,
    }
}

fn map_search(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::ToggleSearch,
        KeyCode::Enter => Action::SearchSubmit,
        KeyCode::Backspace => Action::SearchBackspace,
        KeyCode::Char(c) => Action::SearchInput(c),
        _ => Action::Noop,
    }
}

fn map_dialog(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => Action::KillProcessConfirm,
        KeyCode::Char('n') | KeyCode::Esc => Action::CancelDialog,
        _ => Action::Noop,
    }
}

fn map_help(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => Action::ToggleHelp,
        _ => Action::Noop,
    }
}

fn map_hidden_list(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc | KeyCode::Char('v') => Action::CancelDialog,
        KeyCode::Up | KeyCode::Char('k') => Action::HiddenListUp,
        KeyCode::Down | KeyCode::Char('j') => Action::HiddenListDown,
        KeyCode::Enter | KeyCode::Delete => Action::HiddenListUnhide,
        KeyCode::Char('a') => Action::HiddenListUnhideAll,
        _ => Action::Noop,
    }
}

fn map_context_menu(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::CancelDialog,
        KeyCode::Up | KeyCode::Char('k') => Action::ContextMenuUp,
        KeyCode::Down | KeyCode::Char('j') => Action::ContextMenuDown,
        KeyCode::Enter => Action::ContextMenuSelect,
        _ => Action::Noop,
    }
}

fn map_settings(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::CancelDialog,
        KeyCode::Up | KeyCode::Char('k') => Action::SettingsUp,
        KeyCode::Down | KeyCode::Char('j') => Action::SettingsDown,
        KeyCode::Left => Action::SettingsLeft,
        KeyCode::Right => Action::SettingsRight,
        KeyCode::Enter => Action::SettingsSelect,
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::SettingsSave,
        _ => Action::Noop,
    }
}

fn map_new_process(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::CancelDialog,
        KeyCode::Enter => Action::SubmitNewProcess,
        KeyCode::Backspace => Action::NewProcessBackspace,
        KeyCode::Char(c) => Action::NewProcessInput(c),
        _ => Action::Noop,
    }
}

fn map_pid_filter(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::CancelDialog,
        KeyCode::Enter => Action::PidFilterSubmit,
        KeyCode::Backspace => Action::PidFilterBackspace,
        KeyCode::Char(c) if c.is_ascii_digit() => Action::PidFilterInput(c),
        _ => Action::Noop,
    }
}

fn map_theme_picker(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc | KeyCode::Char('t') => Action::CancelDialog,
        KeyCode::Up | KeyCode::Char('k') => Action::ThemePickerUp,
        KeyCode::Down | KeyCode::Char('j') => Action::ThemePickerDown,
        KeyCode::Enter => Action::ThemePickerSelect,
        _ => Action::Noop,
    }
}
