// use rocket::request::FlashMessage;
use rocket::{get, post};
use rocket_contrib::json::Json;
// use serde::Serialize;
use rocket::State;
// use serde::{Deserialize, Serialize};
use wmnet::channel::*;
use wmnet::Workspace;

#[get("/")]
pub fn list(state: State<Workspace>) -> Json<Vec<ChanInfo>> {
    let list = state.channels().list_info().unwrap();
    Json(list)
}
#[get("/<name>", format = "json")]
pub fn get(name: String, state: State<Workspace>) -> Option<Json<ChanInfo>> {
    if let Some(ch) = state.channels().get(name.as_str()) {
        Some(Json(ch.info()))
    } else {
        None
    }
}
