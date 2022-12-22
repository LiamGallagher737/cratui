use anyhow::Result;
use app::{App, run_app};
use config::{load_config, save_config};
use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod cargo;
mod ui;
mod config;
mod pages;

const BINARY_NAME: &str = "cratui";

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create and run App
    let tick_rate = Duration::from_millis(250);
    let mut app = App::new(load_config().unwrap_or_default());
    let res = run_app(&mut terminal, &mut app, tick_rate);

    // Save Config
    if let Err(e) = save_config(app.config) {
        println!("Error saving config!\n{e:#?}");
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
