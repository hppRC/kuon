#![allow(dead_code)]
use kuon::TwitterAPI;
use kuon_request_derive::KuonRequest;

#[derive(KuonRequest)]
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

fn main() {}
