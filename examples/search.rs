use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_from_env().await?;
    let res: kuon::SearchResult = api.search_tweets("rust").await?;

    for tweet in res.statuses {
        println!("{}", tweet.text);
    }

    Ok(())
}