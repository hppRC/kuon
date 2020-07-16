use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res: kuon::TweetResult = api.tweet("example tweet").await?;

    println!("{}", serde_json::to_string(&res)?);

    Ok(())
}
