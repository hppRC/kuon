![kuon](.github/images/kuon.png)

# Kuon

Twitter Client Library written in Rust.

inspired by [anaconda](https://github.com/ChimeraCoder/anaconda)

## Example

```rust
let builder = kuon::TwitterAPI::builder()
    .access_token("access_token")
    .access_token_secret("access_token_secret")
    .api_key("api_key")
    .api_secret_key("api_secret_key");

let api = builder.build().await?;

let res = api.search_tweets("rust").await?;
let res = api.favorite("tweet_id").await?;
let res = api.retweet("tweet_id").await?;
```

## Easy to use

```rust
// Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in your environment
let api = kuon::TwitterAPI::new_using_env().await?;

let res = api.search_tweets("rust").await?;
for tweet in res.statuses {
    println!("{}", tweet.text);
}
```


## Advanced Usage

```rust
let mut params = std::collections::HashMap::new();
params.insert("count", "15");
params.insert("from", "2020-04-01")

let res = api.search_tweets_with_params("rust", params);
```


This crate is named after Japanese Virtual YouTuber [Chitose Kudou](https://www.youtube.com/channel/UCP2o-o6u4uX3uq1hXspl0rg)
