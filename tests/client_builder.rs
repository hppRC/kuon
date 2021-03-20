use anyhow::Result;

#[tokio::test]
async fn client_builder() -> Result<()> {
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

    let res: kuon::SearchResult = api.search_tweets().q("rust").send().await?;
    assert_eq!(res.search_metadata.query, "rust");

    Ok(())
}

#[tokio::test]
async fn client_builder_for_oauth() -> Result<()> {
    let api_key = &std::env::var("API_KEY").unwrap();
    let api_secret_key = &std::env::var("API_SECRET_KEY").unwrap();

    let builder = kuon::TwitterAPI::builder()
        .api_key(api_key)
        .api_secret_key(api_secret_key);

    let url = builder.pre_build(kuon::Callback::PIN).await?;
    println!("{:?}", url);

    Ok(())
}
