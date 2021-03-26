use crate::{Error, Tweet, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
#[doc = r#"

# Example

```no_run
# use anyhow::Result;
# async fn doc() -> Result<()> {
let api = kuon::TwitterAPI::new_using_env().await?;
let res = api.favorite().id(0).include_entities(true).send().await?;
# Ok(())
# }
```

# POST favorites/create
Note: favorites are now known as likes.
Favorites (likes) the Tweet specified in the ID parameter as the authenticating user. Returns the favorite Tweet when successful.
The process invoked by this method is asynchronous. The immediately returned Tweet object may not indicate the resultant favorited status of the Tweet. A 200 OK response from this method will indicate whether the intended action was successful or not.

## Resource Information

Q.|A.
-|-
Requires authentication?|Yes (user context only)
Rate limited?|Yes
Requests / 24-hour window|1000 per user; 1000 per app

https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/api-reference/post-favorites-create
"#]
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
