use crate::{Error, SearchResult, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;

#[derive(Clone, Debug)]
pub struct SearchTweetsRequest<'a, Q> {
    api: &'a TwitterAPI,
    required_params: SearchTweetsRequestRequiredParams<Q>,
    optional_params: SearchTweetsRequestOptionalParams,
}

#[derive(Clone, Debug, Default)]
pub struct SearchTweetsRequestRequiredParams<Q> {
    q: Q,
}
#[derive(Clone, Debug, Default)]
pub struct SearchTweetsRequestOptionalParams {
    geocode: Option<String>,
    lang: Option<String>,
    locale: Option<String>,
    result_type: Option<String>,
    count: Option<u64>,
    until: Option<String>,
    since_id: Option<u64>,
    max_id: Option<u64>,
    include_entities: Option<bool>,
}

impl TwitterAPI {
    pub fn search_tweets(&self) -> SearchTweetsRequest<()> {
        SearchTweetsRequest {
            api: self,
            required_params: Default::default(),
            optional_params: Default::default(),
        }
    }
}

impl<'a> SearchTweetsRequest<'a, ()> {
    pub fn q<Q>(&self, q: Q) -> SearchTweetsRequest<'a, Q>
    where
        Q: ToString,
    {
        SearchTweetsRequest {
            api: self.api,
            required_params: SearchTweetsRequestRequiredParams { q },
            optional_params: self.optional_params.clone(),
        }
    }
}

impl<'a, Q> SearchTweetsRequest<'a, Q> {
    pub fn geocode(&mut self, geocode: impl Into<String>) -> &mut SearchTweetsRequest<'a, Q> {
        self.optional_params.geocode = Some(geocode.into());
        self
    }
    pub fn locale(&mut self, locale: impl Into<String>) -> &mut SearchTweetsRequest<'a, Q> {
        self.optional_params.locale = Some(locale.into());
        self
    }
    pub fn result_type(
        &mut self,
        result_type: impl Into<String>,
    ) -> &mut SearchTweetsRequest<'a, Q> {
        self.optional_params.result_type = Some(result_type.into());
        self
    }
    pub fn count(&mut self, count: u64) -> &mut SearchTweetsRequest<'a, Q> {
        self.optional_params.count = Some(count);
        self
    }
    pub fn until(&mut self, until: impl Into<String>) -> &mut SearchTweetsRequest<'a, Q> {
        self.optional_params.until = Some(until.into());
        self
    }
    pub fn since_id(&mut self, since_id: u64) -> &mut SearchTweetsRequest<'a, Q> {
        self.optional_params.since_id = Some(since_id);
        self
    }
    pub fn max_id(&mut self, max_id: u64) -> &mut SearchTweetsRequest<'a, Q> {
        self.optional_params.max_id = Some(max_id);
        self
    }
    pub fn include_entities(&mut self, include_entities: bool) -> &mut SearchTweetsRequest<'a, Q> {
        self.optional_params.include_entities = Some(include_entities.into());
        self
    }
}

impl<'a, Q> SearchTweetsRequest<'a, Q>
where
    Q: ToString,
{
    pub async fn send(&self) -> Result<SearchResult, Error> {
        let endpoint = "https://api.twitter.com/1.1/search/tweets.json";
        let mut params = hashmap! {"q" => self.required_params.q.to_string()};

        if let Some(lang) = self.optional_params.lang.clone() {
            params.insert("lang", lang.to_string());
        }
        if let Some(locale) = self.optional_params.locale.clone() {
            params.insert("locale", locale.to_string());
        }
        if let Some(result_type) = self.optional_params.result_type.clone() {
            params.insert("result_type", result_type.to_string());
        }
        if let Some(count) = self.optional_params.count.clone() {
            params.insert("count", count.to_string());
        }
        if let Some(until) = self.optional_params.until.clone() {
            params.insert("until", until.to_string());
        }
        if let Some(since_id) = self.optional_params.since_id.clone() {
            params.insert("since_id", since_id.to_string());
        }
        if let Some(max_id) = self.optional_params.max_id.clone() {
            params.insert("max_id", max_id.to_string());
        }
        if let Some(include_entities) = self.optional_params.include_entities.clone() {
            params.insert("include_entities", include_entities.to_string());
        }

        self.api.raw_get(endpoint, &params).await
    }
}
