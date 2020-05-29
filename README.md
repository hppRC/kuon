# Kuon

Twitter Client Library written in Rust.

inspired by [anaconda](https://github.com/ChimeraCoder/anaconda)

## Example

### Search Tweets

```rust
use kuon::TwitterClient;
use std::collections::HashMap;

async fn main() -> Result<()> {
    let api = TwitterClient::new("api_key", "api_secret_key", "access_token", "access_token_secret").await?;

    let mut params = HashMap::new();
    params.insert("q", "rust");
    params.insert("count", "3");

    let results: SearchResult = api.search_tweets(&params).await?;

    println!("{:?}", results.statuses);
}
```