use crate::app::App;
use crossterm::event::Event;
use tui::{backend::Backend, layout::Rect, Frame};

#[derive(Default, Debug)]
pub struct ManagePage {
    _expanded_help_message: bool,
}

pub fn event(_app: &mut App, _e: Event) -> bool {
    true
}

pub fn update<B: Backend>(_f: &mut Frame<B>, _app: &mut App, _area: Rect) {}
