use anyhow::Result;
use maplit::hashmap;

#[tokio::test]
async fn followers_ids() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"screen_name" => "rustlang"};
    let res: kuon::FollowersIdsResult = api.follwers_ids(&params).await?;

    assert!(res.ids.len() > 0);

    Ok(())
}

#[tokio::test]
async fn followers_list() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"screen_name" => "rustlang"};
    let res: kuon::FollowersListResult = api.follwers_list(&params).await?;

    assert!(res.users.len() > 0);

    Ok(())
}
