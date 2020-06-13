
use rocket::{get,post,put};
use rocket_contrib::json::{Json};
use crate::mio::{PinStatus};
use rocket::{State};
use serde_json::json;
use crate::Workspace;

/// Pin status
#[get("/<num>")]
pub fn status(num: usize,s: State<Workspace>) ->  Option<Json<PinStatus>> {
    Some(Json(PinStatus::default()))
}

/// Updade value
#[put("/<num>/<value>")]
pub fn update(num:u64,value:String,s: State<Workspace>) -> Option<Json<String>> {
    Some(json!({ "status": "ok" }))
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/<num>", format = "json", data = "<pin>")]
pub fn new(num: u64,pin: Json<PinStatus>,s: State<Workspace>) -> Json<String> {
    Json(json!({ "status": "ok" }))
}
