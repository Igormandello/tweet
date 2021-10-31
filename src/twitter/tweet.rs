use std::error::Error;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Tweet {
    pub id_str: String,
    pub created_at: String,
    pub full_text: String,
    pub user: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub screen_name: String,
}

impl Tweet {
    pub fn parse_timeline(json_str: impl AsRef<str>) -> Result<Vec<Tweet>, Box<dyn Error>> {
        let tweets = serde_json::from_str(json_str.as_ref())?;
        Ok(tweets)
    }
}
