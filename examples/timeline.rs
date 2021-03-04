use anyhow::Result;
use maplit::hashmap;

#[tokio::main]
async fn main() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let params = hashmap! {"count" => "10"};
    // let res = api.home_timeline(&params).await?;
    // let res = api.user_timeline(&params).await?;
    let res = api.mentions_timeline(&params).await?;

    for tweet in res {
        println!("{}", tweet.text);
    }
    Ok(())
}
