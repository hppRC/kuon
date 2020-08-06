use crate::{TwitterAPI, FollowersIdsResult, FollowersListResult};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    pub async fn follwers_ids(&self, params: &HashMap<&str, &str>) -> Result<FollowersIdsResult> {
        let endpoint = "https://api.twitter.com/1.1/followers/ids.json";
        self.get(endpoint, &params).await
    }

    pub async fn follwers_list(&self, params: &HashMap<&str, &str>) -> Result<FollowersListResult> {
        let endpoint = "https://api.twitter.com/1.1/followers/list.json";
        self.get(endpoint, &params).await
    }
}
