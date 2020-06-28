use rocket::get;
// use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
pub struct TeraContext {
    name: String,
}

#[get("/")]
pub fn index() -> Template {
    let context = TeraContext {
        name: "empty".to_owned(),
    };
    Template::render("history", &context)
}
