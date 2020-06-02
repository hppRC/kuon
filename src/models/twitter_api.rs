use crate::client_builder::ClientBuilder;
use crate::models::bearer::BearerToken;

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
    pub fn builder() -> ClientBuilder<(), (), (), ()> {
        ClientBuilder::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test() -> Result<()> {
        Ok(())
    }
}
