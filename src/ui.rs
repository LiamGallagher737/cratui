use crate::{app::App, pages::search};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Paragraph, Tabs},
    Frame,
};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.size());

    draw_header(f, app, chunks[1]);

    match app.tab {
        0 => search::update(f, app, chunks[2]),
        _ => {}
    }
}

fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Length(1)])
        .split(area);

    // Title
    let title = Text::styled(
        " cratui ",
        Style::default()
            .bg(app.primary_color())
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(
        Paragraph::new(title),
        Layout::default()
            .constraints([Constraint::Length(1)])
            .horizontal_margin(3)
            .split(chunks[0])[0],
    );

    // Tabs
    let tabs = Tabs::new(vec![
        Span::raw("search").into(),
        Span::raw("manage").into(),
        Span::raw("favourites").into(),
    ])
    .style(Style::default().add_modifier(Modifier::DIM))
    .highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .remove_modifier(Modifier::DIM),
    )
    .divider("│")
    .select(app.tab as usize);
    f.render_widget(
        tabs,
        Layout::default()
            .constraints([Constraint::Length(1)])
            .horizontal_margin(2)
            .split(chunks[1])[0],
    );
}

pub fn rgb(rgb: [u8; 3]) -> Color {
    Color::Rgb(rgb[0], rgb[1], rgb[2])
}
