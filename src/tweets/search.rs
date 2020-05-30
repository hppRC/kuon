use crate::{SearchResult, TwitterAPI};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    pub async fn search_tweets(&self, query: &str) -> Result<SearchResult> {
        let params = maplit::hashmap! { "q" => query };
        let search_results: SearchResult = self
            .get("https://api.twitter.com/1.1/search/tweets.json", &params)
            .await?;
        Ok(search_results)
    }

    pub async fn search_tweets_with_param(
        &self,
        query: &str,
        mut params: HashMap<&str, &str>,
    ) -> Result<SearchResult> {
        params.insert("q", query);
        let search_results: SearchResult = self
            .get("https://api.twitter.com/1.1/search/tweets.json", &params)
            .await?;
        Ok(search_results)
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
