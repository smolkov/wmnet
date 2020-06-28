// use rocket::request::{Form, FromForm};
// use rocket::response::{Flash, Redirect};
// use rocket::State;
use rocket::get;
// use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;
// use std::fs;
// use std::path::PathBuf;
// use wqms::network::*;
// use wqms::Result;
// use wqms::Workspace;

#[derive(Serialize)]
pub struct TeraContext {
    name: String,
}

#[get("/")]
pub fn index() -> Template {
    let context = TeraContext {
        name: "empty".to_owned(),
    };
    Template::render("setting", &context)
}
