use serde_derive::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Tweet {
    // pub coordinates: Option<RawCoordinates>,
    // #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: String,
    // pub current_user_retweet: Option<CurrentUserRetweet>,
    pub display_text_range: Option<(usize, usize)>,
    // pub entities: TweetEntities,
    // pub extended_entities: Option<ExtendedTweetEntities>,
    // pub extended_tweet: Option<RawExtendedTweet>,
    pub favorite_count: u64,
    pub favorited: Option<bool>,
    // pub filter_level: Option<FilterLevel>,
    pub id: u64,
    pub id_str: String,
    pub in_reply_to_user_id: Option<u64>,
    pub in_reply_to_screen_name: Option<String>,
    pub in_reply_to_status_id: Option<u64>,
    pub lang: Option<String>,
    // pub place: Option<place::Place>,
    pub possibly_sensitive: Option<bool>,
    pub quoted_status_id: Option<u64>,
    pub quoted_status: Option<Box<Tweet>>,
    pub retweet_count: u64,
    pub retweeted: Option<bool>,
    pub retweeted_status: Option<Box<Tweet>>,
    // #[serde(deserialize_with = "deserialize_tweet_source")]
    // pub source: Option<TweetSource>,
    pub text: String,
    // pub full_text: Option<String>,
    pub truncated: bool,
    // pub user: Option<Box<user::TwitterUser>>,
    #[serde(default)]
    pub withheld_copyright: bool,
    pub withheld_in_countries: Option<Vec<String>>,
    pub withheld_scope: Option<String>,
}
