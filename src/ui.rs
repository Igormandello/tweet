use std::cmp::max;
use tui::{backend::Backend, Frame};
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.size());

    render_tweet(f, chunks[0], app);
    render_commands(f, chunks[1]);
}

fn render_tweet<B>(f: &mut Frame<B>, area: Rect, app: &mut App) where B: Backend {
    let block = Block::default().borders(Borders::ALL);
    let tweet = app.current_tweet.clone().unwrap_or(String::new());
    let text = Span::from(tweet.as_ref());
    let tweet = Paragraph::new(text).wrap(Wrap { trim: false });

    let width = max(area.width, 60);
    let height = max(area.height, 14);

    let center_chunk = Layout::default()
        .constraints([Constraint::Min(0)])
        .horizontal_margin((width - 60) / 2)
        .vertical_margin((height - 14) / 2)
        .split(area)[0];

    let text_chunk = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .horizontal_margin(4)
        .vertical_margin(2)
        .split(center_chunk)[0];

    f.render_widget(block, center_chunk);
    f.render_widget(tweet, text_chunk);
}

fn render_commands<B>(f: &mut Frame<B>, area: Rect) where B: Backend {
    let text = Spans::from(vec![
        Span::styled(" q ", Style::default().add_modifier(Modifier::REVERSED)),
        Span::raw(" quit "),
        Span::styled(" < ", Style::default().add_modifier(Modifier::REVERSED)),
        Span::raw(" previous tweet "),
        Span::styled(" > ", Style::default().add_modifier(Modifier::REVERSED)),
        Span::raw(" next tweet "),
    ]);

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Commands",
        Style::default().add_modifier(Modifier::BOLD),
    ));

    let commands = Paragraph::new(text).block(block).wrap(Wrap { trim: false });

    f.render_widget(commands, area);
}