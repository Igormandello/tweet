use tui::{backend::Backend, Frame, widgets::Tabs};
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);

    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        // 0 => draw_first_tab(f, app, chunks[1]),
        // 1 => draw_second_tab(f, app, chunks[1]),
        // 2 => draw_third_tab(f, app, chunks[1]),
        _ => {}
    };
}