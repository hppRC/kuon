use anyhow::Result;

#[tokio::test]
async fn favorite() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;

    let res: kuon::SearchResult = api.search_tweets("にじさんじ").await?;
    let tweet: kuon::Tweet = res.statuses[0].clone();
    assert!(tweet.text.len() > 0);

    let res: kuon::FavoriteResult = api.favorite(&tweet.id_str).await?;

    // assert_eq!(res.user, "");
    Ok(())
}
