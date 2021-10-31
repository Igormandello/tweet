use std::cmp::min;
use std::collections::HashMap;

use webbrowser;

use crate::twitter::{Tweet, TwitterClient};

pub struct App {
    pub running: bool,
    pub tweets: Vec<Tweet>,
    pub current_tweet: usize,
    pub showing_image: bool,
    pub image: HashMap<[u8; 3], Vec<(f64, f64)>>,
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            tweets: TwitterClient::fetch_timeline(),
            current_tweet: 0,
            showing_image: false,
            image: HashMap::new(),
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => self.running = false,
            ',' | '<' => self.previous_tweet(),
            '.' | '>' => self.next_tweet(),
            'r' => self.load_tweets(),
            'o' => self.open_in_browser(),
            'v' => self.showing_image = !self.showing_image,
            _ => {}
        }
    }

    pub fn previous_tweet(&mut self) {
        self.reset_image();
        self.current_tweet = if self.current_tweet == 0 { 0 } else { self.current_tweet - 1 };
    }

    pub fn next_tweet(&mut self) {
        self.reset_image();
        self.current_tweet = min(self.current_tweet + 1, self.tweets.len() - 1);
    }

    pub fn on_tick(&mut self) {}

    fn load_tweets(&mut self) {
        self.reset_image();
        self.current_tweet = 0;
        self.tweets = TwitterClient::fetch_timeline();
    }

    fn open_in_browser(&self) {
        let tweet_id = &self.tweets[self.current_tweet].id_str;
        let url = format!("https://twitter.com/i/web/status/{}", tweet_id);
        let _ = webbrowser::open(&url);
    }

    fn reset_image(&mut self) {
        self.showing_image = false;
        self.image = HashMap::new();
    }
}