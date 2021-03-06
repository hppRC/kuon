use anyhow::Result;

#[tokio::test]
async fn retweet() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api.retweet("00000000000000").await;
    assert!(res.is_err());
    if let Err(kuon::Error::TwitterAPIError(err)) = res {
        println!("{:?}", err);
        assert!(err.errors.len() > 0);
    };

    Ok(())
}
