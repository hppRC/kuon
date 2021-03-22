use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct HomeTimeline<'a> {
    api: &'a TwitterAPI,
    count: Option<u64>,
    since_id: Option<u64>,
    max_id: Option<u64>,
    trim_user: Option<bool>,
    exclude_replies: Option<bool>,
    include_entities: Option<bool>,
}

impl<'a> HomeTimeline<'a> {
    pub async fn send(&self) -> Result<Vec<TrimTweet>, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let params = self.to_hashmap();
        self.api.raw_get(endpoint, &params).await
    }
}
