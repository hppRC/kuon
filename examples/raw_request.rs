use std::collections::HashMap;

use anyhow::Result;
use maplit::hashmap;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let url = "https://api.twitter.com/1.1/account/update_profile.json";
    let res: HashMap<String, String> = api
        .raw_post(url, &hashmap! { "name" => "new_user_name" }, None)
        .await?;

    println!("{:?}", res);
    Ok(())
}
