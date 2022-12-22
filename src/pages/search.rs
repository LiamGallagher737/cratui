use crate::{
    app::App,
    cargo::{self, search::Crate, SearchResponse},
    ui::rgb,
};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use smart_default::SmartDefault;
use std::thread::{self, JoinHandle};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Default, Debug)]
pub struct SearchPage {
    query_state: QueryState,
    results_state: Option<ResultsState>,
    per_page: usize,
    expanded_help_message: bool,
}

#[derive(SmartDefault, Debug)]
pub struct QueryState {
    #[default(true)]
    active: bool,
    query: String,
    cursor: usize,
}

#[derive(Default, Debug)]
pub struct ResultsState {
    index: usize,
    page: usize,
    results: Vec<Vec<Crate>>,
    query: String,
    loaded_all: bool,
    request_handle: Option<JoinHandle<Result<SearchResponse>>>,
}

impl SearchPage {
    fn next_index(&mut self) {
        if let Some(state) = &mut self.results_state {
            if state.index < state.results[state.page].len() - 1 {
                state.index += 1;
            } else if state.page < state.results.len() - 1 {
                state.page += 1;
                state.index = 0;
            } else {
                state.page = 0;
                state.index = 0;
            }
        }
    }
    fn previous_index(&mut self) {
        if let Some(state) = &mut self.results_state {
            if state.index > 0 {
                state.index -= 1;
            } else if state.page > 0 {
                state.page -= 1;
                state.index = self.per_page;
            } else {
                let last_page = state.results.len() - 1;
                state.page = last_page;
                state.index = state.results[last_page].len() - 1;
            }
        }
    }
    fn next_page(&mut self) {
        if let Some(state) = &mut self.results_state {
            let last_page = state.results.len() - 1;
            if state.page < last_page {
                state.page += 1;
                if state.page == last_page && state.index >= state.results[last_page].len() {
                    state.index = state.results[last_page].len() - 1;
                }
            } else {
                state.page = 0;
            }
        }
    }
    fn previous_page(&mut self) {
        if let Some(state) = &mut self.results_state {
            let last_page = state.results.len() - 1;
            if state.page > 0 {
                state.page -= 1;
            } else {
                state.page = last_page;
                if state.index >= state.results[last_page].len() {
                    state.index = state.results[last_page].len() - 1;
                }
            }
        }
    }
    fn selected_crate(&self) -> Option<Crate> {
        let state = self.results_state.as_ref()?;
        Some(state.results.get(state.page)?.get(state.index)?.to_owned())
    }
}

