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
let tweet = api.show_tweet().id(1283742285381816320u64).send().await?;
# Ok(())
# }
```

# GET statuses/show/:id
Returns a single Tweet, specified by the id parameter. The Tweet's author will also be embedded within the Tweet.

See GET statuses / lookup for getting Tweets in bulk (up to 100 per call). See also Embedded Timelines, Embedded Tweets, and GET statuses/oembed for tools to render Tweets according to Display Requirements.

### About Geo
If there is no geotag for a status, then there will be an empty `<geo></geo>` or `"geo" : {}`. This can only be populated if the user has used the Geotagging API to send a statuses/update.

The JSON response mostly uses conventions laid out in GeoJSON. The coordinates that Twitter renders are reversed from the GeoJSON specification (GeoJSON specifies a longitude then a latitude, whereas Twitter represents it as a latitude then a longitude), eg: `"geo": { "type":"Point", "coordinates":[37.78029, -122.39697] }`

Q.|A.
-|-
Requires authentication?|Yes
Rate limited?|Yes
Requests / 15-min window (user auth)|900
Requests / 15-min window (app auth)|900

https://developer.twitter.com/en/docs/twitter-api/v1/tweets/post-and-engage/api-reference/get-statuses-show-id
"#]
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
