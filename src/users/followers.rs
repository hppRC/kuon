use crate::{FollowersIdsResult, FollowersListResult, TwitterAPI};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    pub async fn follwers_ids(&self, params: &HashMap<&str, &str>) -> Result<FollowersIdsResult> {
        let endpoint = "https://api.twitter.com/1.1/followers/ids.json";
        self.raw_get(endpoint, &params, None).await
    }

    pub async fn follwers_list(&self, params: &HashMap<&str, &str>) -> Result<FollowersListResult> {
        let endpoint = "https://api.twitter.com/1.1/followers/list.json";
        self.raw_get(endpoint, &params, None).await
    }
}
