use std::cmp::max;
use regex::Regex;

use tui::{backend::Backend, Frame};
use tui::layout::{Alignment, Constraint, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::app::App;

lazy_static! {
    static ref TWITTER_LINK_REGEX: Regex = Regex::new("( )?https://t.co/\\w*").unwrap();
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.size());

    render_tweet(f, chunks[0], app);
    render_commands(f, chunks[1]);
}

fn render_tweet<B>(f: &mut Frame<B>, area: Rect, app: &mut App) where B: Backend {
    let tweet_index = format!(" {}/{} ", app.current_tweet + 1, app.tweets.len());
    let block = Block::default()
        .borders(Borders::ALL)
        .title(tweet_index)
        .title_alignment(Alignment::Right);

    let tweet = app.tweets[app.current_tweet].clone();
    let filtered_tweet_text = &*TWITTER_LINK_REGEX.replace_all(&tweet.full_text, "");
    let text = vec![
        Spans::from(vec![
            Span::styled(tweet.user.name, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" @"),
            Span::from(tweet.user.screen_name),
        ]),
        Spans::default(),
        Spans::from(filtered_tweet_text),
    ];

    let tweet = Paragraph::new(text).wrap(Wrap { trim: false });

    let width = max(area.width, 60);
    let height = max(area.height, 16);

    let center_chunk = Layout::default()
        .constraints([Constraint::Min(0)])
        .horizontal_margin((width - 60) / 2)
        .vertical_margin((height - 16) / 2)
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