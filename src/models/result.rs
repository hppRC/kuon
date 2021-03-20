use crate::models::*;
use serde_derive::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub search_metadata: SearchMetadata,
    pub statuses: Vec<Tweet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PremiumSearchResult {
    pub results: Vec<Tweet>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FavoriteResult {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RetweetResult {
    pub user: User,
    pub retweeted_status: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TweetResult {
    pub created_at: Value,
    pub id: u64,
    pub text: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchMetadata {
    pub completed_in: f64,
    pub count: u64,
    // I'm afraid of overflows...
    pub max_id: u64,
    pub max_id_str: String,
    pub next_results: Option<String>,
    pub query: String,
    pub refresh_url: Option<String>,
    pub since_id: u64,
    pub since_id_str: String,
}
