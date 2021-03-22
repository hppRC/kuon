use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct Tweet<'a, Status> {
    api: &'a TwitterAPI,

    status: Status,

    in_reply_to_status_id: Option<u64>,
    auto_populate_reply_metadata: Option<bool>,
    exclude_reply_user_ids: Option<Vec<u64>>,
    attachment_url: Option<String>,
    media_ids: Option<Vec<u64>>,
    possibly_sensitive: Option<bool>,
    lat: Option<f64>,
    long: Option<f64>,
    place_id: Option<String>,
    display_coordinates: Option<bool>,
    trim_user: Option<bool>,
    enable_dmcommands: Option<bool>,
    fail_dmcommands: Option<bool>,
    card_uri: Option<String>,
}

impl<'a, Status> Tweet<'a, Status>
where
    Status: ToString,
{
    pub async fn send(&self) -> Result<TrimTweet, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/update.json";
        let params = self.to_hashmap();
        self.api.raw_post(endpoint, &params).await
    }
}
