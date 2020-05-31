use serde_derive::*;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub search_metadata: Value,
    pub statuses: Vec<Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FavoriteResult {
    pub user: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RetweetResult {
    pub user: Value,
    pub retweeted_status: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TweetResult {
    pub created_at: Value,
    pub text: Value,
}
