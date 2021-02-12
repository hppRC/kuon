use crate::{PremiumSearchResult, SearchResult, TwitterAPI};
use anyhow::Result;
use std::collections::HashMap;

impl TwitterAPI {
    pub async fn search_tweets(&self, query: &str) -> Result<SearchResult> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let params = maplit::hashmap! { "q" => query };

        self.raw_get(endpoint, &params, None).await
    }

    pub async fn search_tweets_with_params(
        &self,
        query: &str,
        params: &HashMap<&str, &str>,
    ) -> Result<SearchResult> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let mut params = params.clone();
        params.insert("q", query);

        self.raw_get(endpoint, &params, None).await
    }

    pub async fn premium_search_30days(
        &self,
        params: &HashMap<&str, &str>,
        devenv_name: &str,
    ) -> Result<PremiumSearchResult> {
        let endpoint = &format!(
            "https://api.twitter.com/1.1/tweets/search/30day/{}.json",
            devenv_name
        );

        self.raw_get(endpoint, &params, None).await
    }

    pub async fn premium_search_fullarchive(
        &self,
        params: &HashMap<&str, &str>,
        devenv_name: &str,
    ) -> Result<PremiumSearchResult> {
        let endpoint = &format!(
            "https://api.twitter.com/1.1/tweets/search/fullarchive/{}.json",
            devenv_name
        );

        self.raw_get(endpoint, &params, None).await
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
