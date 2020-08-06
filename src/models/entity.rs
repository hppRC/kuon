use serde_derive::*;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TweetEntities {
    hashtags: Value,
    symbols: Value,
    user_mentions: Value,
    urls: Vec<UrlEntities>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlEntities {
    url: Option<String>,
    expanded_url: Option<String>,
    display_url: Option<String>,
    indices: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEntities {
    url: Option<Url>,
    description: Option<Url>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Url {
    urls: Vec<UrlEntities>,
}
