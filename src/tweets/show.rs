use crate::{Error, Tweet, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;

impl TwitterAPI {
    /// # Example
    pub async fn show(self, id: u64) -> Result<Tweet, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/show.json";
        let id_str: &str = &id.to_string();
        let params = hashmap! { "id" => id_str };

        self.raw_get(endpoint, &params).await
    }
}
