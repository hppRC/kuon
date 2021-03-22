use anyhow::Result;

#[tokio::test]
async fn tweet() -> Result<()> {
    let api: kuon::TwitterAPI = kuon::TwitterAPI::new_using_env().await?;
    let ids = vec![0, 1];
    let res = api
        .tweet()
        .status("status")
        .in_reply_to_status_id(0)
        .auto_populate_reply_metadata(true)
        .exclude_reply_user_ids(ids)
        .attachment_url("https://example.com")
        .media_ids(vec![0, 1])
        .possibly_sensitive(true)
        .lat(0.0)
        .long(0.0)
        .place_id("place_id")
        .display_coordinates(true)
        .trim_user(true)
        .enable_dmcommands(true)
        .fail_dmcommands(true)
        .card_uri("uri")
        .send()
        .await;

    match res {
        Ok(tweet) => assert!(tweet.id > 0),
        Err(kuon::Error::TwitterAPIError(e, _)) => {
            assert!(e.errors.len() > 0)
        }
        _ => panic!("Unexpected error!"),
    }

    Ok(())
}
