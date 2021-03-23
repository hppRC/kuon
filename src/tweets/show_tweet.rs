use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct ShowTweet<'a, Id> {
    api: &'a TwitterAPI,
    #[doc = "**(required)**
The numerical ID of the desired Tweet."]
    id: Id,
    #[doc = "When set to either true, t or 1, each Tweet returned in a timeline will include a user object including only the status authors numerical ID. Omit this parameter to receive the complete user object."]
    trim_user: Option<bool>,
    #[doc = "When set to either true, t or 1, any Tweets returned that have been retweeted by the authenticating user will include an additional current_user_retweet node, containing the ID of the source status for the retweet."]
    include_my_retweet: Option<bool>,
    #[doc = "The entities node will not be included when set to false."]
    include_entities: Option<bool>,
    #[doc = "If alt text has been added to any attached media entities, this parameter will return an ext_alt_text value in the top-level key for the media entity. If no value has been set, this will be returned as `null`"]
    include_ext_alt_text: Option<bool>,
    #[doc = "When set to either true , t or 1 , the retrieved Tweet will include a card_uri attribute when there is an ads card attached to the Tweet and when that card was attached using the card_uri value."]
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
