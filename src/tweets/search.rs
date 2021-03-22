use crate::{Error, SearchResult, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct SearchTweets<'a, Q> {
    api: &'a TwitterAPI,
    q: Q,
    geocode: Option<String>,
    lang: Option<String>,
    locale: Option<String>,
    result_type: Option<String>,
    count: Option<u64>,
    until: Option<String>,
    since_id: Option<u64>,
    max_id: Option<u64>,
    include_entities: Option<bool>,
}

impl<'a, Q> SearchTweets<'a, Q>
where
    Q: ToString,
{
    pub async fn send(&self) -> Result<SearchResult, Error> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let params = self.to_hashmap();

        self.api.raw_get(endpoint, &params).await
    }
}
