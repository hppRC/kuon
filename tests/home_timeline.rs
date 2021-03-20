use anyhow::Result;

#[tokio::test]
async fn home_timeline() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api
        .home_timeline()
        .count(100)
        .since_id(0)
        .max_id(0)
        .trim_user(true)
        .exclude_replies(true)
        .include_entities(true)
        .send()
        .await;

    match res {
        Ok(v) => assert!(v.len() == 0),
        Err(kuon::Error::TwitterAPIError(e, _)) => assert!(e.errors.len() >= 1),
        _ => panic!("panic with favorite test"),
    }

    Ok(())
}
