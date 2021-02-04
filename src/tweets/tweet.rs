use crate::{TweetResult, TwitterAPI};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    /// # Example
    /// ```
    /// # use anyhow::Result;
    /// # async fn doc() -> Result<()> {
    ///
    /// let api = kuon::TwitterAPI::new_using_env().await?;
    /// let res = api.tweet("これはてすとなんだなも").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tweet(&self, status: &str) -> Result<TweetResult> {
        let endpoint = "https://api.twitter.com/1.1/statuses/update.json";
        let params = maplit::hashmap! { "status" => status };

        self.raw_post(endpoint, &params).await
    }

    pub async fn tweet_with_params(
        &self,
        status: &str,
        params: &HashMap<&str, &str>,
    ) -> Result<TweetResult> {
        let endpoint = "https://api.twitter.com/1.1/statuses/update.json";
        let mut params = params.clone();
        params.insert("status", status);

        self.raw_post(endpoint, &params).await
    }
}
