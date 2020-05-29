use crate::models::{SearchResult, TwitterClient};
use anyhow::Result;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::collections::HashMap;

impl TwitterClient {
    pub async fn search_tweets(&self, params: &HashMap<&str, &str>) -> Result<SearchResult> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let header_bearer = format!("Bearer {}", self.bearer.access_token);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_bearer.parse()?);

        // TODO: better error handling
        let text = reqwest::Client::new()
            .get(endpoint)
            .query(&params)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let search_results: SearchResult = serde_json::from_str(&text)?;
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
