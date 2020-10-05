pub mod prop;
pub mod channel;
pub mod wifi;

use handlebars::to_json;
use serde_json::value::Map;
use tide::{http::mime, Response};

pub async fn welcome(req: crate::Request) -> tide::Result<impl Into<Response>> {
    let state = &req.state();
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("name".to_string(), to_json("Tider!"));
    data.insert("parent".to_string(), to_json("layouts/main"));
    Ok(Response::builder(200)
        .body(hb.render("welcome", &data)?)
        .content_type(mime::HTML))
}