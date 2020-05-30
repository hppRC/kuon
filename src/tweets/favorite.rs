use crate::{FavoriteResult, TwitterAPI};
use anyhow::Result;

impl TwitterAPI {
    pub async fn favorite(&self, id: &str) -> Result<FavoriteResult> {
        let endpoint = "https://api.twitter.com/1.1/favorites/create.json";
        let params = maplit::hashmap! {
            "id" => id,
        };

        let favorite_results: FavoriteResult = self.post(endpoint, &params).await?;
        Ok(favorite_results)
    }
}
