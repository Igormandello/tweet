use std::cmp::{max, min};
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
            ',' | '<' => self.previous_tweet(),
            '.' | '>' => self.next_tweet(),
            _ => {}
        }
    }

    pub fn previous_tweet(&mut self) {
        self.current_tweet = if self.current_tweet == 0 { 0 } else { self.current_tweet - 1 };
    }

    pub fn next_tweet(&mut self) {
        self.current_tweet = min(self.current_tweet + 1, self.tweets.len() - 1);
    }

    pub fn on_tick(&mut self) {}
}