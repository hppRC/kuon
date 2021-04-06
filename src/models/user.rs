use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_derive::*;
use serde_json::Value;

use crate::serde_twitter_data::{optional_twitter_date, twitter_date};
use crate::UserEntities;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// The integer representation of the unique identifier for this User.
    /// This number is greater than 53 bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// Using a signed 64 bit integer for storing this identifier is safe.
    /// Use `id_str` to fetch the identifier to be safe. See [Twitter IDs](https://developer.twitter.com/en/docs/twitter-ids) for more information.
    pub id: u64,
    /// The string representation of the unique identifier for this User.
    /// Implementations should use this rather than the large, possibly un-consumable integer in `id`.
    pub id_str: String,
    /// The name of the user, as they’ve defined it. Not necessarily a person’s name.
    /// Typically capped at 50 characters, but subject to change.
    pub name: String,
    /// The screen name, handle, or alias that this user identifies themselves with.
    /// screen_names are unique but subject to change. Use `id_str` as a user identifier whenever possible.
    /// Typically a maximum of 15 characters long, but some historical accounts may exist with longer names.
    pub screen_name: String,
    pub location: Option<String>,
    /// Nullable. A URL provided by the user in association with their profile.
    pub url: Option<String>,
    /// Nullable. The user-defined UTF-8 string describing their account.
    pub description: Option<String>,
    /// When true, indicates that this user has chosen to protect their Tweets.
    /// See [About Public and Protected Tweets](https://help.twitter.com/en/safety-and-security/public-and-protected-tweets).
    pub protected: bool,
    /// When true, indicates that the user has a verified account.
    /// See [Verified Accounts](https://help.twitter.com/en/managing-your-account/about-twitter-verified-accounts).
    pub verified: bool,

    /// The number of followers this account currently has. Under certain conditions of duress, this field will temporarily indicate “0”.
    pub followers_count: u64,
    /// The number of users this account is following (AKA their “followings”). Under certain conditions of duress, this field will temporarily indicate “0”.
    pub friends_count: u64,
    /// The number of public lists that this user is a member of.
    pub listed_count: u64,
    /// The number of Tweets this user has liked in the account’s lifetime. British spelling used in the field name for historical reasons.
    pub favourites_count: u64,
    /// The number of Tweets (including retweets) issued by the user.
    pub statuses_count: u64,

    /// The UTC datetime that the user account was created on Twitter.
    #[serde(with = "twitter_date")]
    pub created_at: DateTime<Utc>,
    /// The HTTPS-based URL pointing to the standard web representation of the user’s uploaded profile banner.
    /// By adding a final path element of the URL, it is possible to obtain different image sizes optimized for specific displays.
    /// For size variants, please see [User Profile Images and Banners](https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/user-profile-images-and-banners).
    pub profile_banner_url: Option<String>,
    /// A HTTPS-based URL pointing to the user’s profile image.
    pub profile_image_url_https: String,
    /// When true, indicates that the user has not altered the theme or background of their user profile.
    pub default_profile: bool,
    /// When true, indicates that the user has not uploaded their own profile image and a default image is used instead.
    pub default_profile_image: bool,
    pub entities: Option<UserEntities>,

    /// Enterprise only\
    /// Enterprise APIs only Collection of Enrichment metadata derived for user.
    /// Provides the Profile Geo Enrichment metadata. See referenced documentation for more information, including JSON data dictionaries.
    pub derived: Option<Value>,

    /// When present, indicates a list of uppercase two-letter country codes this content is withheld from.
    /// Twitter supports the following non-country values for this field:
    ///
    /// “XX” - Content is withheld in all countries “XY” - Content is withheld due to a DMCA request.
    pub withheld_in_countries: Option<Vec<String>>,

    /// When present, indicates that the content being withheld is a “user.”
    pub withheld_scope: Option<String>,

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrimUser {
    /// The integer representation of the unique identifier for this User.
    /// This number is greater than 53 bits and some programming languages may have difficulty/silent defects in interpreting it.
    /// Using a signed 64 bit integer for storing this identifier is safe.
    /// Use `id_str` to fetch the identifier to be safe. See [Twitter IDs](https://developer.twitter.com/en/docs/twitter-ids) for more information.
    pub id: u64,
    /// The string representation of the unique identifier for this User.
    /// Implementations should use this rather than the large, possibly un-consumable integer in `id`.
    pub id_str: String,
    pub name: Option<String>,
    pub screen_name: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub protected: Option<bool>,
    pub verified: Option<bool>,

    /// The number of followers this account currently has. Under certain conditions of duress, this field will temporarily indicate “0”.
    pub followers_count: Option<u64>,
    /// The number of users this account is following (AKA their “followings”). Under certain conditions of duress, this field will temporarily indicate “0”.
    pub friends_count: Option<u64>,
    /// The number of public lists that this user is a member of.
    pub listed_count: Option<u64>,
    /// The number of Tweets this user has liked in the account’s lifetime. British spelling used in the field name for historical reasons.
    pub favourites_count: Option<u64>,
    /// The number of Tweets (including retweets) issued by the user.
    pub statuses_count: Option<u64>,

    /// The UTC datetime that the user account was created on Twitter.
    #[serde(with = "optional_twitter_date")]
    pub created_at: Option<DateTime<Utc>>,

    /// The HTTPS-based URL pointing to the standard web representation of the user’s uploaded profile banner.
    /// By adding a final path element of the URL, it is possible to obtain different image sizes optimized for specific displays.
    /// For size variants, please see [User Profile Images and Banners](https://developer.twitter.com/en/docs/twitter-api/v1/accounts-and-users/user-profile-images-and-banners).
    pub profile_banner_url: Option<String>,
    /// A HTTPS-based URL pointing to the user’s profile image.
    pub profile_image_url_https: Option<String>,
    /// When true, indicates that the user has not altered the theme or background of their user profile.
    pub default_profile: Option<bool>,
    /// When true, indicates that the user has not uploaded their own profile image and a default image is used instead.
    pub default_profile_image: Option<bool>,
    pub entities: Option<UserEntities>,

    /// Enterprise only\
    /// Enterprise APIs only Collection of Enrichment metadata derived for user.
    /// Provides the Profile Geo Enrichment metadata. See referenced documentation for more information, including JSON data dictionaries.
    pub derived: Option<Value>,

    /// When present, indicates a list of uppercase two-letter country codes this content is withheld from.
    /// Twitter supports the following non-country values for this field:
    ///
    /// “XX” - Content is withheld in all countries “XY” - Content is withheld due to a DMCA request.
    pub withheld_in_countries: Option<Vec<String>>,

    /// When present, indicates that the content being withheld is a “user.”
    pub withheld_scope: Option<String>,

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
        withheld_in_countries: None,
        withheld_scope: None,
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
        translator_type: None,
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