pub fn event(app: &mut App, e: Event) -> bool {
    if let Event::Key(key) = e {
        if app.search_page.query_state.active {
            let state = &mut app.search_page.query_state;
            match key.code {
                KeyCode::Char(c) => {
                    state.query.push(c);
                    state.cursor += 1;
                }
                KeyCode::Backspace => {
                    if state.cursor > 0 {
                        state.query.remove(state.cursor - 1);
                        state.cursor -= 1;
                    }
                }
                KeyCode::Delete => {
                    if state.cursor < state.query.len() {
                        state.query.remove(state.cursor);
                    }
                }
                KeyCode::Left => {
                    if state.cursor > 0 {
                        state.cursor -= 1;
                    }
                }
                KeyCode::Right => {
                    if state.cursor < state.query.len() {
                        state.cursor += 1;
                    }
                }
                KeyCode::Enter => {
                    app.search_page.results_state = Some(ResultsState {
                        index: 0,
                        page: 0,
                        results: vec![],
                        query: state.query.to_owned(),
                        loaded_all: false,
                        request_handle: None,
                    });
                    state.active = false;
                }
                KeyCode::Esc => {
                    state.active = false;
                }
                _ => return true,
            }
        } else {
            match key.code {
                KeyCode::Char('s') => {
                    app.search_page.query_state.active = true;
                }
                KeyCode::Char('?') => {
                    app.search_page.expanded_help_message = !app.search_page.expanded_help_message
                }
                // Movement
                KeyCode::Up | KeyCode::Char('j') => app.search_page.previous_index(),
                KeyCode::Down | KeyCode::Char('k') => app.search_page.next_index(),
                KeyCode::Left | KeyCode::Char('h') => app.search_page.previous_page(),
                KeyCode::Right | KeyCode::Char('l') => app.search_page.next_page(),
                // Links
                KeyCode::Char('c') => {
                    if let Some(c) = app.search_page.selected_crate() {
                        open::that(format!("https://crates.io/crates/{}", c.id))
                            .unwrap_or_else(|_| app.error());
                    } else {
                        app.warn();
                    }
                }
                KeyCode::Char('d') => {
                    if let Some(c) = app.search_page.selected_crate() {
                        open::that(format!("https://docs.rs/{}/latest", c.id))
                            .unwrap_or_else(|_| app.error());
                    } else {
                        app.warn();
                    }
                }
                KeyCode::Char('g') => {
                    if let Some(Some(repo)) = app.search_page.selected_crate().map(|c| c.repository)
                    {
                        open::that(repo).unwrap_or_else(|_| app.error());
                    } else {
                        app.warn();
                    }
                }
                // Actions
                KeyCode::Char('a') => {
                    if let Some(c) = app.search_page.selected_crate() {
                        cargo::add(c.id, c.max_stable_version.unwrap_or(c.max_version))
                            .unwrap_or_else(|_| app.error());
                    } else {
                        app.warn();
                    }
                }
                KeyCode::Char('r') => {
                    if let Some(c) = app.search_page.selected_crate() {
                        cargo::remove(c.id).unwrap_or_else(|_| app.error());
                    } else {
                        app.warn();
                    }
                }
                KeyCode::Char('i') => {
                    if let Some(c) = app.search_page.selected_crate() {
                        cargo::install(c.id).unwrap();
                    } else {
                        app.warn();
                    }
                }
                KeyCode::Char('f') => {
                    if let Some(c) = app.search_page.selected_crate() {
                        app.config.favourites.crates.push(c.id);
                    } else {
                        app.warn();
                    }
                }
                _ => return true,
            }
        }
    } else if let Event::Resize(_, height) = e {
        if height < 13 {
            return true;
        }
        app.search_page.per_page = (height as usize - 13) / 4 + 1;
        if let Some(state) = &mut app.search_page.results_state {
            let crates: Vec<Crate> = state.results.to_owned().into_iter().flatten().collect();
            let pages = crates
                .chunks(app.search_page.per_page)
                .map(|chunk| chunk.to_vec())
                .collect();
            state.results = pages;
        }
    }

    false
}

