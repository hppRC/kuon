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

let res = api.search_tweets().q("rust").send().await?;
let res = api.favorite().id(0).send().await?;
let res = api.retweet().id(0).send().await?;
```

### Shord-hand

```rust
// Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in your environment
let api = kuon::TwitterAPI::new_using_env().await?;

let res = api.search_tweets().q("rust").send().await?;
for tweet in res.statuses {
    println!("{}", tweet.text);
}
```


## Advanced Type-safe Usage

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Please set API_KEY, API_SECRET_KEY, ACCESS_TOKEN, ACCESS_TOKEN_SECRET in environment
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let res = api
        .search_tweets()
        .q("rust")
        // .geocode("geocode")
        // .lang("lang")
        // .locale("locale")
        // .result_type("result_type")
        // .count(100)
        // .until("2000-01-01")
        // .since_id(0)
        // .max_id(100000000)
        // .include_entities(true)
        .send()
        .await;

    match res {
        Ok(search_result) => {
            for tweet in search_result.statuses {
                println!("{}", tweet.text);
            }
        }
        Err(kuon::Error::TwitterAPIError(e, param_str)) => {
            // You can confirm a error originated from Twitter API.
            println!("{}", param_str);
            assert!(e.errors.len() > 0)
        }
        Err(kuon::Error::HTTPRequestError(e)) => {
            println!("{}", e);
            // Do something!
        }
        _ => panic!("Unexpected error!"),
    }

    Ok(())
}
```

See more details for `/examples`.


This crate is named after Japanese Virtual YouTuber [Chitose Kudou](https://www.youtube.com/channel/UCP2o-o6u4uX3uq1hXspl0rg)
