use anyhow::Result;
use maplit::hashmap;

#[tokio::main]
async fn main() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"count" => "10"};
    // let res = api.home_timeline(&params).await?;
    let res = api.user_timeline(&params).await?;
    // let res = api.mentions_timeline(&params).await?;

    for tweet in res {
        println!("{}", tweet.text);
        // show original quolity images' url
        for media in tweet.extended_entities.unwrap().media {
            println!("{}:orig", media.media_url_https)
        }
    }
    Ok(())
}
