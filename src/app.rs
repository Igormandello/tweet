use std::iter::Cycle;
use crate::twitter::{Tweet, TwitterClient};

pub struct App {
    pub running: bool,
    pub tweets: Vec<Tweet>,
    pub current_tweet: usize,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            tweets: TwitterClient::fetch_timeline(),
            current_tweet: 0,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => self.running = false,
            'p' => self.current_tweet += 1,
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}