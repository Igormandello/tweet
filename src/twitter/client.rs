use std::{borrow::Cow, collections::HashMap, str};
use std::error::Error;

pub use oauth_client as oauth;
use oauth_client::Result as Result;
use oauth_client::Token;
pub use serde_json;

use crate::event::Config;
use crate::twitter::Tweet;

pub const TIMELINE_URL: &str = "https://api.twitter.com/1.1/statuses/home_timeline.json";

pub struct TwitterClient {}

impl TwitterClient {
    pub fn fetch_timeline() -> Vec<Tweet> {
        let (api_key, token) = TwitterClient::generate_credentials();
        let mut param = HashMap::new();
        param.insert("count".into(), "20".into());
        param.insert("exclude_replies".into(), "true".into());
        param.insert("tweet_mode".into(), "extended".into());

        let bytes = oauth_client::get(TIMELINE_URL, &api_key, Some(&token), Some(&param)).unwrap();
        let response_body = String::from_utf8(bytes).unwrap();
        serde_json::from_str::<Vec<Tweet>>(&response_body).unwrap()
    }

    fn generate_credentials() -> (Token<'static>, Token<'static>) {
        let key = oauth_client::Token::new("", "");
        let token = oauth_client::Token::new("", "");
        (key, token)
    }
}