use crate::{Error, FavoriteResult, TwitterAPI};
use anyhow::Result;

use maplit::hashmap;
#[derive(Clone, Debug)]
pub struct FavoriteRequest<'a, Id> {
    api: &'a TwitterAPI,
    required_params: FavoriteRequestRequiredParams<Id>,
    optional_params: FavoriteRequestOptionalParams,
}

#[derive(Clone, Debug, Default)]
pub struct FavoriteRequestRequiredParams<Id> {
    id: Id,
}
#[derive(Clone, Debug, Default)]
pub struct FavoriteRequestOptionalParams {
    include_entities: Option<bool>,
}

impl TwitterAPI {
    pub fn favorite(&self) -> FavoriteRequest<()> {
        FavoriteRequest {
            api: self,
            required_params: Default::default(),
            optional_params: Default::default(),
        }
    }
}

impl<'a> FavoriteRequest<'a, ()> {
    pub fn id<Id>(&self, id: Id) -> FavoriteRequest<'a, Id>
    where
        Id: ToString,
    {
        FavoriteRequest {
            api: self.api,
            required_params: FavoriteRequestRequiredParams { id },
            optional_params: self.optional_params.clone(),
        }
    }
}

impl<'a, Id> FavoriteRequest<'a, Id> {
    pub fn include_entities(&mut self, include_entities: bool) -> &mut Self {
        self.optional_params.include_entities = Some(include_entities);
        self
    }
}

impl<'a, Id> FavoriteRequest<'a, Id>
where
    Id: ToString,
{
    pub async fn send(&self) -> Result<FavoriteResult, Error> {
        let endpoint = "https://api.twitter.com/1.1/favorites/create.json";
        let mut params = hashmap! {"id" => self.required_params.id.to_string()};
        if let Some(include_entities) = self.optional_params.include_entities {
            params.insert("include_entities", include_entities.to_string());
        }

        self.api.raw_post(endpoint, &params).await
    }
}
