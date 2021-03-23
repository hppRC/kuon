use crate::{Error, Tweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct Favorite<'a, Id> {
    api: &'a TwitterAPI,
    #[doc = "**(required)**
The numerical ID of the Tweet to like."]
    id: Id,
    #[doc = "The entities node will be omitted when set to false."]
    include_entities: Option<bool>,
}

impl<'a, Id> Favorite<'a, Id>
where
    Id: ToString,
{
    pub async fn send(&self) -> Result<Tweet, Error> {
        let endpoint = "https://api.twitter.com/1.1/favorites/create.json";
        let params = self.to_hashmap();
        self.api.raw_post(endpoint, &params).await
    }
}
