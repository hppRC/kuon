use crate::models::bearer::BearerToken;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_derive::*;

#[derive(Deserialize, Debug, Clone)]
// TODO: #3 Thinking about naming, it might be better to use TwitterAPIClient
pub struct TwitterClient {
    pub(crate) access_token: String,
    pub(crate) access_token_secret: String,
    pub(crate) api_key: String,
    pub(crate) api_secret_key: String,
    pub(crate) bearer: BearerToken,
}

impl TwitterClient {
    pub async fn new(
        api_key: &str,
        api_secret_key: &str,
        access_token: &str,
        access_token_secret: &str,
    ) -> Result<Self> {
        let bearer: BearerToken = Self::get_bearer(api_key, api_secret_key).await?;
        Ok(Self {
            access_token: access_token.into(),
            access_token_secret: access_token_secret.into(),
            api_key: api_key.into(),
            api_secret_key: api_secret_key.into(),
            bearer,
        })
    }

    async fn get_bearer(api_key: &str, api_secret_key: &str) -> Result<BearerToken> {
        let endpoint = "https://api.twitter.com/oauth2/token";
        let encoded_keys = base64::encode(&format!("{}:{}", api_key, api_secret_key));
        let header_auth = format!("Basic {}", encoded_keys);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse()?);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        // TODO: better error handling
        let text = reqwest::Client::new()
            .post(endpoint)
            .body("grant_type=client_credentials")
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let bearer: BearerToken = serde_json::from_str(&text)?;
        Ok(bearer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() -> Result<()> {
        let api_key = &std::env::var("API_KEY").unwrap();
        let api_secret_key = &std::env::var("API_SECRET_KEY").unwrap();

        let bearer: BearerToken = TwitterClient::get_bearer(api_key, api_secret_key).await?;
        dbg!(&bearer);
        assert_eq!(bearer.token_type, "bearer");
        Ok(())
    }
}
