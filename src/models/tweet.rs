use serde_derive::*;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub id: u64,
    pub id_str: String,
    pub text: String,
    pub truncated: bool,
    pub entities: Box<TweetEntities>,
    pub metadata: TweetMetadata,
    pub source: String,
    pub user: Value, //TODO: implement correct type

    pub in_reply_to_status_id: Option<u64>,
    pub in_reply_to_status_id_str: Option<String>,
    pub in_reply_to_user_id: Option<u64>,
    pub in_reply_to_user_id_str: Option<String>,
    pub in_reply_to_screen_name: Option<String>,

    pub geo: Option<Value>,          //TODO: implement correct type
    pub coordinates: Option<Value>,  //TODO: implement correct type
    pub place: Option<Value>,        //TODO: implement correct type
    pub contributors: Option<Value>, //TODO: implement correct type
    pub is_quote_status: bool,
    pub quoted_status_id: Option<u64>,
    pub quoted_status_id_str: Option<String>,

    pub quoted_status: Option<Value>, //TODO: implement correct type

    pub retweet_count: u64,
    pub favorite_count: u64,
    pub favorited: bool,
    pub retweeted: bool,
    pub possibly_sensitive: Option<bool>,
    pub lang: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TweetEntities {
    hashtags: Value,
    symbols: Value,
    user_mentions: Value,
    urls: Vec<EntityUrls>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EntityUrls {
    url: String,
    expanded_url: String,
    display_url: String,
    indices: Vec<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TweetMetadata {
    iso_language_code: String,
    result_type: String,
}
