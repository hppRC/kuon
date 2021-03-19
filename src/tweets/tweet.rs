use crate::{Error, Tweet, TwitterAPI};
use anyhow::Result;
use maplit::hashmap;

#[derive(Clone, Debug)]
pub struct TweetRequest<'a, Status> {
    api: &'a TwitterAPI,
    required_params: TweetRequestRequiredParams<Status>,
    optional_params: TweetRequestOptionalParams,
}

#[derive(Clone, Debug, Default)]
pub struct TweetRequestRequiredParams<Status> {
    status: Status,
}
#[derive(Clone, Debug, Default)]
pub struct TweetRequestOptionalParams {
    in_reply_to_status_id: Option<u64>,
    auto_populate_reply_metadata: Option<bool>,
    exclude_reply_user_ids: Option<Vec<u64>>,
    attachment_url: Option<String>,
    media_ids: Option<Vec<u64>>,
    possibly_sensitive: Option<bool>,
    lat: Option<f64>,
    long: Option<f64>,
    place_id: Option<String>,
    display_coordinates: Option<bool>,
    trim_user: Option<bool>,
    enable_dmcommands: Option<bool>,
    fail_dmcommands: Option<bool>,
    card_uri: Option<String>,
}

impl TwitterAPI {
    /// # Example
    /// ```
    /// # use anyhow::Result;
    /// # async fn doc() -> Result<()> {
    ///
    /// let api = kuon::TwitterAPI::new_using_env().await?;
    /// let res = api.tweet().status("this is test tweet from kuon.").send().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tweet(&self) -> TweetRequest<()> {
        TweetRequest {
            api: self,
            required_params: Default::default(),
            optional_params: Default::default(),
        }
    }
}

impl<'a> TweetRequest<'a, ()> {
    pub fn status<Status>(&self, status: Status) -> TweetRequest<'a, Status>
    where
        Status: ToString,
    {
        TweetRequest {
            api: self.api,
            required_params: TweetRequestRequiredParams { status },
            optional_params: self.optional_params.clone(),
        }
    }
}

impl<'a, Status> TweetRequest<'a, Status> {
    pub fn in_reply_to_status_id(&mut self, in_reply_to_status_id: u64) -> &mut Self {
        self.optional_params.in_reply_to_status_id = Some(in_reply_to_status_id);
        self
    }
    pub fn auto_populate_reply_metadata(
        &mut self,
        auto_populate_reply_metadata: bool,
    ) -> &mut Self {
        self.optional_params.auto_populate_reply_metadata = Some(auto_populate_reply_metadata);
        self
    }
    pub fn exclude_reply_user_ids(&mut self, exclude_reply_user_ids: Vec<u64>) -> &mut Self {
        self.optional_params.exclude_reply_user_ids = Some(exclude_reply_user_ids);
        self
    }
    pub fn attachment_url(&mut self, attachment_url: String) -> &mut Self {
        self.optional_params.attachment_url = Some(attachment_url);
        self
    }
    pub fn media_ids(&mut self, media_ids: Vec<u64>) -> &mut Self {
        self.optional_params.media_ids = Some(media_ids);
        self
    }
    pub fn possibly_sensitive(&mut self, possibly_sensitive: bool) -> &mut Self {
        self.optional_params.possibly_sensitive = Some(possibly_sensitive);
        self
    }
    pub fn lat(&mut self, lat: f64) -> &mut Self {
        self.optional_params.lat = Some(lat);
        self
    }
    pub fn long(&mut self, long: f64) -> &mut Self {
        self.optional_params.long = Some(long);
        self
    }
    pub fn place_id(&mut self, place_id: impl Into<String>) -> &mut Self {
        self.optional_params.place_id = Some(place_id.into());
        self
    }
    pub fn display_coordinates(&mut self, display_coordinates: bool) -> &mut Self {
        self.optional_params.display_coordinates = Some(display_coordinates);
        self
    }
    pub fn trim_user(&mut self, trim_user: bool) -> &mut Self {
        self.optional_params.trim_user = Some(trim_user);
        self
    }
    pub fn enable_dmcommands(&mut self, enable_dmcommands: bool) -> &mut Self {
        self.optional_params.enable_dmcommands = Some(enable_dmcommands);
        self
    }
    pub fn fail_dmcommands(&mut self, fail_dmcommands: bool) -> &mut Self {
        self.optional_params.fail_dmcommands = Some(fail_dmcommands);
        self
    }
    pub fn card_uri(&mut self, card_uri: impl Into<String>) -> &mut Self {
        self.optional_params.card_uri = Some(card_uri.into());
        self
    }
}

impl<'a, Status> TweetRequest<'a, Status>
where
    Status: ToString,
{
    pub async fn send(&self) -> Result<Tweet, Error> {
        let endpoint = "https://api.twitter.com/1.1/statuses/update.json";
        let mut params = hashmap! {"status" => self.required_params.status.to_string()};

        if let Some(in_reply_to_status_id) = self.optional_params.in_reply_to_status_id {
            params.insert("in_reply_to_status_id", in_reply_to_status_id.to_string());
        }
        if let Some(auto_populate_reply_metadata) =
            self.optional_params.auto_populate_reply_metadata
        {
            params.insert(
                "auto_populate_reply_metadata",
                auto_populate_reply_metadata.to_string(),
            );
        }
        if let Some(exclude_reply_user_ids) = self.optional_params.exclude_reply_user_ids.clone() {
            params.insert(
                "exclude_reply_user_ids",
                exclude_reply_user_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(attachment_url) = self.optional_params.attachment_url.clone() {
            params.insert("attachment_url", attachment_url.to_string());
        }
        if let Some(media_ids) = self.optional_params.media_ids.clone() {
            params.insert(
                "media_ids",
                media_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(possibly_sensitive) = self.optional_params.possibly_sensitive {
            params.insert("possibly_sensitive", possibly_sensitive.to_string());
        }
        if let Some(lat) = self.optional_params.lat {
            params.insert("lat", lat.to_string());
        }
        if let Some(long) = self.optional_params.long {
            params.insert("long", long.to_string());
        }
        if let Some(place_id) = self.optional_params.place_id.clone() {
            params.insert("place_id", place_id);
        }
        if let Some(display_coordinates) = self.optional_params.display_coordinates {
            params.insert("display_coordinates", display_coordinates.to_string());
        }
        if let Some(trim_user) = self.optional_params.trim_user {
            params.insert("trim_user", trim_user.to_string());
        }
        if let Some(enable_dmcommands) = self.optional_params.enable_dmcommands {
            params.insert("enable_dmcommands", enable_dmcommands.to_string());
        }
        if let Some(fail_dmcommands) = self.optional_params.fail_dmcommands {
            params.insert("fail_dmcommands", fail_dmcommands.to_string());
        }
        if let Some(card_uri) = self.optional_params.card_uri.clone() {
            params.insert("card_uri", card_uri);
        }

        self.api.raw_post(endpoint, &params).await
    }
}
