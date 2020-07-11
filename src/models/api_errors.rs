use serde_derive::*;

#[derive(Deserialize, Clone, Debug)]
pub struct APIError {
    errors: Vec<ErrorObject>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ErrorObject {
    code: i64,
    message: String,
}

impl std::convert::From<APIError> for anyhow::Error {
    fn from(error: APIError) -> Self {
        anyhow::anyhow!(error)
    }
}
