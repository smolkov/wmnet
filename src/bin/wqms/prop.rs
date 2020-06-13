// use rocket::request::FlashMessage;
use rocket::{get, put};
// use rocket_contrib::json::Json;
// use serde::Serialize;
use rocket::State;
// use serde::{Deserialize, Serialize};
use wqms::iface::*;
use wqms::Workspace;

#[get("/", format = "json")]
pub fn list(state: State<Workspace>) -> String {
    format!("list of prop")
}

#[put("/<name>", format = "json", data = "<value>")]
pub fn set(name: String, value: String, state: State<Workspace>) -> String {
    state.set(&name, &value);
    format!("set prop:{}", name)
}
#[get("/<name>", format = "json")]
pub fn get(name: String, state: State<Workspace>) -> String {
    state
        .get(&name)
        .unwrap_or(format!("property {} not fount", name))
}
