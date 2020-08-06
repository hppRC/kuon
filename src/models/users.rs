use crate::models::*;
use serde_derive::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FollowersIdsResult {
    pub ids: Vec<u64>,
    pub next_cursor: u64,
    pub next_cursor_str: String,
    pub previous_cursor: u64,
    pub previous_cursor_str: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FollowersListResult {
    pub users: Vec<User>,
    pub next_cursor: u64,
    pub next_cursor_str: String,
    pub previous_cursor: u64,
    pub previous_cursor_str: String
}