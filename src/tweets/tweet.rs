use crate::{TweetResult, TwitterAPI};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    pub async fn tweet(&self, status: &str) -> Result<TweetResult> {
        let endpoint = "https://api.twitter.com/1.1/statuses/update.json";
        let params = maplit::hashmap! { "status" => status };

        let retweet_results: TweetResult = self.post(endpoint, &params).await?;
        Ok(retweet_results)
    }

    pub async fn tweet_with_params(
        &self,
        status: &str,
        params: &HashMap<&str, &str>,
    ) -> Result<TweetResult> {
        let endpoint = "https://api.twitter.com/1.1/statuses/update.json";
        let mut params = params.clone();
        params.insert("status", status);

        let retweet_results: TweetResult = self.post(endpoint, &params).await?;
        Ok(retweet_results)
    }
}
