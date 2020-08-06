use anyhow::Result;
use maplit::hashmap;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"screen_name" => "rustlang"};
    let res: kuon::FollowersIdsResult = api.follwers_ids(&params).await?;

    println!("{}", res.ids.len());

    Ok(())
}
