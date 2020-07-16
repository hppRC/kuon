use serde_derive::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIError {
    errors: Vec<ErrorObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorObject {
    code: i64,
    message: String,
}

impl std::convert::From<APIError> for anyhow::Error {
    fn from(error: APIError) -> Self {
        anyhow::anyhow!(error)
    }
}
