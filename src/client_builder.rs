use crate::models::bearer::BearerToken;
use crate::TwitterAPI;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

#[derive(Debug, Default)]
pub struct ClientBuilder<AccessTokenType, AccessTokenSecretType, ApiKeyType, ApiKeySecretType> {
    access_token: AccessTokenType,
    access_token_secret: AccessTokenSecretType,
    api_key: ApiKeyType,
    api_secret_key: ApiKeySecretType,
}

impl ClientBuilder<(), (), (), ()> {
    pub fn new() -> Self {
        ClientBuilder {
            access_token: (),
            access_token_secret: (),
            api_key: (),
            api_secret_key: (),
        }
    }
}

impl ClientBuilder<String, String, String, String> {
    pub async fn build(self) -> Result<TwitterAPI> {
        let client = reqwest::Client::new();
        let bearer = self.get_bearer(&client).await?;

        Ok(TwitterAPI {
            access_token: self.access_token,
            access_token_secret: self.access_token_secret,
            api_key: self.api_key,
            api_secret_key: self.api_secret_key,
            bearer,
            client,
        })
    }

    async fn get_bearer(&self, client: &reqwest::Client) -> Result<BearerToken> {
        let endpoint = "https://api.twitter.com/oauth2/token";
        let headers = self.setup_header()?;

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

    fn setup_header(&self) -> Result<HeaderMap<HeaderValue>> {
        let encoded_keys = base64::encode(&format!("{}:{}", &self.api_key, &self.api_secret_key));
        let header_auth = format!("Basic {}", encoded_keys);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse()?);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        Ok(headers)
    }
}

impl<AccessTokenType, AccessTokenSecretType, ApiKeyType, ApiKeySecretType>
    ClientBuilder<AccessTokenType, AccessTokenSecretType, ApiKeyType, ApiKeySecretType>
{
    pub fn access_token(
        self,
        access_token: impl Into<String>,
    ) -> ClientBuilder<String, AccessTokenSecretType, ApiKeyType, ApiKeySecretType> {
        ClientBuilder {
            access_token: access_token.into(),
            access_token_secret: self.access_token_secret,
            api_key: self.api_key,
            api_secret_key: self.api_secret_key,
        }
    }

    pub fn access_token_secret(
        self,
        access_token_secret: impl Into<String>,
    ) -> ClientBuilder<AccessTokenType, String, ApiKeyType, ApiKeySecretType> {
        ClientBuilder {
            access_token: self.access_token,
            access_token_secret: access_token_secret.into(),
            api_key: self.api_key,
            api_secret_key: self.api_secret_key,
        }
    }

    pub fn api_key(
        self,
        api_key: impl Into<String>,
    ) -> ClientBuilder<AccessTokenType, AccessTokenSecretType, String, ApiKeySecretType> {
        ClientBuilder {
            access_token: self.access_token,
            access_token_secret: self.access_token_secret,
            api_key: api_key.into(),
            api_secret_key: self.api_secret_key,
        }
    }

    pub fn api_secret_key(
        self,
        api_secret_key: impl Into<String>,
    ) -> ClientBuilder<AccessTokenType, AccessTokenSecretType, ApiKeyType, String> {
        ClientBuilder {
            access_token: self.access_token,
            access_token_secret: self.access_token_secret,
            api_key: self.api_key,
            api_secret_key: api_secret_key.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn builder_method_chain() {
        use super::*;
        let builder = ClientBuilder::new()
            .api_key("foo")
            .access_token_secret("bar")
            .api_secret_key("baz")
            .access_token("qux");

        assert_eq!(builder.api_key, "foo");
        assert_eq!(builder.api_secret_key, "baz");
        assert_eq!(builder.access_token, "qux");
        assert_eq!(builder.access_token_secret, "bar");
    }

    // TODO: build test
}
