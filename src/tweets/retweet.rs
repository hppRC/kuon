use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;
use std::fmt::Display;

#[derive(Clone, Debug, KuonRequest)]
#[doc = r#"
# POST statuses/retweet/:id

Retweets a tweet. Returns the original Tweet with Retweet details embedded.

Usage Notes:
- This method is subject to update limits. A HTTP 403 will be returned if this limit as been hit.
- Twitter will ignore attempts to perform duplicate retweets.
- The retweet_count will be current as of when the payload is generated and may not reflect the exact count. It is intended as an approximation.

## Resource Information

Q.|A.
-|-
Requires authentication?|Yes (user context only)
Rate limited?|Yes
Requests / 3-hour window|300* per user; 300* per app

https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/api-reference/post-statuses-retweet-id
"#]
pub struct Retweet<'a, Id> {
    api: &'a TwitterAPI,
    #[doc = "**(required)**
The numerical ID of the desired status."]
    id: Id,
    #[doc = "When set to either true , t or 1 , each tweet returned in a timeline will include a user object including only the status authors numerical ID. Omit this parameter to receive the complete user object."]
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
