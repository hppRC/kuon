use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api
        .user_timeline()
        .screen_name("rustlang")
        .count(10)
        .send()
        .await?;

    for tweet in res {
        println!("{}", tweet.text);
    }

    let res = api.home_timeline().count(10).send().await?;

    for tweet in res {
        println!("{}", tweet.text);
        // show original quality images' url
        if let Some(extended_entities) = tweet.extended_entities {
            for media in extended_entities.media {
                println!("{}:orig", media.media_url_https)
            }
        }
    }
    Ok(())
}
