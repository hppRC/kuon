use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
#[doc = r#"

# Example

```no_run
# use anyhow::Result;
# async fn doc() -> Result<()> {
let api = kuon::TwitterAPI::new_using_env().await?;
let res = api.user_timeline().screen_name("rustlang").count(10).send().await?;
for tweet in res {
    println!("{}", tweet.text);
}
# Ok(())
# }
```

# GET statuses/user_timeline
Important notice: On June 19, 2019, we began enforcing a limit of 100,000 requests per day to the /statuses/user_timeline endpoint, in addition to existing user-level and app-level rate limits. This limit is applied on a per-application basis, meaning that a single developer app can make up to 100,000 calls during any single 24-hour period.

Returns a collection of the most recent Tweets posted by the user indicated by the screen_name or user_id parameters.

User timelines belonging to protected users may only be requested when the authenticated user either "owns" the timeline or is an approved follower of the owner.

The timeline returned is the equivalent of the one seen as a user's profile on Twitter.

This method can only return up to 3,200 of a user's most recent Tweets. Native retweets of other statuses by the user is included in this total, regardless of whether include_rts is set to false when requesting this resource.

See Working with Timelines for instructions on traversing timelines.

See Embedded Timelines, Embedded Tweets, and GET statuses/oembed for tools to render Tweets according to Display Requirements.



## Resource Information

Q.|A.
-|-
Requires authentication?|Yes
Rate limited?|Yes
Requests / 15-min window (user auth)|900
Requests / 15-min window (app auth)|1500
Requests / 24-hour window|100,000

https://developer.twitter.com/en/docs/twitter-api/v1/tweets/timelines/api-reference/get-statuses-user_timeline
"#]
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
