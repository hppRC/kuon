use crate::models::bearer::BearerToken;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

#[derive(Debug, Clone)]
// TODO: #3 Thinking about naming, it might be better to use TwitterAPI
pub struct TwitterAPI {
    pub(crate) access_token: String,
    pub(crate) access_token_secret: String,
    pub(crate) api_key: String,
    pub(crate) api_secret_key: String,
    pub(crate) bearer: BearerToken,
    pub(crate) client: reqwest::Client,
}

impl TwitterAPI {
    pub async fn new(
        api_key: &str,
        api_secret_key: &str,
        access_token: &str,
        access_token_secret: &str,
    ) -> Result<Self> {
        let client = reqwest::Client::new();
        let bearer: BearerToken = Self::get_bearer(api_key, api_secret_key, &client).await?;
        Ok(Self {
            access_token: access_token.into(),
            access_token_secret: access_token_secret.into(),
            api_key: api_key.into(),
            api_secret_key: api_secret_key.into(),
            bearer,
            client,
        })
    }

    pub async fn new_using_env() -> Result<Self> {
        let access_token = &match std::env::var("ACCESS_TOKEN") {
            Ok(v) => v,
            Err(e) => return Err(anyhow::anyhow!("Missing attribute: {}", e)),
        };
        let access_token_secret = &std::env::var("ACCESS_TOKEN_SECRET")?;
        let api_key = &std::env::var("API_KEY")?;
        let api_secret_key = &std::env::var("API_SECRET_KEY")?;
        Self::new(api_key, api_secret_key, access_token, access_token_secret).await
    }

    async fn get_bearer(
        api_key: &str,
        api_secret_key: &str,
        client: &reqwest::Client,
    ) -> Result<BearerToken> {
        let endpoint = "https://api.twitter.com/oauth2/token";
        let encoded_keys = base64::encode(&format!("{}:{}", api_key, api_secret_key));
        let header_auth = format!("Basic {}", encoded_keys);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse()?);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        // TODO: #8 better error handling
        let text = client
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
        Ok(())
    }
}
