use crate::{
    config::Config,
    pages::search::{self, SearchPage},
    ui::{draw_ui, rgb},
};
use anyhow::{anyhow, Result};
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui::{backend::Backend, Terminal, style::Color};

const TAB_COUNT: u8 = 3;

#[derive(Default)]
pub struct App {
    pub config: Config,
    pub tab: u8,
    pub warning_timer: Option<Instant>,
    pub error_timer: Option<Instant>,

    pub search_page: SearchPage,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }
    pub fn warn(&mut self) {
        self.warning_timer = Some(Instant::now());
    }
    pub fn error(&mut self) {
        self.error_timer = Some(Instant::now());
    }
    pub fn is_error(&self) -> bool {
        if let Some(i) = self.error_timer {
            if i.elapsed() < Duration::from_millis(250) {
                return true;
            }
        }
        false
    }
    pub fn is_warning(&self) -> bool {
        if let Some(i) = self.warning_timer {
            if i.elapsed() < Duration::from_millis(250) {
                return true;
            }
        }
        false
    }
    pub fn primary_color(&self) -> Color {
        if self.is_error() {
            rgb(self.config.colors.error)
        } else if self.is_warning() {
            rgb(self.config.colors.warn)
        } else {
            rgb(self.config.colors.primary)
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| draw_ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            let event = event::read()?;

            let can_use = match app.tab {
                0 => search::event(app, event.clone()),
                _ => return Err(anyhow!("Tab index out of range")),
            };

            if let Event::Key(key) = event {
                // Runs no matter what
                if let KeyCode::F(4) = key.code {
                    return Ok(());
                }

                if !can_use {
                    continue;
                }

                // Only runs in current page didn't use it
                match key.code {
                    // Exit app
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),

                    // Switch Tabs
                    KeyCode::Tab => app.tab = (app.tab + 1) % TAB_COUNT,
                    KeyCode::BackTab => app.tab = (app.tab + TAB_COUNT - 1) % TAB_COUNT,
                    KeyCode::Char('1') => app.tab = 0,
                    KeyCode::Char('2') => app.tab = 1,
                    KeyCode::Char('3') => app.tab = 2,

                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
