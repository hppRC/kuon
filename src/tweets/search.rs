use crate::{Error, SearchResult, TwitterAPI};
use anyhow::Result;
use kuon_macro::KuonRequest;

#[derive(Clone, Debug, KuonRequest)]
#[doc = r#"

# Example

```no_run
# use anyhow::Result;
# async fn doc() -> Result<()> {
let api = kuon::TwitterAPI::new_using_env().await?;
let res = api.search_tweets().q("rust").count(100).send().await?;
for tweet in res.statuses {
    println!("{}", tweet.text);
}
# Ok(())
# }
```

# Standard search API

Returns a collection of relevant Tweets matching a specified query.

Please note that Twitter's search service and, by extension, the Search API is not meant to be an exhaustive source of Tweets. Not all Tweets will be indexed or made available via the search interface.

To learn how to use Twitter Search effectively, please see the Standard search operators page for a list of available filter operators. Also, see the Working with Timelines page to learn best practices for navigating results by since_id and max_id.

Q.|A.
-|-
Requires authentication?|Yes
Rate limited?|Yes
Requests / 15-min window (user auth)|180
Requests / 15-min window (app auth)|450

https://developer.twitter.com/en/docs/twitter-api/v1/tweets/search/api-reference/get-search-tweets
"#]
pub struct SearchTweets<'a, Q> {
    api: &'a TwitterAPI,
    #[doc = "**(required)**
A UTF-8, URL-encoded search query of 500 characters maximum, including operators. Queries may additionally be limited by complexity."]
    q: Q,
    #[doc = r#"Returns tweets by users located within a given radius of the given latitude/longitude. The location is preferentially taking from the Geotagging API, but will fall back to their Twitter profile. The parameter value is specified by " latitude,longitude,radius ", where radius units must be specified as either " mi " (miles) or " km " (kilometers). Note that you cannot use the near operator via the API to geocode arbitrary locations; however you can use this geocode parameter to search near geocodes directly. A maximum of 1,000 distinct "sub-regions" will be considered when using the radius modifier."#]
    geocode: Option<String>,
    #[doc = "Restricts tweets to the given language, given by an [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) code. Language detection is best-effort."]
    lang: Option<String>,
    #[doc = "Specify the language of the query you are sending (only ja is currently effective). This is intended for language-specific consumers and the default should work in the majority of cases."]
    locale: Option<String>,
    #[doc = r#"Optional. Specifies what type of search results you would prefer to receive. The current default is "mixed." Valid values include:
* `mixed` : Include both popular and real time results in the response.
* `recent` : return only the most recent results in the response
* `popular` : return only the most popular results in the response."#]
    result_type: Option<String>,
    #[doc = r#"The number of tweets to return per page, up to a maximum of 100. Defaults to 15. This was formerly the "rpp" parameter in the old Search API."#]
    count: Option<u64>,
    #[doc = "Returns tweets created before the given date. Date should be formatted as YYYY-MM-DD. Keep in mind that the search index has a 7-day limit. In other words, no tweets will be found for a date older than one week."]
    until: Option<String>,
    #[doc = "Returns results with an ID greater than (that is, more recent than) the specified ID. There are limits to the number of Tweets which can be accessed through the API. If the limit of Tweets has occured since the since_id, the since_id will be forced to the oldest ID available."]
    since_id: Option<u64>,
    #[doc = "Returns results with an ID less than (that is, older than) or equal to the specified ID."]
    max_id: Option<u64>,
    #[doc = "The `entities` node will not be included when set to `false`."]
    include_entities: Option<bool>,
}

impl<'a, Q> SearchTweets<'a, Q>
where
    Q: ToString,
{
    pub async fn send(&self) -> Result<SearchResult, Error> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let params = self.to_hashmap();

        self.api.raw_get(endpoint, &params).await
    }
}
