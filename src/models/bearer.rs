use serde_derive::*;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BearerToken {
    pub access_token: String,
    pub token_type: String,
}
