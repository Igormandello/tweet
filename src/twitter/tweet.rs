use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Tweet {
    pub id_str: String,
    pub created_at: String,
    pub full_text: String,
    pub user: User,
    pub entities: Entities,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub screen_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Entities {
    pub urls: Vec<URLEntity>,
    pub media: Option<Vec<MediaEntity>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct URLEntity {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MediaEntity {
    pub media_url_https: String,
}
