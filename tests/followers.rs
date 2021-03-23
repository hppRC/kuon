use anyhow::Result;

#[tokio::test]
async fn followers_ids() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api
        .followers_ids()
        .screen_name("rustlang")
        .cursor(-1)
        .count(100)
        .send()
        .await;

    match res {
        Ok(v) => assert!(v.ids.len() >= 1),
        Err(kuon::Error::TwitterAPIError(e, _)) => assert!(e.errors.len() >= 1),
        _ => panic!("panic with favorite test"),
    }

    Ok(())
}