pub fn update<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .vertical_margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let query_state = &mut app.search_page.query_state;

    let search_bar_chunk = Layout::default()
        .horizontal_margin(3)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(query_state.query.len().max(9) as u16 + 1),
            Constraint::Min(0),
        ])
        .split(chunks[0])[0];

    let search_style = if query_state.active {
        Style::default().fg(rgb(app.config.colors.primary))
    } else {
        Style::default()
    };

    let text = Text::styled(&query_state.query, Style::default());
    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().borders(Borders::BOTTOM))
            .style(search_style),
        search_bar_chunk,
    );

    if let Some(state) = app.search_page.results_state.as_mut() {
        if let Some(handle) = state.request_handle.as_mut() {
            if handle.is_finished() {
                let handle = state.request_handle.take().unwrap();
                let res = handle.join().unwrap().unwrap();
                state.results.push(res.crates);
                state.loaded_all = res.meta.next_page.is_none()
                    || state.results.len() > app.config.search.max_pages as usize;
            }
        } else if !state.loaded_all {
            let query = state.query.to_owned();
            let page = state.results.len();
            let limit = app.search_page.per_page;
            let handle = thread::spawn(move || cargo::search(&query, page, limit));
            state.request_handle = Some(handle);
        }
    }

    let results_state = match &app.search_page.results_state {
        Some(e) => e,
        None => return,
    };

    if results_state.results.is_empty() {
        return;
    }

    let list_chunks = Layout::default()
        .constraints(vec![Constraint::Length(4); app.search_page.per_page])
        .split(chunks[1]);

    let crates = &results_state.results[results_state.page];
    for (i, c) in crates.iter().enumerate() {
        let selected = i == results_state.index;

        let style = if selected {
            Style::default().fg(rgb(app.config.colors.primary))
        } else {
            Style::default()
        };

        let prefix = if selected { " │ " } else { "   " };

        let text = Text {
            lines: vec![
                Spans::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(c.id.to_owned(), style.add_modifier(Modifier::BOLD)),
                ]),
                Spans::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(c.description.to_owned(), style.add_modifier(Modifier::DIM)),
                ]),
                Spans::from(vec![
                    Span::styled(prefix, style),
                    Span::styled("Downloads: ", style.add_modifier(Modifier::DIM)),
                    Span::styled(c.downloads.to_string(), style.add_modifier(Modifier::DIM)),
                    Span::styled(" • ", style.add_modifier(Modifier::DIM)),
                    Span::styled("Recent: ", style.add_modifier(Modifier::DIM)),
                    Span::styled(
                        c.recent_downloads.to_string(),
                        style.add_modifier(Modifier::DIM),
                    ),
                ]),
            ],
        };

        f.render_widget(Paragraph::new(text), list_chunks[i]);
    }

    if !app.search_page.expanded_help_message {
        let footer_chunks = Layout::default()
            .horizontal_margin(3)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(chunks[2]);

        let mut dots = vec![];
        for i in 0..results_state.results.len() {
            if i == results_state.page {
                dots.push(Span::styled(
                    "• ",
                    Style::default().add_modifier(Modifier::BOLD),
                ));
            } else {
                dots.push(Span::styled(
                    "• ",
                    Style::default().add_modifier(Modifier::DIM),
                ));
            }
        }
        f.render_widget(Paragraph::new(Spans::from(dots)), footer_chunks[0]);

        let help_text = Text::from(Spans::from(vec![
            Span::styled(
                "c/d/g ",
                Style::default().add_modifier(Modifier::BOLD | Modifier::DIM),
            ),
            Span::styled("links", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(" • ", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(
                "a ",
                Style::default().add_modifier(Modifier::BOLD | Modifier::DIM),
            ),
            Span::styled("add", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(" • ", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(
                "i ",
                Style::default().add_modifier(Modifier::BOLD | Modifier::DIM),
            ),
            Span::styled("install", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(" • ", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(
                "f ",
                Style::default().add_modifier(Modifier::BOLD | Modifier::DIM),
            ),
            Span::styled("favourite", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(" • ", Style::default().add_modifier(Modifier::DIM)),
            Span::styled(
                "? ",
                Style::default().add_modifier(Modifier::BOLD | Modifier::DIM),
            ),
            Span::styled("help", Style::default().add_modifier(Modifier::DIM)),
        ]));
        f.render_widget(Paragraph::new(help_text), footer_chunks[2]);
    } else {
        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(3)
            .constraints([Constraint::Length(4), Constraint::Length(15)].repeat(3))
            .split(chunks[2]);

        let keybinds = [
            vec![("c", "crates.io"), ("d", "docs.rs"), ("g", "git repo")],
            vec![("a", "add"), ("r", "remove"), ("i", "install")],
            vec![("f", "favourite"), ("q", "quit"), ("?", "close help")],
        ];

        let key_style = Style::default().add_modifier(Modifier::BOLD);
        let description_style = Style::default().add_modifier(Modifier::DIM);

        for (i, k) in keybinds.iter().enumerate() {
            let i = i * 2;

            let keys = Text {
                lines: k
                    .iter()
                    .map(|(key, _)| Spans::from(Span::styled(*key, key_style)))
                    .collect(),
            };
            f.render_widget(Paragraph::new(keys), footer_chunks[i]);

            let descriptions = Text {
                lines: k
                    .iter()
                    .map(|(_, description)| {
                        Spans::from(Span::styled(*description, description_style))
                    })
                    .collect(),
            };
            f.render_widget(Paragraph::new(descriptions), footer_chunks[i + 1]);
        }
    }
}
