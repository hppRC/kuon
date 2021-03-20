use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api.retweet().id(1368419790553944066u64).send().await?;
    dbg!(res);

    Ok(())
}
