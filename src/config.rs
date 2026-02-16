use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "proc-manager", version, about = "A high-performance cross-platform process manager TUI")]
pub struct Cli {
    /// Refresh interval in milliseconds
    #[arg(short = 't', long, default_value_t = 1000)]
    pub tick_rate: u64,

    /// Default sort column (pid, name, user, cpu, memory, status, threads, time, disk_read, disk_write, ppid)
    #[arg(short, long, default_value = "cpu")]
    pub sort: String,

    /// Sort ascending
    #[arg(short = 'a', long, default_value_t = false)]
    pub ascending: bool,

    /// Path to config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Run inside current terminal (skip standalone window launch)
    #[arg(long, hide = true)]
    pub embedded: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_tick_rate")]
    pub tick_rate_ms: u64,

    #[serde(default = "default_sort_column")]
    pub sort_column: String,

    #[serde(default)]
    pub sort_ascending: bool,

    #[serde(default = "default_history_len")]
    pub history_len: usize,

    #[serde(default = "default_true")]
    pub standalone_window: bool,

    #[serde(default)]
    pub keep_dead_pins: bool,

    #[serde(default = "default_columns")]
    pub visible_columns: Vec<String>,

    #[serde(default)]
    pub compact_view: bool,

    #[serde(default)]
    pub theme: ThemeConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_true")]
    pub color_enabled: bool,

    #[serde(default = "default_true")]
    pub bold_headers: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            color_enabled: true,
            bold_headers: true,
        }
    }
}

fn default_tick_rate() -> u64 {
    1000
}
fn default_sort_column() -> String {
    "cpu".to_string()
}
fn default_history_len() -> usize {
    60
}
fn default_true() -> bool {
    true
}
fn default_columns() -> Vec<String> {
    vec![
        "pid".to_string(),
        "name".to_string(),
        "user".to_string(),
        "cpu".to_string(),
        "memory".to_string(),
        "status".to_string(),
        "threads".to_string(),
        "time".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tick_rate_ms: default_tick_rate(),
            sort_column: default_sort_column(),
            sort_ascending: false,
            history_len: default_history_len(),
            standalone_window: true,
            keep_dead_pins: false,
            compact_view: false,
            visible_columns: default_columns(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Config {
    pub fn load(cli: &Cli) -> Result<Self> {
        let config_path = cli.config.clone().or_else(default_config_path);

        let mut config = if let Some(path) = &config_path {
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                toml::from_str(&content)?
            } else {
                Config::default()
            }
        } else {
            Config::default()
        };

        // CLI overrides
        if cli.tick_rate != 1000 {
            config.tick_rate_ms = cli.tick_rate;
        }
        if cli.sort != "cpu" {
            config.sort_column = cli.sort.clone();
        }
        if cli.ascending {
            config.sort_ascending = true;
        }

        // Clamp tick rate
        config.tick_rate_ms = config.tick_rate_ms.max(200).min(10000);
        config.history_len = config.history_len.max(10).min(300);

        Ok(config)
    }

    pub fn parse_sort_column(&self) -> crate::data::process::SortColumn {
        use crate::data::process::SortColumn;
        match self.sort_column.to_lowercase().as_str() {
            "pid" => SortColumn::Pid,
            "name" => SortColumn::Name,
            "user" => SortColumn::User,
            "cpu" => SortColumn::Cpu,
            "memory" | "mem" => SortColumn::Memory,
            "status" => SortColumn::Status,
            "threads" => SortColumn::Threads,
            "time" | "start" => SortColumn::StartTime,
            "disk_read" | "diskread" => SortColumn::DiskRead,
            "disk_write" | "diskwrite" => SortColumn::DiskWrite,
            "ppid" | "parent" => SortColumn::Ppid,
            _ => SortColumn::Cpu,
        }
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = default_config_path() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = toml::to_string_pretty(self)?;
            std::fs::write(&path, content)?;
        }
        Ok(())
    }
}

pub fn default_config_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "proc-manager", "proc-manager")
        .map(|dirs| dirs.config_dir().join("config.toml"))
}
