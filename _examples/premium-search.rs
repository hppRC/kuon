use anyhow::Result;
use maplit::hashmap;

#[tokio::main]
async fn main() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"query" => "kuon"};

    let res = api.premium_search_30days(&params, "kuon").await?;

    for tweet in res.results {
        println!("{}", tweet.text)
    }

    Ok(())
}
