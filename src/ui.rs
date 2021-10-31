use std::cmp::max;
use std::collections::HashMap;

use image::imageops::FilterType;
use regex::Regex;
use tui::{backend::Backend, Frame};
use tui::layout::{Alignment, Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::widgets::canvas::{Canvas, Points};

use crate::app::App;

lazy_static! {
    static ref TWITTER_LINK_REGEX: Regex = Regex::new("( )?https://t.co/\\w*").unwrap();
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.size());

    render_commands(f, chunks[1], app);

    if !app.showing_image {
        render_tweet(f, chunks[0], app);
    } else {
        render_image(f, chunks[0], app);
    }
}

fn render_tweet<B>(f: &mut Frame<B>, area: Rect, app: &App) where B: Backend {
    let tweet_index = format!(" {}/{} ", app.current_tweet + 1, app.tweets.len());
    let block = Block::default()
        .borders(Borders::ALL)
        .title(tweet_index)
        .title_alignment(Alignment::Right);

    let tweet = &app.tweets[app.current_tweet];
    let filtered_tweet_text = &*TWITTER_LINK_REGEX.replace_all(&tweet.full_text, "");
    let text = vec![
        Spans::from(vec![
            Span::styled(&tweet.user.name, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" @"),
            Span::raw(&tweet.user.screen_name),
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

fn render_image<B>(f: &mut Frame<B>, area: Rect, app: &mut App) where B: Backend {
    let tweet = &app.tweets[app.current_tweet];
    let medias = match &tweet.entities.media {
        Some(medias) => medias,
        None => return
    };

    if medias.is_empty() {
        return;
    }

    if app.image.is_empty() {
        app.image = fetch_image(&medias[0].media_url_https)
    };

    let img = &app.image;
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .horizontal_margin(30)
        .vertical_margin(8)
        .split(area);

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::NONE))
        .x_bounds([0.0, 450.0])
        .y_bounds([0.0, 300.0])
        .paint(|ctx| {
            for color in img.keys() {
                if let Some(points) = img.get(color) {
                    ctx.draw(&Points {
                        coords: points,
                        color: Color::Rgb(color[0], color[1], color[2]),
                    })
                }
            }
        });

    f.render_widget(canvas, chunks[0]);
}

fn render_commands<B>(f: &mut Frame<B>, area: Rect, app: &mut App) where B: Backend {
    let tweet = &app.tweets[app.current_tweet];
    let mut commands = vec![
        Span::styled(" q ", Style::default().add_modifier(Modifier::REVERSED)),
        Span::raw(" quit "),
        Span::styled(" < ", Style::default().add_modifier(Modifier::REVERSED)),
        Span::raw(" previous tweet "),
        Span::styled(" > ", Style::default().add_modifier(Modifier::REVERSED)),
        Span::raw(" next tweet "),
        Span::styled(" r ", Style::default().add_modifier(Modifier::REVERSED)),
        Span::raw(" reload timeline "),
        Span::styled(" o ", Style::default().add_modifier(Modifier::REVERSED)),
        Span::raw(" open in browser "),
    ];

    if let Some(medias) = &tweet.entities.media {
        if !medias.is_empty() {
            commands.push(Span::styled(" v ", Style::default().add_modifier(Modifier::REVERSED)));
            commands.push(Span::raw(" view image "));
        }
    }

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Commands",
        Style::default().add_modifier(Modifier::BOLD),
    ));

    let text = Spans::from(commands);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn fetch_image(url: &String) -> HashMap<[u8; 3], Vec<(f64, f64)>> {
    let image_url = format!("{}:small", url);
    let img_bytes = reqwest::blocking::get(image_url).unwrap().bytes().unwrap();

    let image = image::load_from_memory(&img_bytes).unwrap()
        .resize_to_fill(450, 300, FilterType::Gaussian)
        .to_rgb8();

    let mut result = HashMap::<[u8; 3], Vec<(f64, f64)>>::new();
    let (_, height) = image.dimensions();
    let height = height as i32;
    for (x, y, color) in image.enumerate_pixels() {
        let x = f64::from(x);
        let y = f64::from(height - 1 - (y as i32));

        let key = color.0;
        let value = (x, y);

        if let Some(coordinates) = result.get_mut(&key) {
            coordinates.push(value);
        } else {
            result.insert(key, vec![value]);
        }
    }

    result
}