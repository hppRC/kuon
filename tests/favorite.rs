use anyhow::Result;

#[tokio::test]
async fn retweet() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;

    let res: kuon::SearchResult = api.search_tweets("rust").await?;
    let tweet: kuon::Tweet = res.statuses[0].clone();

    let res: kuon::FavoriteResult = api.favorite(&tweet.id_str).await?;

    // assert_eq!(res.user, "");
    Ok(())
}