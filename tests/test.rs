use anyhow::Result;

// #[tokio::test]
async fn test() -> Result<()> {
    let access_token = &std::env::var("ACCESS_TOKEN").unwrap();
    let access_token_secret = &std::env::var("ACCESS_TOKEN_SECRET").unwrap();
    let api_key = &std::env::var("API_KEY").unwrap();
    let api_secret_key = &std::env::var("API_SECRET_KEY").unwrap();

    let api: kuon::TwitterAPI =
        kuon::TwitterAPI::new(api_key, api_secret_key, access_token, access_token_secret).await?;

    let res: kuon::SearchResult = api.search_tweets("rust").await?;
    assert_eq!(res.search_metadata, "");

    let res: kuon::FavoriteResult = api.favorite("1265335147550760962").await?;
    assert_eq!(res.user, "");

    let res: kuon::RetweetResult = api.retweet("1266570131901018112").await?;
    assert_eq!(res.retweeted_status, "");

    Ok(())
}
