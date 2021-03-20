use anyhow::Result;

#[tokio::test]
async fn serach_tweets() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;

    let res = api
        .search_tweets()
        .q("rust")
        .geocode("geocode")
        .lang("lang")
        .locale("locale")
        .result_type("result_type")
        .count(100)
        .until("2000-01-01")
        .since_id(0)
        .max_id(0)
        .include_entities(true)
        .send()
        .await;

    match res {
        Ok(search_result) => assert!(search_result.statuses.len() == 0),
        Err(kuon::Error::TwitterAPIError(e, _)) => {
            assert!(e.errors.len() > 0)
        }
        _ => panic!("Unexpected error!"),
    }

    let res = api.search_tweets().q("rust").count(100).send().await?;
    assert!(res.statuses.len() > 0);

    Ok(())
}
