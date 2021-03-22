use crate::{Error, FollowersIdsResult, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct FollowersIds<'a> {
    api: &'a TwitterAPI,
    user_id: Option<u64>,
    screen_name: Option<String>,
    cursor: Option<i64>,
    stringify_ids: Option<bool>,
    count: Option<u64>,
}

impl<'a> FollowersIds<'a> {
    pub async fn send(&self) -> Result<FollowersIdsResult, Error> {
        let endpoint = "https://api.twitter.com/1.1/followers/ids.json";
        let params = self.to_hashmap();

        self.api.raw_get(endpoint, &params).await
    }
}
