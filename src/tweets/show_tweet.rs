use std::fmt::Display;

use crate::{Error, Tweet, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;

#[derive(Clone, Debug)]
pub struct ShowTweetRequest<'a, Id> {
    api: &'a TwitterAPI,
    required_params: ShowTweetRequestRequiredParams<Id>,
    optional_params: ShowTweetRequestOptionalParams,
}

#[derive(Clone, Debug, Default)]
pub struct ShowTweetRequestRequiredParams<Id> {
    id: Id,
}
#[derive(Clone, Debug, Default)]
pub struct ShowTweetRequestOptionalParams {
    trim_user: Option<bool>,
    include_my_retweet: Option<bool>,
    include_entities: Option<bool>,
    include_ext_alt_text: Option<bool>,
    include_card_uri: Option<bool>,
}

impl TwitterAPI {
    pub fn show_tweet<'a>(&'a self) -> ShowTweetRequest<()> {
        ShowTweetRequest {
            api: self,
            required_params: Default::default(),
            optional_params: Default::default(),
        }
    }
}

impl<'a> ShowTweetRequest<'a, ()> {
    pub fn id<Id>(&mut self, id: Id) -> ShowTweetRequest<'a, Id>
    where
        Id: ToString,
    {
        ShowTweetRequest {
            api: self.api,
            required_params: ShowTweetRequestRequiredParams { id },
            optional_params: self.optional_params.clone(),
        }
    }
}

impl<'a, Id> ShowTweetRequest<'a, Id> {
    pub fn trim_user(&mut self, trim_user: bool) -> &mut Self {
        self.optional_params.trim_user = Some(trim_user);
        self
    }
    pub fn include_my_retweet(&mut self, include_my_retweet: bool) -> &mut Self {
        self.optional_params.include_my_retweet = Some(include_my_retweet);
        self
    }
    pub fn include_entities(&mut self, include_entities: bool) -> &mut Self {
        self.optional_params.include_entities = Some(include_entities);
        self
    }
    pub fn include_ext_alt_text(&mut self, include_ext_alt_text: bool) -> &mut Self {
        self.optional_params.include_ext_alt_text = Some(include_ext_alt_text);
        self
    }
    pub fn include_card_uri(&mut self, include_card_uri: bool) -> &mut Self {
        self.optional_params.include_card_uri = Some(include_card_uri);
        self
    }
}

impl<'a, Id> ShowTweetRequest<'a, Id>
where
    Id: Display,
{
    pub async fn send(&self) -> Result<Tweet, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/show.json";
        let mut params = hashmap! {"id" => self.required_params.id.to_string()};

        if let Some(trim_user) = self.optional_params.trim_user {
            params.insert("trim_user", trim_user.to_string());
        }
        if let Some(include_my_retweet) = self.optional_params.include_my_retweet {
            params.insert("include_my_retweet", include_my_retweet.to_string());
        }
        if let Some(include_entities) = self.optional_params.include_entities {
            params.insert("include_entities", include_entities.to_string());
        }
        if let Some(include_ext_alt_text) = self.optional_params.include_ext_alt_text {
            params.insert("include_ext_alt_text", include_ext_alt_text.to_string());
        }
        if let Some(include_card_uri) = self.optional_params.include_card_uri {
            params.insert("include_card_uri", include_card_uri.to_string());
        }

        self.api.raw_post(endpoint, &params).await
    }
}
