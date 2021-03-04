use crate::{Tweet, TwitterAPI};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    pub async fn home_timeline(&self, params: &HashMap<&str, &str>) -> Result<Vec<Tweet>> {
        let endpoint = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        self.raw_get(endpoint, &params, None).await
    }

    pub async fn user_timeline(&self, params: &HashMap<&str, &str>) -> Result<Vec<Tweet>> {
        let endpoint = "https://api.twitter.com/1.1/statuses/user_timeline.json";
        self.raw_get(endpoint, &params, None).await
    }

    pub async fn mentions_timeline(&self, params: &HashMap<&str, &str>) -> Result<Vec<Tweet>> {
        let endpoint = "https://api.twitter.com/1.1/statuses/mentions_timeline.json";
        self.raw_get(endpoint, &params, None).await
    }
}
