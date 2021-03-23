use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct HomeTimeline<'a> {
    api: &'a TwitterAPI,
    #[doc = "Specifies the number of records to retrieve. Must be less than or equal to 200. Defaults to 20. The value of count is best thought of as a limit to the number of tweets to return because suspended or deleted content is removed after the count has been applied."]
    count: Option<u64>,
    #[doc = "Returns results with an ID greater than (that is, more recent than) the specified ID. There are limits to the number of Tweets which can be accessed through the API. If the limit of Tweets has occured since the since_id, the since_id will be forced to the oldest ID available."]
    since_id: Option<u64>,
    #[doc = "Returns results with an ID less than (that is, older than) or equal to the specified ID."]
    max_id: Option<u64>,
    #[doc = "When set to either true , t or 1 , each Tweet returned in a timeline will include a user object including only the status authors numerical ID. Omit this parameter to receive the complete user object."]
    trim_user: Option<bool>,
    #[doc = "This parameter will prevent replies from appearing in the returned timeline. Using exclude_replies with the count parameter will mean you will receive up-to count Tweets — this is because the count parameter retrieves that many Tweets before filtering out retweets and replies."]
    exclude_replies: Option<bool>,
    #[doc = "The entities node will not be included when set to false."]
    include_entities: Option<bool>,
}

impl<'a> HomeTimeline<'a> {
    pub async fn send(&self) -> Result<Vec<TrimTweet>, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let params = self.to_hashmap();
        self.api.raw_get(endpoint, &params).await
    }
}
