use rocket::request::{Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket::{get, post};
use rocket_contrib::json::Json;
use serde::Serialize;
use wqms::network::*;
use wqms::Workspace;

#[derive(Debug, FromForm, Serialize)]
pub struct Credentials {
    pub uuid: String,
    pub pass: String,
}

#[get("/", format = "json")]
pub fn status(state: State<Workspace>) -> String {
    let net = state.network();
    format!("NET:{}", net.state())
}

#[post("/wpa", data = "<credential_form>")]
pub fn wpa_new(credential_form: Form<Credentials>, state: State<Workspace>) -> Flash<Redirect> {
    let credential = credential_form.into_inner();
    if credential.uuid.is_empty() {
        Flash::error(Redirect::to("/"), "uuid cannot be empty.")
    } else if credential.pass.is_empty() {
        Flash::error(Redirect::to("/"), "password cannot be empty.")
    } else {
        if let Err(e) = state
            .network()
            .credentials(credential.uuid.as_str(), credential.pass.as_str())
        {
            Flash::error(
                Redirect::to("/"),
                "Todo could not be inserted due an internal error.",
            )
        } else {
            Flash::success(Redirect::to("/"), "Todo successfully added.")
        }
    }
}

#[get("/wpa", format = "json")]
pub fn wpa(state: State<Workspace>) -> Json<Credentials> {
    let net = state.network();
    Json(Credentials {
        uuid: net.wpassid(),
        pass: net.wpapass(),
    })
}
