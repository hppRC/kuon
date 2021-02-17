use std::collections::HashMap;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let url = "https://api.twitter.com/1.1/statuses/user_timeline.json";
    let res: Vec<kuon::Tweet> = api.raw_get(url, &HashMap::new(), None).await?;

    for tweet in res {
        println!("{:?}", tweet);
    }

    Ok(())
}
