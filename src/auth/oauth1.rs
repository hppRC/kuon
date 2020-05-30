use chrono::Utc;
use std::collections::HashMap;

use crate::{TwitterAPI, FRAGMENT};
use percent_encoding::utf8_percent_encode;

impl TwitterAPI {
    pub fn encode(target: &str) -> percent_encoding::PercentEncode {
        utf8_percent_encode(&target, &FRAGMENT)
    }

    pub(crate) fn create_oauth_header(
        &self,
        endpoint: &str,
        params: &HashMap<&str, &str>,
    ) -> String {
        let oauth_nonce: &str = &format!("nonce{}", Utc::now().timestamp());
        let oauth_signature_method: &str = "HMAC-SHA1";
        let oauth_timestamp: &str = &format!("{}", Utc::now().timestamp());
        let oauth_version: &str = "1.0";

        let mut map: HashMap<&str, &str> = maplit::hashmap! {
            "oauth_nonce" => oauth_nonce,
            "oauth_version" => oauth_version,
            "oauth_timestamp" => oauth_timestamp,
            "oauth_consumer_key" => &self.api_key,
            "oauth_token" => &self.access_token,
            "oauth_signature_method" => oauth_signature_method,
        };

        for (k, v) in params {
            map.insert(k, v);
        }

        let oauth_signature: &str = &self.create_oauth_signature("POST", &endpoint, &map);

        format!(
            "OAuth oauth_consumer_key=\"{}\", oauth_nonce=\"{}\", oauth_signature=\"{}\", oauth_signature_method=\"{}\", oauth_timestamp=\"{}\", oauth_token=\"{}\", oauth_version=\"{}\"",
            Self::encode(&self.api_key),
            Self::encode(oauth_nonce),
            Self::encode(oauth_signature),
            Self::encode(oauth_signature_method),
            Self::encode(oauth_timestamp),
            Self::encode(&self.access_token),
            Self::encode(oauth_version),
        )
    }

    pub(crate) fn create_oauth_signature(
        &self,
        http_method: &str,
        endpoint: &str,
        params: &HashMap<&str, &str>,
    ) -> String {
        //create oauth key
        let cs_encoded = Self::encode(&self.api_secret_key);
        let ts_encoded = Self::encode(&self.access_token_secret);
        let key: String = format!("{}&{}", cs_encoded, ts_encoded);

        // sort for oauth
        let mut params: Vec<(&str, &str)> = params.iter().map(|(&k, &v)| (k, v)).collect();
        params.sort();

        let param = params
            .into_iter()
            .map(|(k, v)| format!("{}={}", Self::encode(k), Self::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        let data = format!(
            "{}&{}&{}",
            Self::encode(http_method),
            Self::encode(endpoint),
            Self::encode(&param)
        );

        let hash = hmacsha1::hmac_sha1(key.as_bytes(), data.as_bytes());
        base64::encode(&hash)
    }
}
