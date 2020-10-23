use rocket::request::Request;
use rocket::response::content;
use rocket::{catch};
use serde_json::json;


#[catch(404)]
pub fn not_found(req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":
                format!(
                    "Look elsewhere, perhaps? No matching route for uri={}",
                    req.uri()
                )
        })
        .to_string(),
    )
}

#[catch(500)]
pub fn internal_server_error(_req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":"Internal server error 🤖"
        })
        .to_string(),
    )
}

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":"The request was well-formed but was unable to be followed due to semantic errors."
        })
        .to_string(),
    )
}
