use crate::{Error, FollowersIdsResult, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
#[doc = r#"
# GET followers/ids
Returns a cursored collection of user IDs for every user following the specified user.
At this time, results are ordered with the most recent following first — however, this ordering is subject to unannounced change and eventual consistency issues. Results are given in groups of 5,000 user IDs and multiple "pages" of results can be navigated through using the next_cursor value in subsequent requests. See Using cursors to navigate collections for more information.
This method is especially powerful when used in conjunction with GET users / lookup, a method that allows you to convert user IDs into full user objects in bulk.

## Resource Information

Q.|A.
-|-
Response formats|JSON
Requires authentication?|Yes
Rate limited?|Yes
Requests / 15-min window (user auth)|15
Requests / 15-min window (app auth)|15

https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/follow-search-get-users/api-reference/get-followers-ids
"#]
pub struct FollowersIds<'a> {
    api: &'a TwitterAPI,
    #[doc = "The ID of the user for whom to return results."]
    user_id: Option<u64>,
    #[doc = "The screen name of the user for whom to return results."]
    screen_name: Option<String>,
    #[doc = r#"Causes the list of connections to be broken into pages of no more than 5000 IDs at a time. The number of IDs returned is not guaranteed to be 5000 as suspended users are filtered out after connections are queried. If no cursor is provided, a value of -1 will be assumed, which is the first "page."
The response from the API will include a previous_cursor and next_cursor to allow paging back and forth. See [Using cursors to navigate collections](https://developer.twitter.com/en/docs/pagination) for more information."#]
    cursor: Option<i64>,

    // This is used to determine whether to accept the type of ids as an integer or a string.
    // Rust's type system cannot accept properties with multiple types like this.
    // Therefore, this option should be omitted.
    //
    // #[doc = "Some programming environments will not consume Twitter IDs due to their size. Provide this option to have IDs returned as strings instead. More about [Twitter IDs.](https://developer.twitter.com/en/docs/twitter-ids)"]
    // stringify_ids: Option<bool>,
    #[doc = "Specifies the number of IDs attempt retrieval of, up to a maximum of 5,000 per distinct request. The value of count is best thought of as a limit to the number of results to return. When using the count parameter with this method, it is wise to use a consistent count value across all requests to the same user's collection. Usage of this parameter is encouraged in environments where all 5,000 IDs constitutes too large of a response."]
    count: Option<u64>,
}

impl<'a> FollowersIds<'a> {
    pub async fn send(&self) -> Result<FollowersIdsResult, Error> {
        let endpoint = "https://api.twitter.com/1.1/followers/ids.json";
        let params = self.to_hashmap();

        self.api.raw_get(endpoint, &params).await
    }
}
