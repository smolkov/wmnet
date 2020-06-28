use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Firebase {
    apikey: String,
}

/// firebase
pub fn firebase() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").get(|_| async { Ok("Hello, world") });
    api.at("/").post(|_| async { Ok("Goodbye, world") });
    api
}

/// statistic
pub fn statistic() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").get(|_| async { Ok("get statistic") });
    api.at("/").post(|_| async { Ok("post statistic") });
    api
}
