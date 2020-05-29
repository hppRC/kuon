use serde_derive::*;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub search_metadata: Value,
    pub statuses: Vec<Value>,
}
