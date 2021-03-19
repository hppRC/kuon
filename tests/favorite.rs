use anyhow::Result;

#[tokio::test]
async fn favorite() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;

    let res: Result<kuon::FavoriteResult, kuon::Error> = api.favorite().id(0).send().await;
    match res {
        Ok(v) => assert!(v.user.screen_name.len() >= 1),
        Err(kuon::Error::TwitterAPIError(e)) => assert!(e.errors.len() >= 1),
        _ => panic!("panic with favorite test"),
    }

    Ok(())
}
