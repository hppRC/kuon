use crate::TwitterAPI;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

impl TwitterAPI {
    pub(crate) async fn get<T>(&self, endpoint: &str, params: &HashMap<&str, &str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let header_bearer = format!("Bearer {}", self.bearer.access_token);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_bearer.parse()?);

        // TODO: better error handling
        let text = self
            .client
            .get(endpoint)
            .query(&params)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let result = serde_json::from_str(&text)?;
        Ok(result)
    }

    pub(crate) async fn post<T>(&self, endpoint: &str, params: &HashMap<&str, &str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let header_auth = self.create_oauth_header(endpoint, &params);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse()?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let text: String = self
            .client
            .post(endpoint)
            .query(&params)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let result = serde_json::from_str(&text)?;
        Ok(result)
    }
}
