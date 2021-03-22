use std::fmt::Display;

use crate::{models::bearer::BearerToken, OAuthRequestToken};
use crate::{OAuthToken, TwitterAPI};
use anyhow::{Context, Result};
use chrono::Utc;
use maplit::hashmap;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, Method,
};

/// A `ClientBuilder` can help construct a [`TwitterAPI`] instance with your configuration.
/// Before calling [`build`] method, you must set four values:
///
/// 1. `Access token`
/// 2. `Access token secret`
/// 3. `API key`
/// 4. `API secret key`
///
/// The four setter methods can be called with any order.
///
/// # Example
///
/// ```rust
/// # async fn doc() -> Result<(), anyhow::Error> {
/// use kuon::ClientBuilder;
/// let builder = ClientBuilder::new();
///
/// // The order of setter methods can be changed.
/// let api_client = builder
///     .access_token("YOUR_ACCESS_TOKEN")
///     .access_token_secret("YOUR_ACCESS_TOKEN_SECRET")
///     .api_key("YOUR_API_KEY")
///     .api_secret_key("YOUR_API_SECRET_KEY")
///     .build() // This can be called only after all values have been set.
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// [`TwitterAPI`]: struct.TwitterAPI.html
/// [`build`]: struct.ClientBuilder.html#method.build
#[derive(Debug, Default)]
pub struct ClientBuilder<AccessTokenType, AccessTokenSecretType, ApiKeyType, ApiKeySecretType> {
    access_token: AccessTokenType,
    access_token_secret: AccessTokenSecretType,
    api_key: ApiKeyType,
    api_secret_key: ApiKeySecretType,
}

pub enum Callback {
    Pin,
    Url(String),
}

impl Display for Callback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Pin => String::from("oob"),
            Self::Url(url) => url.to_string(),
        };
        write!(f, "{}", string)
    }
}

impl ClientBuilder<(), (), (), ()> {
    /// Creates a builder instance.
    ///
    /// This is exactly equivalent to [`TwitterAPI::builder`].
    ///
    /// [`TwitterAPI::builder`]: struct.TwitterAPI.html#method.builder
    pub fn new() -> Self {
        ClientBuilder {
            access_token: (),
            access_token_secret: (),
            api_key: (),
            api_secret_key: (),
        }
    }
}

impl ClientBuilder<(), (), String, String> {
    pub async fn pre_build(&self, callback: Callback) -> Result<OAuthRequestToken> {
        let client = reqwest::Client::new();

        let api = TwitterAPI {
            access_token: String::new(),
            access_token_secret: String::new(),
            api_key: self.api_key.clone(),
            api_secret_key: self.api_secret_key.clone(),
            bearer: BearerToken {
                access_token: String::new(),
                token_type: String::new(),
            },
            client,
        };

        Self::request_oauth_token(&api, callback).await
    }

    pub async fn build(
        self,
        request_token: OAuthRequestToken,
        verifier: &str,
    ) -> Result<TwitterAPI> {
        let client = reqwest::Client::new();
        let bearer = self.get_bearer(&client).await?;

        let OAuthToken { token, secret } =
            Self::request_token(&client, request_token, verifier).await?;

        Ok(TwitterAPI {
            access_token: token,
            access_token_secret: secret,
            api_key: self.api_key,
            api_secret_key: self.api_secret_key,
            bearer,
            client,
        })
    }

    async fn request_oauth_token(
        api: &TwitterAPI,
        callback: Callback,
    ) -> Result<OAuthRequestToken> {
        let endpoint = "https://api.twitter.com/oauth/request_token";
        let oauth_nonce = format!("nonce{}", Utc::now().timestamp());
        let oauth_signature_method = "HMAC-SHA1".to_string();
        let oauth_timestamp = format!("{}", Utc::now().timestamp());
        let oauth_version = "1.0".to_string();
        let oauth_callback = callback.to_string();

        let params = hashmap! {
            "oauth_nonce" => oauth_nonce,
            "oauth_callback" => oauth_callback,
            "oauth_version" => oauth_version,
            "oauth_timestamp" => oauth_timestamp,
            "oauth_consumer_key" => api.api_key.clone(),
            "oauth_signature_method" => oauth_signature_method,
        };

        let res: String = api.request(endpoint, Method::POST, &params).await?;

        OAuthRequestToken::from(&res).with_context(|| "Failed parse")
    }

    async fn request_token(
        client: &Client,
        request_token: OAuthRequestToken,
        verifier: &str,
    ) -> Result<OAuthToken> {
        let url = "https://api.twitter.com/oauth/access_token";
        let params = hashmap! {
            "oauth_token" => request_token.token,
            "oauth_verifier" => String::from(verifier)
        };

        let text = client.post(url).query(&params).send().await?.text().await?;

        OAuthToken::from(&text).with_context(|| "Failed parse")
    }
}

impl ClientBuilder<String, String, String, String> {
    /// Builds a [`TwitterAPI`] instance with the values you've set.
    ///
    /// You can call this method only after the four required values have been set.
    ///
    /// # Error
    ///
    /// This method fails if there is an error when obtaining a bearer token.
    ///
    /// [`TwitterAPI`]: struct.TwitterAPI.html
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
}

impl<AccessTokenType, AccessTokenSecretType>
    ClientBuilder<AccessTokenType, AccessTokenSecretType, String, String>
{
    async fn get_bearer(&self, client: &reqwest::Client) -> Result<BearerToken> {
        let endpoint = "https://api.twitter.com/oauth2/token";
        let headers = self.setup_header()?;
        let bearer: BearerToken = Self::request_oauth(client, endpoint, headers).await?;
        Ok(bearer)
    }

    async fn request_oauth(
        client: &reqwest::Client,
        endpoint: impl reqwest::IntoUrl,
        header: HeaderMap<HeaderValue>,
    ) -> Result<BearerToken> {
        // TODO: #8 better error handling
        let res = client
            .post(endpoint)
            .body("grant_type=client_credentials")
            .headers(header)
            .send()
            .await?
            .text()
            .await?;
        let bearer: BearerToken = serde_json::from_str(&res)?;
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
    /// Sets the access token.
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

    /// Sets the access token secret.
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

    /// Sets the api key.
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

    /// Sets the api secret key.
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
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn builder_method_chain() {
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

    #[test]
    fn setup_header() {
        let builder = ClientBuilder::new()
            .api_key("foo")
            .access_token_secret("bar")
            .api_secret_key("baz")
            .access_token("qux");

        let expected = {
            let mut h = HeaderMap::new();
            h.insert(
                AUTHORIZATION,
                HeaderValue::from_static("Basic Zm9vOmJheg=="),
            );
            h.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
            h
        };

        assert_eq!(builder.setup_header().unwrap(), expected);
    }

    #[tokio::test]
    async fn request_oauth() {
        // arrange the behavior of the mock server
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "foo",
                "token_type": "bar",
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        // prepare for calling `ClientBuilder::request_oauth`
        let client = reqwest::Client::new();
        let uri = mock_server.uri();

        let res =
            ClientBuilder::<(), (), String, String>::request_oauth(&client, &uri, HeaderMap::new())
                .await
                .unwrap();

        assert_eq!(
            res,
            BearerToken {
                access_token: "foo".to_string(),
                token_type: "bar".to_string(),
            }
        );
    }
}
