use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
pub struct UserTimeline<'a> {
    api: &'a TwitterAPI,
    #[doc = "The ID of the user for whom to return results."]
    user_id: Option<u64>,
    #[doc = "The screen name of the user for whom to return results."]
    screen_name: Option<String>,
    #[doc = "Returns results with an ID greater than (that is, more recent than) the specified ID. There are limits to the number of Tweets that can be accessed through the API. If the limit of Tweets has occured since the since_id, the since_id will be forced to the oldest ID available."]
    count: Option<u64>,
    #[doc = "Specifies the number of Tweets to try and retrieve, up to a maximum of 200 per distinct request. The value of count is best thought of as a limit to the number of Tweets to return because suspended or deleted content is removed after the count has been applied. We include retweets in the count, even if include_rts is not supplied. It is recommended you always send include_rts=1 when using this API method."]
    since_id: Option<u64>,
    #[doc = "Returns results with an ID less than (that is, older than) or equal to the specified ID."]
    max_id: Option<u64>,
    #[doc = "When set to either true , t or 1 , each Tweet returned in a timeline will include a user object including only the status authors numerical ID. Omit this parameter to receive the complete user object."]
    trim_user: Option<bool>,
    #[doc = "This parameter will prevent replies from appearing in the returned timeline. Using exclude_replies with the count parameter will mean you will receive up-to count tweets — this is because the count parameter retrieves that many Tweets before filtering out retweets and replies."]
    exclude_replies: Option<bool>,
    #[doc = "When set to false , the timeline will strip any native retweets (though they will still count toward both the maximal length of the timeline and the slice selected by the count parameter). Note: If you're using the trim_user parameter in conjunction with include_rts, the retweets will still contain a full user object."]
    include_rts: Option<bool>,
}

impl<'a> UserTimeline<'a> {
    pub async fn send(&self) -> Result<Vec<TrimTweet>, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let params = self.to_hashmap();

        self.api.raw_get(endpoint, &params).await
    }
}
