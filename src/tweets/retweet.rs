use std::{collections::HashMap, fmt::Display};

use crate::{Error, Tweet, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;

#[derive(Clone, Debug)]
pub struct RetweetRequest<'a, Id> {
    api: &'a TwitterAPI,
    id: Id,
    trim_user: Option<bool>,
}

impl TwitterAPI {
    pub fn retweet(&self) -> RetweetRequest<()> {
        RetweetRequest {
            api: self,
            id: (),
            trim_user: None,
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
            id,
            trim_user: self.trim_user,
        }
    }
}

impl<'a, Id> RetweetRequest<'a, Id> {
    pub fn trim_user(&self, trim_user: bool) -> RetweetRequest<'a, Id> {
        RetweetRequest {
            api: self.api,
            id: self.id,
            trim_user: Some(trim_user),
        }
    }
}

impl<'a, Id> RetweetRequest<'a, Id>
where
    Id: Display,
{
    pub async fn send(&self) -> Result<Tweet, Error> {
        let endpoint = &format!(
            "https://api.twitter.com/1.1/statuses/retweet/{}.json",
            self.id
        );
        let mut params: HashMap<&str, String> = hashmap! {};
        if let Some(trim_user) = self.trim_user {
            params.insert("trim_user", trim_user.to_string());
        }

        self.api.raw_post(endpoint, &params).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() -> Result<()> {
        let api: TwitterAPI = TwitterAPI::new_using_env().await?;
        let res = api.retweet().id(1).send().await?;
        Ok(())
    }
}
