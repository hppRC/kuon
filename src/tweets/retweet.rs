use crate::{RetweetResult, TwitterAPI};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    pub async fn retweet(&self, id: &str) -> Result<RetweetResult> {
        let endpoint = &format!("https://api.twitter.com/1.1/statuses/retweet/{}.json", id);

        let retweet_results: RetweetResult = self.post(endpoint, &HashMap::new()).await?;
        Ok(retweet_results)
    }
}
