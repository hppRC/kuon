use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api
        .followers_ids()
        .screen_name("rustlang")
        .count(100)
        .send()
        .await?;

    println!("{:?}", res.ids);

    Ok(())
}
