use crate::{Error, TrimTweet, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;

#[derive(Clone, Debug)]
pub struct UserTimelineRequest<'a> {
    api: &'a TwitterAPI,
    optional_params: UserTimelineRequestOptionalParams,
}

#[derive(Clone, Debug, Default)]
struct UserTimelineRequestOptionalParams {
    user_id: Option<u64>,
    screen_name: Option<String>,
    count: Option<u64>,
    since_id: Option<u64>,
    max_id: Option<u64>,
    trim_user: Option<bool>,
    exclude_replies: Option<bool>,
    include_rts: Option<bool>,
}

impl TwitterAPI {
    pub fn user_timeline(&self) -> UserTimelineRequest {
        UserTimelineRequest {
            api: self,
            optional_params: Default::default(),
        }
    }
}

impl<'a> UserTimelineRequest<'a> {
    pub fn user_id(&mut self, user_id: u64) -> &mut Self {
        self.optional_params.user_id = Some(user_id);
        self
    }
    pub fn screen_name(&mut self, screen_name: impl Into<String>) -> &mut Self {
        self.optional_params.screen_name = Some(screen_name.into());
        self
    }
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
    pub fn include_rts(&mut self, include_rts: bool) -> &mut Self {
        self.optional_params.include_rts = Some(include_rts);
        self
    }
}

impl<'a> UserTimelineRequest<'a> {
    pub async fn send(&self) -> Result<Vec<TrimTweet>, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/home_timeline.json";
        let mut params = hashmap! {};

        if let Some(user_id) = self.optional_params.user_id {
            params.insert("user_id", user_id.to_string());
        }
        if let Some(screen_name) = self.optional_params.screen_name.clone() {
            params.insert("screen_name", screen_name.to_string());
        }
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
        if let Some(include_rts) = self.optional_params.include_rts {
            params.insert("include_rts", include_rts.to_string());
        }

        self.api.raw_get(endpoint, &params).await
    }
}
