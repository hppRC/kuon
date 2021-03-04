use anyhow::Result;
use maplit::hashmap;

#[tokio::test]
async fn assert_length_of_timelines() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"count" => "10"};

    let res = api.home_timeline(&params).await?;
    assert!(res.len() > 0);
    let res = api.user_timeline(&params).await?;
    assert!(res.len() > 0);
    let res = api.mentions_timeline(&params).await?;
    assert!(res.len() > 0);

    Ok(())
}
