use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let tweet = api.show_tweet().id(1283742285381816320u64).send().await?;

    println!("{:?}", tweet);
    println!("{:?}", tweet.in_reply_to_screen_name);
    println!("{:?}", tweet.in_reply_to_status_id_str);
    println!("{:?}", tweet.in_reply_to_user_id_str);

    Ok(())
}
