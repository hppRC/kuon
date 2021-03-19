use std::{collections::HashMap, fmt::Display};

use crate::{Error, Tweet, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;
#[derive(Clone, Debug)]
pub struct RetweetRequest<'a, Id> {
    api: &'a TwitterAPI,
    required_params: RetweetRequestRequiredParams<Id>,
    optional_params: RetweetRequestOptionalParams,
}

#[derive(Clone, Debug, Default)]
pub struct RetweetRequestRequiredParams<Id> {
    id: Id,
}
#[derive(Clone, Debug, Default)]
pub struct RetweetRequestOptionalParams {
    trim_user: Option<bool>,
}

impl TwitterAPI {
    pub fn retweet(&self) -> RetweetRequest<()> {
        RetweetRequest {
            api: self,
            required_params: Default::default(),
            optional_params: Default::default(),
        }
    }
}

impl<'a> RetweetRequest<'a, ()> {
    pub fn id<Id>(&self, id: Id) -> RetweetRequest<'a, Id>
    where
        Id: Display,
    {
        RetweetRequest {
            api: self.api,
            required_params: RetweetRequestRequiredParams { id },
            optional_params: self.optional_params.clone(),
        }
    }
}

impl<'a, Id> RetweetRequest<'a, Id> {
    pub fn trim_user(&mut self, trim_user: bool) -> &mut Self {
        self.optional_params.trim_user = Some(trim_user);
        self
    }
}

impl<'a, Id> RetweetRequest<'a, Id>
where
    Id: Display,
{
    pub async fn send(&self) -> Result<Tweet, Error> {
        let endpoint = &format!(
            "https://api.twitter.com/1.1/statuses/retweet/{}.json",
            self.required_params.id
        );
        let mut params: HashMap<&str, String> = hashmap! {};
        if let Some(trim_user) = self.optional_params.trim_user {
            params.insert("trim_user", trim_user.to_string());
        }

        self.api.raw_post(endpoint, &params).await
    }
}
