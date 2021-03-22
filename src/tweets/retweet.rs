use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;
use std::fmt::Display;

#[derive(Clone, Debug, KuonRequest)]
pub struct Retweet<'a, Id> {
    api: &'a TwitterAPI,
    id: Id,
    trim_user: Option<bool>,
}

impl<'a, Id> Retweet<'a, Id>
where
    Id: Display,
{
    // pub async fn send(&self) -> Result<RetweetResult, Error> {
    pub async fn send(&self) -> Result<TrimTweet, Error> {
        let endpoint = &format!(
            "https://api.twitter.com/1.1/statuses/retweet/{}.json",
            self.id
        );
        let params = self.to_hashmap();

        self.api.raw_post(endpoint, &params).await
    }
}
