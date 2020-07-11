use anyhow::Result;

#[tokio::test]
async fn builder() -> Result<()> {
    let access_token = &std::env::var("ACCESS_TOKEN").unwrap();
    let access_token_secret = &std::env::var("ACCESS_TOKEN_SECRET").unwrap();
    let api_key = &std::env::var("API_KEY").unwrap();
    let api_secret_key = &std::env::var("API_SECRET_KEY").unwrap();

    let builder = kuon::TwitterAPI::builder()
        .access_token(access_token)
        .access_token_secret(access_token_secret)
        .api_key(api_key)
        .api_secret_key(api_secret_key);

    let api = builder.build().await?;

    let res: kuon::SearchResult = api.search_tweets("rust").await?;
    assert_eq!(res.search_metadata.query, "rust");

    Ok(())
}
