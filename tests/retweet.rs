use anyhow::Result;

#[tokio::test]
async fn retweet() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api
        .retweet()
        .id(1372893139128619008u64)
        .trim_user(false)
        .send()
        .await?;

    dbg!(res);
    assert!(true);

    // match res {
    //     Ok(v) => {}
    //     Err(kuon::Error::TwitterAPIError(e)) => assert!(e.errors.len() >= 1),
    //     _ => panic!("panic with favorite test"),
    // }

    Ok(())
}
