use anyhow::Result;
use maplit::hashmap;

#[tokio::main]
async fn main() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"count" => "100"};
    let user_name = "asahi";
    let res: kuon::SearchResult = api
        .search_tweets_with_params(&format!("@{} -rt", user_name), &params)
        .await?;

    for tweet in res.statuses {
        if let Some(v) = tweet.in_reply_to_screen_name {
            if v == user_name {
                println!("{} {}", tweet.user.screen_name, tweet.user.id_str);
            }
        }
    }

    Ok(())
}
