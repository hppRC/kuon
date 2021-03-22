use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct ShowTweet<'a, Id> {
    api: &'a TwitterAPI,
    id: Id,
    trim_user: Option<bool>,
    include_my_retweet: Option<bool>,
    include_entities: Option<bool>,
    include_ext_alt_text: Option<bool>,
    include_card_uri: Option<bool>,
}

impl<'a, Id> ShowTweet<'a, Id>
where
    Id: ToString,
{
    pub async fn send(&self) -> Result<TrimTweet, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/show.json";
        let params = self.to_hashmap();

        self.api.raw_post(endpoint, &params).await
    }
}
