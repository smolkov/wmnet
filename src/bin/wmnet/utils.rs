
#[allow(dead_code)]
pub(crate) async fn deserialize_body<T>(request: &mut crate::Request) -> tide::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    match request.content_type() {
        Some(c) if c == tide::http::mime::FORM => request.body_form().await,
        Some(c) if c == tide::http::mime::JSON => request.body_json().await,
        _ => Err(tide::Error::from_str(
            tide::StatusCode::NotAcceptable,
            "unrecognized content type",
        )),
    }
}

pub trait AsRoute {
    fn as_route(&self) -> std::borrow::Cow<str>;
}

impl AsRoute for str {
    fn as_route(&self) -> std::borrow::Cow<str> {
        self.into()
    }
}

impl AsRoute for String {
    fn as_route(&self) -> std::borrow::Cow<str> {
        self.into()
    }
}