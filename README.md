# Kuon

Twitter Client Library written in Rust.

inspired by [anaconda](https://github.com/ChimeraCoder/anaconda)

## Example

### Search Tweets

```rust
let api =
    kuon::TwitterClient::new("api_key", "api_secret_key", "access_token", "access_token_secret").await?;

let mut params = HashMap::new();
params.insert("q", "rust");
params.insert("count", "3");

let result = api.search_tweets(&params).await?;
println!("{:?}", result.statuses);
```
