use crate::TwitterAPI;
use anyhow::Result;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Method,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

impl TwitterAPI {
    pub(crate) async fn request(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: &HashMap<&str, &str>,
        query: Option<&HashMap<&str, &str>>,
    ) -> Result<String> {
        let token = self.create_oauth_header(endpoint, &method.to_string(), params, query);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, token.parse()?);

        let text = self
            .client
            .request(method, endpoint)
            .query(&params)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        Ok(text)
    }

    pub async fn raw_get<T>(
        &self,
        endpoint: &str,
        params: &HashMap<&str, &str>,
        query: Option<&HashMap<&str, &str>>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let text = self.request(endpoint, Method::GET, params, query).await?;
        let result = serde_json::from_str(&text)?;
        Ok(result)
    }

    pub async fn raw_post<T>(
        &self,
        endpoint: &str,
        params: &HashMap<&str, &str>,
        query: Option<&HashMap<&str, &str>>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let text = self.request(endpoint, Method::POST, params, query).await?;
        let result = serde_json::from_str(&text)?;
        Ok(result)
    }
}
