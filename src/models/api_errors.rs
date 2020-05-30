pub struct APIError {
    errors: Vec<ErrorObject>,
}

pub struct ErrorObject {
    code: i64,
    message: String,
}
