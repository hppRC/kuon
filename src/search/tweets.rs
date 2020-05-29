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
        let access_token = &std::env::var("ACCESS_TOKEN").unwrap();
        let access_token_secret = &std::env::var("ACCESS_TOKEN_SECRET").unwrap();
        let api_key = &std::env::var("API_KEY").unwrap();
        let api_secret_key = &std::env::var("API_SECRET_KEY").unwrap();

        let api =
            TwitterClient::new(api_key, api_secret_key, access_token, access_token_secret).await?;
        let mut params = HashMap::new();
        params.insert("q", "rust");
        params.insert("count", "3");

        let results: SearchResult = api.search_tweets(&params).await?;
        println!("{:?}", results.statuses[0]);
        Ok(())
    }
}
