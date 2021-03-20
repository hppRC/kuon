use crate::{Error, FollowersIdsResult, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FollowersIdsRequest<'a> {
    api: &'a TwitterAPI,
    optional_params: FollowersIdsRequestOptionalParams,
}

#[derive(Clone, Debug, Default)]
pub struct FollowersIdsRequestOptionalParams {
    user_id: Option<u64>,
    screen_name: Option<String>,
    cursor: Option<i64>,
    stringify_ids: Option<bool>,
    count: Option<u64>,
}

impl TwitterAPI {
    pub fn followers_ids(&self) -> FollowersIdsRequest {
        FollowersIdsRequest {
            api: self,
            optional_params: Default::default(),
        }
    }
}

impl<'a> FollowersIdsRequest<'a> {
    pub fn user_id(&mut self, user_id: u64) -> &mut Self {
        self.optional_params.user_id = Some(user_id);
        self
    }
    pub fn screen_name(&mut self, screen_name: impl Into<String>) -> &mut Self {
        self.optional_params.screen_name = Some(screen_name.into());
        self
    }
    pub fn cursor(&mut self, cursor: i64) -> &mut Self {
        self.optional_params.cursor = Some(cursor);
        self
    }
    pub fn stringify_ids(&mut self, stringify_ids: bool) -> &mut Self {
        self.optional_params.stringify_ids = Some(stringify_ids);
        self
    }
    pub fn count(&mut self, count: u64) -> &mut Self {
        self.optional_params.count = Some(count);
        self
    }
}

impl<'a> FollowersIdsRequest<'a> {
    pub async fn send(&self) -> Result<FollowersIdsResult, Error> {
        let endpoint = "https://api.twitter.com/1.1/followers/ids.json";

        let mut params: HashMap<&str, String> = hashmap! {};
        if let Some(user_id) = self.optional_params.user_id {
            params.insert("user_id", user_id.to_string());
        }
        if let Some(screen_name) = self.optional_params.screen_name.clone() {
            params.insert("screen_name", screen_name);
        }
        if let Some(cursor) = self.optional_params.cursor {
            params.insert("cursor", cursor.to_string());
        }
        if let Some(stringify_ids) = self.optional_params.stringify_ids {
            params.insert("stringify_ids", stringify_ids.to_string());
        }
        if let Some(count) = self.optional_params.count {
            params.insert("count", count.to_string());
        }

        self.api.raw_get(endpoint, &params).await
    }
}
