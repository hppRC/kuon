use anyhow::Result;

// #[tokio::test]
async fn test() -> Result<()> {
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

    let params = maplit::hashmap! { "count" => "15" };

    let res: kuon::SearchResult = api.search_tweets("rust").await?;
    assert_eq!(res.search_metadata, "");

    let res = api.search_tweets_with_params("rust", &params).await?;
    let res = api.search_tweets_with_params("rust", &params).await?;

    // let res: kuon::FavoriteResult = api.favorite("1265335147550760962").await?;
    // assert_eq!(res.user, "");

    // let res: kuon::RetweetResult = api.retweet("1266570131901018112").await?;
    // assert_eq!(res.retweeted_status, "");

    // let res: kuon::TweetResult = api.tweet("test").await?;
    // assert_eq!(res.text, "te");

    Ok(())
}
