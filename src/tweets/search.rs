use crate::{SearchResult, TwitterAPI};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    pub async fn search_tweets(&self, query: &str) -> Result<SearchResult> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let params = maplit::hashmap! { "q" => query };

        self.get(endpoint, &params).await
    }

    pub async fn search_tweets_with_params(
        &self,
        query: &str,
        params: &HashMap<&str, &str>,
    ) -> Result<SearchResult> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let mut params = params.clone();
        params.insert("q", query);

        self.get(endpoint, &params).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() -> Result<()> {
        Ok(())
    }
}
