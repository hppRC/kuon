use crate::models::*;
use serde_derive::*;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    pub created_at: Option<String>,
    pub id: u64,
    pub id_str: Option<String>,
    pub text: String,
    pub truncated: bool,
    pub entities: Box<TweetEntities>,
    pub metadata: Option<TweetMetadata>,
    pub source: Option<String>,
    pub user: User,

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
    pub retweeted_status: Option<Box<Tweet>>,
    pub possibly_sensitive: Option<bool>,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TweetMetadata {
    iso_language_code: Option<String>,
    result_type: Option<String>,
}
