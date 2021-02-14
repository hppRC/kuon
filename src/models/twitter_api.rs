use crate::models::bearer::BearerToken;
use crate::{client_builder::ClientBuilder, OAuthToken};
use anyhow::Context as _;
use anyhow::Result;

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
    /// Creates [`ClientBuilder`] that helps construct a configured `TwitterAPI`.
    ///
    /// This is exactly equivalent to [`ClientBuilder::new`].
    ///
    /// [`ClientBuilder`]: struct.ClientBuilder.html
    /// [`ClientBuilder::new`]: struct.ClientBuilder.html#method.new
    pub fn builder() -> ClientBuilder<(), (), (), ()> {
        ClientBuilder::new()
    }

    pub async fn new_using_env() -> Result<Self> {
        let access_token = &std::env::var("ACCESS_TOKEN")
            .with_context(|| "Could not find ACCESS_TOKEN in your environment")?;
        let access_token_secret = &std::env::var("ACCESS_TOKEN_SECRET")
            .with_context(|| "Could not find ACCESS_TOKEN_SECRET in your environment")?;
        let api_key = &std::env::var("API_KEY")
            .with_context(|| "Could not find API_KEY in your environment")?;
        let api_secret_key = &std::env::var("API_SECRET_KEY")
            .with_context(|| "Could not find API_SECRET_KEY in your environment")?;

        Self::builder()
            .api_key(api_key)
            .api_secret_key(api_secret_key)
            .access_token(access_token)
            .access_token_secret(access_token_secret)
            .build()
            .await
    }

    pub fn oauth_token(&self) -> OAuthToken {
        OAuthToken {
            token: self.access_token.clone(),
            secret: self.access_token_secret.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test() -> Result<()> {
        // let api = crate::TwitterAPI::new_using_env().await?;

        Ok(())
    }
}
