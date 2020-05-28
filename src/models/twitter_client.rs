use async_trait::async_trait;

// TODO: Thinking about naming, it might be better to use TwitterAPIClient
pub struct TwitterClient {}

// TODO: Devide trait into async and sync
#[async_trait]
pub trait TwitterClientTrait {
    async fn new(access_token: &str, access_token_secret: &str) -> Self;
    async fn new_with_credentials(
        api_key: &str,
        api_key_secret: &str,
        access_token: &str,
        access_token_secret: &str,
    ) -> Self;
}

#[async_trait]
impl TwitterClientTrait for TwitterClient {
    async fn new(access_token: &str, access_token_secret: &str) -> Self {
        todo!()
    }
    async fn new_with_credentials(
        api_key: &str,
        api_key_secret: &str,
        access_token: &str,
        access_token_secret: &str,
    ) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() {}
}
