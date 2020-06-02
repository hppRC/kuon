# Kuon

Twitter Client Library written in Rust.

inspired by [anaconda](https://github.com/ChimeraCoder/anaconda)

## Example

```rust
let api = kuon::TwitterAPI::new("api_key", "api_secret_key", "access_token", "access_token_secret").await?;

let res = api.search_tweets("rust").await?;
let res = api.favorite("tweet_id").await?;
let res = api.retweet("tweet_id").await?;
```

### Easy to use

Full example.

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_from_env().await?;
    let res: kuon::SearchResult = api.search_tweets("rust").await?;

    for tweet in res.statuses {
        println!("{}", tweet.text);
    }

    Ok(())
}
```


## Advanced Usage

```rust
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("count", "15");
params.insert("from", "2020-04-01")

let res = api.search_tweets_with_params("rust", params);
```


This crate is named after Japanese Virtual YouTuber [Chitose Kudou](https://www.youtube.com/channel/UCP2o-o6u4uX3uq1hXspl0rg)
