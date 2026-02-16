# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build                    # Dev build
cargo build --release          # Release build → target/release/prc.exe
cargo run                      # Run (launches standalone window by default)
cargo run -- --embedded        # Run inside current terminal
cargo run -- --tick-rate 500   # Custom refresh rate
```

No test suite exists. Verify changes with `cargo build --release`.

## Architecture

**Binary:** `prc` (configured via `[[bin]]` in Cargo.toml)

### TEA Pattern (The Elm Architecture)

The app follows Event → Action → State → Render:

1. **EventHandler** (`src/event.rs`) — async tokio task using `select!` over a tick interval + crossterm input polling. Tick rate is dynamically adjustable via a `watch` channel.

2. **Input Mapping** (`src/input.rs`) — `map_key(key, mode) → Action`. All keys are normalized to lowercase via `normalize_key()` so keybindings are case-insensitive. Each `AppMode` has its own key map function.

3. **Dispatch** (`src/app.rs`) — `App::dispatch(action)` is the single state mutation point. All state changes go through here.

4. **Rendering** (`src/ui/`) — Pure functions taking `&App` and drawing to a `Frame`. No business logic in UI code.

### App Modes

`AppMode` enum gates input handling and overlay rendering: `Normal`, `Search`, `Dialog`, `Help`, `NewProcess`, `Settings`, `ContextMenu`, `HiddenList`.

### Data Flow (each tick)

```
collector.refresh() → sysinfo API calls
  → process_list.update(entries) → stable sort preserving order, rebuild filter
  → resolve_pinned_pids() → map names to current PIDs
  → prune_dead_pins(5s) → remove ghost entries if keep_dead_pins is off
  → push history ring buffers → sparkline data
```

### Process List (`src/data/process.rs`)

`ProcessList` maintains `entries: Vec<ProcessEntry>` and `filtered_indices: Vec<usize>`. Filtering excludes hidden processes and applies search. Pinned processes sort to the top in the UI (handled by `process_table.rs` via `pinned_visible()` / `unpinned_visible()`).

**Dead pins:** When a pinned process dies, a ghost entry (`is_dead: true`) is kept with last known data, rendered in gray. Removed after 5 seconds unless `keep_dead_pins` config is on. On app restart, no ghosts are created for missing processes.

### Persistence

- **Config:** `directories::ProjectDirs` → `config_dir/config.toml` (TOML via serde)
- **State:** `data_dir/state.toml` — pinned and hidden process names (by name, not PID)

### Responsive Layout (`src/ui/layout.rs`)

- `<80w`: 2-column dashboard
- `80-119w`: 3-column (CPU, Memory, Network)
- `>=120w`: 4-column (+ Disk panel)
- `<20h`: Reduced dashboard height

### Standalone Window (`src/main.rs`)

Without `--embedded`, the binary relaunches itself in `conhost.exe` (Windows) to avoid terminal tabs. The `--embedded` flag is hidden from CLI help. Config's `standalone_window` bool controls this.

### Mouse Support

`process_table_area` stored as `Cell<Rect>` for interior mutability from the immutable `draw()` context. Left-click selects, right-click opens context menu, scroll wheel navigates.

## Key Conventions

- Platform-specific code uses `#[cfg(target_os = "windows")]` with `#[cfg(not(...))]` fallbacks
- Windows thread counting uses Win32 ToolHelp API (`CreateToolhelp32Snapshot`)
- Settings page items are data-driven (`SettingsItem` enum: Slider/Toggle/Cycle/Action)
- Key conflicts are avoided by case-insensitive normalization — each function gets a unique lowercase letter
