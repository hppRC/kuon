#![feature(prelude_import)]
#![allow(dead_code)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use kuon::TwitterAPI;
use kuon_request_derive::KuonRequest;
pub struct TweetRequest<'a, Status, Hoge, Piyo> {
    api: &'a TwitterAPI,
    status: Status,
    hoge: Hoge,
    piyo: Piyo,
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
impl<'a, Status, Hoge, Piyo> TweetRequest<'a, Status, Hoge, Piyo> {
    fn to_hashmap(&self) -> std::collections::HashMap<&str, String> {
        let mut params = std::collections::HashMap::new();
        if let Some(ref in_reply_to_status_id) = self.in_reply_to_status_id {
            params.insert("in_reply_to_status_id", in_reply_to_status_id.to_string());
        }
        if let Some(ref auto_populate_reply_metadata) = self.auto_populate_reply_metadata {
            params.insert(
                "auto_populate_reply_metadata",
                auto_populate_reply_metadata.to_string(),
            );
        }
        if let Some(ref exclude_reply_user_ids) = self.exclude_reply_user_ids {
            params.insert(
                "exclude_reply_user_ids",
                exclude_reply_user_ids
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(ref attachment_url) = self.attachment_url {
            params.insert("attachment_url", attachment_url.to_string());
        }
        if let Some(ref media_ids) = self.media_ids {
            params.insert(
                "media_ids",
                media_ids
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(ref possibly_sensitive) = self.possibly_sensitive {
            params.insert("possibly_sensitive", possibly_sensitive.to_string());
        }
        if let Some(ref lat) = self.lat {
            params.insert("lat", lat.to_string());
        }
        if let Some(ref long) = self.long {
            params.insert("long", long.to_string());
        }
        if let Some(ref place_id) = self.place_id {
            params.insert("place_id", place_id.to_string());
        }
        if let Some(ref display_coordinates) = self.display_coordinates {
            params.insert("display_coordinates", display_coordinates.to_string());
        }
        if let Some(ref trim_user) = self.trim_user {
            params.insert("trim_user", trim_user.to_string());
        }
        if let Some(ref enable_dmcommands) = self.enable_dmcommands {
            params.insert("enable_dmcommands", enable_dmcommands.to_string());
        }
        if let Some(ref fail_dmcommands) = self.fail_dmcommands {
            params.insert("fail_dmcommands", fail_dmcommands.to_string());
        }
        if let Some(ref card_uri) = self.card_uri {
            params.insert("card_uri", card_uri.to_string());
        }
        params
    }
    pub fn in_reply_to_status_id(&mut self, in_reply_to_status_id: impl Into<u64>) -> &mut Self {
        self.in_reply_to_status_id = Some(in_reply_to_status_id.into());
        self
    }
    pub fn auto_populate_reply_metadata(
        &mut self,
        auto_populate_reply_metadata: impl Into<bool>,
    ) -> &mut Self {
        self.auto_populate_reply_metadata = Some(auto_populate_reply_metadata.into());
        self
    }
    pub fn exclude_reply_user_ids(
        &mut self,
        exclude_reply_user_ids: impl Into<Vec<u64>>,
    ) -> &mut Self {
        self.exclude_reply_user_ids = Some(exclude_reply_user_ids.into());
        self
    }
    pub fn attachment_url(&mut self, attachment_url: impl Into<String>) -> &mut Self {
        self.attachment_url = Some(attachment_url.into());
        self
    }
    pub fn media_ids(&mut self, media_ids: impl Into<Vec<u64>>) -> &mut Self {
        self.media_ids = Some(media_ids.into());
        self
    }
    pub fn possibly_sensitive(&mut self, possibly_sensitive: impl Into<bool>) -> &mut Self {
        self.possibly_sensitive = Some(possibly_sensitive.into());
        self
    }
    pub fn lat(&mut self, lat: impl Into<f64>) -> &mut Self {
        self.lat = Some(lat.into());
        self
    }
    pub fn long(&mut self, long: impl Into<f64>) -> &mut Self {
        self.long = Some(long.into());
        self
    }
    pub fn place_id(&mut self, place_id: impl Into<String>) -> &mut Self {
        self.place_id = Some(place_id.into());
        self
    }
    pub fn display_coordinates(&mut self, display_coordinates: impl Into<bool>) -> &mut Self {
        self.display_coordinates = Some(display_coordinates.into());
        self
    }
    pub fn trim_user(&mut self, trim_user: impl Into<bool>) -> &mut Self {
        self.trim_user = Some(trim_user.into());
        self
    }
    pub fn enable_dmcommands(&mut self, enable_dmcommands: impl Into<bool>) -> &mut Self {
        self.enable_dmcommands = Some(enable_dmcommands.into());
        self
    }
    pub fn fail_dmcommands(&mut self, fail_dmcommands: impl Into<bool>) -> &mut Self {
        self.fail_dmcommands = Some(fail_dmcommands.into());
        self
    }
    pub fn card_uri(&mut self, card_uri: impl Into<String>) -> &mut Self {
        self.card_uri = Some(card_uri.into());
        self
    }
}
fn main() {}
