mod action;
mod app;
mod config;
mod data;
mod event;
mod input;
mod ui;
mod util;

use anyhow::Result;
use app::App;
use clap::Parser;
use config::Cli;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::Config::load(&cli)?;

    // If not embedded and standalone window is enabled, relaunch in a standalone window
    if !cli.embedded && config.standalone_window {
        return launch_standalone(&cli);
    }

    // Set console title
    crossterm::execute!(std::io::stdout(), crossterm::terminal::SetTitle("Process Manager"))?;

    let mut terminal = ratatui::init();
    execute!(std::io::stdout(), EnableMouseCapture)?;

    let result = App::new(config).run(&mut terminal).await;

    execute!(std::io::stdout(), DisableMouseCapture)?;
    ratatui::restore();
    result
}

fn launch_standalone(cli: &Cli) -> Result<()> {
    let exe = std::env::current_exe()?;

    // Build args: pass through all original args + add --embedded
    let mut args = vec!["--embedded".to_string()];

    if cli.tick_rate != 1000 {
        args.push("--tick-rate".to_string());
        args.push(cli.tick_rate.to_string());
    }
    if cli.sort != "cpu" {
        args.push("--sort".to_string());
        args.push(cli.sort.clone());
    }
    if cli.ascending {
        args.push("--ascending".to_string());
    }
    if let Some(config_path) = &cli.config {
        args.push("--config".to_string());
        args.push(config_path.to_string_lossy().to_string());
    }

    #[cfg(target_os = "windows")]
    {
        // Use conhost.exe for a clean, tab-free window
        // conhost.exe opens a classic console window without Windows Terminal tabs
        let mut cmd = std::process::Command::new("conhost.exe");
        cmd.arg(exe.to_string_lossy().as_ref());
        for arg in &args {
            cmd.arg(arg);
        }
        cmd.spawn()?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On Linux/macOS, try common terminal emulators
        let terminals = ["x-terminal-emulator", "xterm", "gnome-terminal", "konsole"];
        let mut launched = false;
        for term in &terminals {
            let mut cmd = std::process::Command::new(term);
            cmd.arg("-e");
            cmd.arg(exe.to_string_lossy().as_ref());
            for arg in &args {
                cmd.arg(arg);
            }
            if cmd.spawn().is_ok() {
                launched = true;
                break;
            }
        }
        if !launched {
            // Fallback: just run embedded in current terminal
            let config = config::Config::load(cli)?;
            crossterm::execute!(
                std::io::stdout(),
                crossterm::terminal::SetTitle("Process Manager")
            )?;
            let mut terminal = ratatui::init();
            execute!(std::io::stdout(), EnableMouseCapture)?;
            let result = App::new(config).run(&mut terminal).await;
            execute!(std::io::stdout(), DisableMouseCapture)?;
            ratatui::restore();
            return result;
        }
    }

    Ok(())
}
