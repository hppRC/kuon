use crate::{Error, Tweet, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;

#[derive(Clone, Debug)]
pub struct HomeTimelineRequest<'a> {
    api: &'a TwitterAPI,
    optional_params: HomeTimelineRequestOptionalParams,
}

#[derive(Clone, Debug, Default)]
struct HomeTimelineRequestOptionalParams {
    count: Option<u64>,
    since_id: Option<u64>,
    max_id: Option<u64>,
    trim_user: Option<bool>,
    exclude_replies: Option<bool>,
    include_entities: Option<bool>,
}

impl TwitterAPI {
    pub fn home_timeline(&self) -> HomeTimelineRequest {
        HomeTimelineRequest {
            api: self,
            optional_params: Default::default(),
        }
    }
}

impl<'a> HomeTimelineRequest<'a> {
    pub fn count(&mut self, count: u64) -> &mut Self {
        self.optional_params.count = Some(count);
        self
    }
    pub fn since_id(&mut self, since_id: u64) -> &mut Self {
        self.optional_params.since_id = Some(since_id);
        self
    }
    pub fn max_id(&mut self, max_id: u64) -> &mut Self {
        self.optional_params.max_id = Some(max_id);
        self
    }
    pub fn trim_user(&mut self, trim_user: bool) -> &mut Self {
        self.optional_params.trim_user = Some(trim_user);
        self
    }
    pub fn exclude_replies(&mut self, exclude_replies: bool) -> &mut Self {
        self.optional_params.exclude_replies = Some(exclude_replies);
        self
    }
    pub fn include_entities(&mut self, include_entities: bool) -> &mut Self {
        self.optional_params.include_entities = Some(include_entities);
        self
    }
}

impl<'a> HomeTimelineRequest<'a> {
    pub async fn send(&self) -> Result<Tweet, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let mut params = hashmap! {};

        if let Some(count) = self.optional_params.count {
            params.insert("count", count.to_string());
        }
        if let Some(since_id) = self.optional_params.since_id {
            params.insert("since_id", since_id.to_string());
        }
        if let Some(max_id) = self.optional_params.max_id {
            params.insert("max_id", max_id.to_string());
        }
        if let Some(trim_user) = self.optional_params.trim_user {
            params.insert("trim_user", trim_user.to_string());
        }
        if let Some(exclude_replies) = self.optional_params.exclude_replies {
            params.insert("exclude_replies", exclude_replies.to_string());
        }
        if let Some(include_entities) = self.optional_params.include_entities {
            params.insert("include_entities", include_entities.to_string());
        }

        self.api.raw_post(endpoint, &params).await
    }
}
