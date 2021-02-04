use crate::{RetweetResult, TwitterAPI};
use anyhow::Result;

// TODO: Think adding a simple API module (kuon::simple::retweet)
impl TwitterAPI {
    pub async fn retweet(&self, id: &str) -> Result<RetweetResult> {
        let endpoint = &format!("https://api.twitter.com/1.1/statuses/retweet/{}.json", id);
        let params = maplit::hashmap! {};

        self.raw_post::<RetweetResult>(endpoint, &params).await
    }
}
