use rocket::request::{Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket::{get, post};
// use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;
// use std::fs;
// use std::path::Path;
// use wmnet::network::*;
// use wmnet::util::name_from_path;
use wmnet::wifi::Wpa;
// use wmnet::Result;
use wmnet::Workspace;

#[derive(Debug, FromForm, Serialize)]
pub struct Credent {
    pub ssid: String,
    pub key: String,
    pub iface: String,
}

#[derive(Serialize)]
pub struct TeraContext {
    ssid: String,
    key: String,
    iface: String,
    interfaces: Vec<String>,
}

#[get("/")]
pub fn index(state: State<Workspace>) -> Template {
    let wifi = state.wifi();
    let context = TeraContext {
        ssid: wifi.ssid(),
        key: wifi.key(),
        iface: wifi.interface(),
        interfaces: wifi.interfaces().unwrap_or(vec!["empty".to_owned()]),
    };
    Template::render("wifi", &context)
}

#[post("/", data = "<credent_form>")]
pub fn update(credent_form: Form<Credent>, state: State<Workspace>) -> Flash<Redirect> {
    let credent = credent_form.into_inner();
    println!("wpa_form:{:?}", credent);
    if credent.ssid.is_empty() {
        Flash::error(Redirect::to("/wifi"), "uuid cannot be empty.")
    } else if credent.key.is_empty() {
        Flash::error(Redirect::to("/wifi"), "password cannot be empty.")
    } else if let Err(e) = state.wifi().change_interface(&credent.iface) {
        Flash::error(
            Redirect::to("/wifi"),
            format!("change interface error {}", e),
        )
    } else if let Err(e) = state.wifi().credentials(&credent.ssid, &credent.key) {
        Flash::error(
            Redirect::to("/wifi"),
            format!("update credentials error {}", e),
        )
    } else if let Err(e) = state.wifi().connect() {
        Flash::error(Redirect::to("/wifi"), format!("wifi connect error {}", e))
    } else {
        Flash::success(
            Redirect::to("/wifi"),
            "WPA credential successfully changed.",
        )
    }
}

// #[get("/wpa", format = "json")]
// pub fn get(state: State<Workspace>) -> Json<Wpa> {
//     let net = state.network();
//     Json(Wpa {
//         uuid: net.wpassid(),
//         pass: net.wpapass(),
//     })
// }
