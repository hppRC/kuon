use anyhow::Result;

#[tokio::test]
async fn favorite() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;

    let res: Result<kuon::Tweet, kuon::Error> =
        api.favorite().id(0u64).include_entities(true).send().await;
    match res {
        Ok(v) => assert!(v.user.screen_name.len() >= 1),
        Err(kuon::Error::TwitterAPIError(e, _)) => assert!(e.errors.len() >= 1),
        _ => panic!("panic with favorite test"),
    }

    Ok(())
}
