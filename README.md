# Kuon

Twitter Client Library written in Rust.

inspired by [anaconda](https://github.com/ChimeraCoder/anaconda)

## Example

### Search Tweets

```rust
let api =
    kuon::TwitterAPI::new("api_key", "api_secret_key", "access_token", "access_token_secret").await?;

let res: kuon::SearchResult = api.search_tweets("rust").await?;
```

This crate is named after Japanese Virtual YouTuber [Chitose Kudo](https://www.youtube.com/channel/UCP2o-o6u4uX3uq1hXspl0rg)