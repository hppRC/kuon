use serde_derive::*;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TweetEntities {
    pub hashtags: Vec<HashTag>,
    pub symbols: Value,
    pub user_mentions: Vec<UserMention>,
    pub media: Option<Vec<Media>>,
    pub urls: Vec<UrlEntities>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlEntities {
    pub url: Option<String>,
    pub expanded_url: Option<String>,
    pub display_url: Option<String>,
    pub indices: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEntities {
    pub url: Option<Url>,
    pub description: Option<Url>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Url {
    pub urls: Vec<UrlEntities>,
}

// Media Object

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    pub display_url: String,
    pub expanded_url: String,
    pub id: u64,
    pub id_str: String,
    pub sizes: Sizes,
    pub url: String,
    pub media_url: String,
    pub media_url_https: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sizes {
    pub thumb: Size,
    pub large: Size,
    pub medium: Size,
    pub small: Size,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Size {
    pub w: u32,
    pub h: u32,
    pub resize: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashTag {
    pub indices: Vec<u64>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMention {
    pub id: u64,
    pub id_str: String,
    pub indices: Vec<u64>,
    pub name: String,
    pub screen_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtendedEntities {
    pub media: Vec<Media>,
}
