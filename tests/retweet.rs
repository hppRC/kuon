use anyhow::Result;

#[tokio::test]
async fn retweet() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api
        .retweet()
        .id(1367367057055109125u64)
        .trim_user(true)
        .send()
        .await;

    match res {
        Ok(v) => assert!(v.text.len() > 0),
        Err(kuon::Error::TwitterAPIError(e, _)) => assert!(e.errors.len() > 0),
        _ => panic!("panic with favorite test"),
    }
    let res = api
        .retweet()
        .id(1367367057055109125u64)
        .trim_user(false)
        .send()
        .await;

    match res {
        Ok(v) => assert!(v.text.len() > 0),
        Err(kuon::Error::TwitterAPIError(e, _)) => assert!(e.errors.len() > 0),
        _ => panic!("panic with favorite test"),
    }

    Ok(())
}
