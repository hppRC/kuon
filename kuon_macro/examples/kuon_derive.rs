use kuon_macro::RequestBuilder;

#[derive(RequestBuilder)]
pub struct HogeRequest {
    huga: String,
    foo: Option<String>,
}

fn main() {}
