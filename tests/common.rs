use anyhow::Result;

pub async fn get_api_client() -> Result<kuon::TwitterAPI> {
    let access_token = &std::env::var("ACCESS_TOKEN")?;
    let access_token_secret = &std::env::var("ACCESS_TOKEN_SECRET")?;
    let api_key = &std::env::var("API_KEY")?;
    let api_secret_key = &std::env::var("API_SECRET_KEY")?;

    kuon::TwitterAPI::new(api_key, api_secret_key, access_token, access_token_secret).await
}