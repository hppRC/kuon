use crate::models::*;
use crate::serde_twitter_data::{optional_twitter_date, twitter_date};
use chrono::{DateTime, Utc};
use serde_derive::*;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tweet {
    /// UTC time when this Tweet was created.
    #[serde(with = "twitter_date")]
    pub created_at: DateTime<Utc>,
    /// The integer representation of the unique identifier for this Tweet.
    /// This number is greater than 53 bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// Using a signed 64 bit integer for storing this identifier is safe.
    /// Use `id_str` to fetch the identifier to be safe. See [Twitter IDs](https://developer.twitter.com/en/docs/twitter-ids) for more information.
    pub id: u64,
    /// The string representation of the unique identifier for this Tweet.
    /// Implementations should use this rather than the large integer in `id`.
    pub id_str: Option<String>,
    /// The actual UTF-8 text of the status update.
    /// See [twitter-text](https://github.com/twitter/twitter-text/blob/master/rb/lib/twitter-text/regex.rb) for details on what characters are currently considered valid.
    pub text: String,
    /// Utility used to post the Tweet, as an HTML-formatted string.
    /// Tweets from the Twitter website have a source value of web.
    pub source: Option<String>,
    /// Indicates whether the value of the `text` parameter was truncated, for example, as a result of a retweet exceeding the original Tweet text length limit of 140 characters.
    /// Truncated text will end in ellipsis, like this `...` Since Twitter now rejects long Tweets vs truncating them, the large majority of Tweets will have this set to `false`.
    /// Note that while native retweets may have their toplevel `text` property shortened, the original text will be available under the `retweeted_status` object and the `truncated` parameter will be set to the value of the original status (in most cases, `false` ).
    pub truncated: bool,
    pub entities: TweetEntities,
    pub extended_entities: Option<ExtendedEntities>,
    pub metadata: Option<TweetMetadata>,
    /// The user who posted this Tweet. See User data dictionary for complete list of attributes.
    pub user: User,

    /// Nullable. If the represented Tweet is a reply, this field will contain the integer representation of the original Tweet’s ID.
    pub in_reply_to_status_id: Option<u64>,
    /// Nullable. If the represented Tweet is a reply, this field will contain the string representation of the original Tweet’s ID.
    pub in_reply_to_status_id_str: Option<String>,
    /// Nullable. If the represented Tweet is a reply, this field will contain the integer representation of the original Tweet’s author ID. This will not necessarily always be the user directly mentioned in the Tweet.
    pub in_reply_to_user_id: Option<u64>,
    /// Nullable. If the represented Tweet is a reply, this field will contain the string representation of the original Tweet’s author ID. This will not necessarily always be the user directly mentioned in the Tweet.
    pub in_reply_to_user_id_str: Option<String>,
    /// Nullable. If the represented Tweet is a reply, this field will contain the screen name of the original Tweet’s author.
    pub in_reply_to_screen_name: Option<String>,

    pub geo: Option<Value>,
    /// Nullable. Represents the geographic location of this Tweet as reported by the user or client application. The inner coordinates array is formatted as geoJSON (longitude first, then latitude).
    pub coordinates: Option<Value>, //TODO: implement correct type
    /// Nullable When present, indicates that the tweet is associated (but not necessarily originating from) a Place
    pub place: Option<Value>,
    pub contributors: Option<Value>, //TODO: implement correct type
    pub is_quote_status: bool,
    pub quoted_status_id: Option<u64>,
    pub quoted_status_id_str: Option<String>,

    pub quoted_status: Option<Box<Tweet>>,

    pub retweet_count: u64,
    pub favorite_count: u64,
    pub favorited: bool,
    pub retweeted: bool,
    pub retweeted_status: Option<Box<Tweet>>,
    pub possibly_sensitive: Option<bool>,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TweetMetadata {
    iso_language_code: Option<String>,
    result_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrimTweet {
    /// UTC time when this Tweet was created.
    #[serde(with = "optional_twitter_date")]
    pub created_at: Option<DateTime<Utc>>,
    /// The integer representation of the unique identifier for this Tweet.
    /// This number is greater than 53 bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// Using a signed 64 bit integer for storing this identifier is safe.
    /// Use `id_str` to fetch the identifier to be safe. See [Twitter IDs](https://developer.twitter.com/en/docs/twitter-ids) for more information.
    pub id: u64,
    /// The string representation of the unique identifier for this Tweet.
    /// Implementations should use this rather than the large integer in `id`.
    pub id_str: Option<String>,
    /// The actual UTF-8 text of the status update.
    /// See [twitter-text](https://github.com/twitter/twitter-text/blob/master/rb/lib/twitter-text/regex.rb) for details on what characters are currently considered valid.
    pub text: String,
    /// Utility used to post the Tweet, as an HTML-formatted string.
    /// Tweets from the Twitter website have a source value of web.
    pub source: Option<String>,
    /// Indicates whether the value of the `text` parameter was truncated, for example, as a result of a retweet exceeding the original Tweet text length limit of 140 characters.
    /// Truncated text will end in ellipsis, like this `...` Since Twitter now rejects long Tweets vs truncating them, the large majority of Tweets will have this set to `false`.
    /// Note that while native retweets may have their toplevel `text` property shortened, the original text will be available under the `retweeted_status` object and the `truncated` parameter will be set to the value of the original status (in most cases, `false` ).
    pub truncated: bool,
    pub entities: TweetEntities,
    pub extended_entities: Option<ExtendedEntities>,
    pub metadata: Option<TweetMetadata>,
    /// The user who posted this Tweet. See User data dictionary for complete list of attributes.
    pub user: TrimUser,

    /// Nullable. If the represented Tweet is a reply, this field will contain the integer representation of the original Tweet’s ID.
    pub in_reply_to_status_id: Option<u64>,
    /// Nullable. If the represented Tweet is a reply, this field will contain the string representation of the original Tweet’s ID.
    pub in_reply_to_status_id_str: Option<String>,
    /// Nullable. If the represented Tweet is a reply, this field will contain the integer representation of the original Tweet’s author ID. This will not necessarily always be the user directly mentioned in the Tweet.
    pub in_reply_to_user_id: Option<u64>,
    /// Nullable. If the represented Tweet is a reply, this field will contain the string representation of the original Tweet’s author ID. This will not necessarily always be the user directly mentioned in the Tweet.
    pub in_reply_to_user_id_str: Option<String>,
    /// Nullable. If the represented Tweet is a reply, this field will contain the screen name of the original Tweet’s author.
    pub in_reply_to_screen_name: Option<String>,

    pub geo: Option<String>,
    /// Nullable. Represents the geographic location of this Tweet as reported by the user or client application. The inner coordinates array is formatted as geoJSON (longitude first, then latitude).
    pub coordinates: Option<Value>, //TODO: implement correct type
    /// Nullable When present, indicates that the tweet is associated (but not necessarily originating from) a Place
    pub place: Option<Value>,
    pub contributors: Option<Value>, //TODO: implement correct type
    pub is_quote_status: bool,
    pub quoted_status_id: Option<u64>,
    pub quoted_status_id_str: Option<String>,

    pub quoted_status: Option<Box<Tweet>>,

    pub retweet_count: u64,
    pub favorite_count: u64,
    pub favorited: bool,
    pub retweeted: bool,
    pub retweeted_status: Option<Box<TrimTweet>>,
    pub possibly_sensitive: Option<bool>,
    pub lang: Option<String>,
}
