use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        &std::env::var("API_KEY").with_context(|| "Could not find API_KEY in your environment")?;
    let api_secret_key = &std::env::var("API_SECRET_KEY")
        .with_context(|| "Could not find API_SECRET_KEY in your environment")?;

    let builder = kuon::TwitterAPI::builder()
        .api_key(api_key)
        .api_secret_key(api_secret_key);
    let request_token = builder.pre_build(kuon::Callback::Pin).await?;

    println!(
        "Please access https://api.twitter.com/oauth/authorize?oauth_token={}",
        request_token.token
    );

    println!("---Input pin code---");
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    let pin = word.trim().to_string();

    let api = builder.build(request_token, &pin).await?;
    println!("{:?}", api);

    let result = api.search_tweets().q("rust").send().await?;

    for tweet in result.statuses {
        let user = tweet.user;
        println!("{} {} {}", user.screen_name, user.name, tweet.text);
    }

    Ok(())
}
