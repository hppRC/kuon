use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_derive::*;
use serde_json::Value;

use crate::UserEntities;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
    pub location: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub protected: bool,
    pub verified: bool,
    pub followers_count: u64,
    pub friends_count: u64,
    pub listed_count: u64,
    pub favourites_count: u64,
    pub statuses_count: u64,
    #[serde(with = "twitter_date")]
    pub created_at: DateTime<Utc>,
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub entities: Option<UserEntities>,

    // Enterprise only
    pub derived: Option<Vec<Value>>,

    // No longer supported (deprecated) attributes
    utc_offset: Option<Value>,
    time_zone: Option<Value>,
    lang: Option<Value>,
    geo_enabled: Option<Value>,
    following: Option<Value>,
    follow_request_sent: Option<Value>,
    has_extended_profile: Option<Value>,
    notifications: Option<Value>,
    profile_location: Option<Value>,
    contributors_enabled: Option<Value>,
    profile_image_url: Option<Value>,
    profile_background_color: Option<Value>,
    profile_background_image_url: Option<Value>,
    profile_background_image_url_https: Option<Value>,
    profile_background_tile: Option<Value>,
    profile_link_color: Option<Value>,
    profile_sidebar_border_color: Option<Value>,
    profile_sidebar_fill_color: Option<Value>,
    profile_text_color: Option<Value>,
    profile_use_background_image: Option<Value>,
    is_translator: Option<Value>,
    is_translation_enabled: Option<Value>,
    translator_type: Option<Value>,
}

mod twitter_date {
    use chrono::{DateTime, Utc};
    use serde::de::{Unexpected, Visitor};
    use serde::{Deserializer, Serializer};

    struct TwitterDateTimeVisitor;
    impl<'de> Visitor<'de> for TwitterDateTimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            formatter.write_str("a string in the format by \"%a %b %d %H:%M:%S %z %Y\"")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            chrono::DateTime::parse_from_str(v, "%a %b %d %H:%M:%S %z %Y")
                .map(|x| x.with_timezone(&Utc))
                .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &self))
        }
    }

    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.format("%a %b %d %H:%M:%S %z %Y").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TwitterDateTimeVisitor)
    }
}

#[test]
fn test_serde_user() {
    use crate::{Url, UrlEntities};
    use chrono::TimeZone;

    let user = User {
        id: 6253282,
        id_str: "6253282".to_string(),
        name: "Twitter API".to_string(),
        screen_name: "TwitterAPI".to_string(),
        location: Some("San Francisco, CA".to_string()),
        derived: None,
        url: Some("https://t.co/8IkCzCDr19".to_string()),
        description: Some("The Real Twitter API. Tweets about API changes, service issues and our Developer Platform. Don\'t get an answer? It\'s on my website.".to_string()), protected: false,
        verified: true,
        followers_count: 6133636,
        friends_count: 12,
        listed_count: 12936,
        favourites_count: 31,
        statuses_count: 3656,
        created_at: Utc.ymd(2007, 3, 23).and_hms(6, 1, 13),
        profile_banner_url: None,
        profile_image_url_https: "https://pbs.twimg.com/profile_images/942858479592554497/BbazLO9L_normal.jpg".to_string(),
        default_profile: false,
        default_profile_image: false,
        entities: Some(
            UserEntities {
                url: Some(
                    Url {
                        urls: vec![
                            UrlEntities {
                                url: Some("https://t.co/8IkCzCDr19".to_string()),
                                expanded_url: Some("https://developer.twitter.com".to_string()),
                                display_url: Some("developer.twitter.com".to_string()),
                                indices: vec![0, 23]
                            }
                        ]
                    }
                ),
                description: Some(Url { urls: vec![] })
            }
        ),
        utc_offset: None,
        time_zone: None,
        lang: None,
        geo_enabled: None,
        following: None,
        follow_request_sent: None,
        has_extended_profile: None,
        notifications: None,
        profile_location: None,
        contributors_enabled: None,
        profile_image_url: None,
        profile_background_color: None,
        profile_background_image_url: None,
        profile_background_image_url_https: None,
        profile_background_tile: None,
        profile_link_color: None,
        profile_sidebar_border_color: None,
        profile_sidebar_fill_color: None,
        profile_text_color: None,
        profile_use_background_image: None,
        is_translator: None,
        is_translation_enabled: None,
        translator_type: None
    };

    let actual = serde_json::to_string(&user);
    if let Err(err) = actual {
        panic!("Failed: {}", err);
    }

    let actual = actual.and_then(|x| serde_json::from_str::<User>(&x));
    match actual {
        Ok(actual) => assert_eq!(format!("{:?}", user), format!("{:?}", actual)),
        Err(err) => panic!("{}", err),
    }
}
