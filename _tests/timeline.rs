use anyhow::Result;
use maplit::hashmap;

#[tokio::test]
async fn assert_length_of_timelines() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"count" => "10"};

    assert!(api.home_timeline(&params).await.is_ok());
    assert!(api.user_timeline(&params).await.is_ok());
    assert!(api.mentions_timeline(&params).await.is_ok());

    Ok(())
}
