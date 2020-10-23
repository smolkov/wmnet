use rocket::get;
// use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
pub struct TeraContext {
    gpios: Vec<String>,
}

#[get("/")]
pub fn index() -> Template {
    let context = TeraContext {
        gpios: vec!["gpio1".to_owned(), "gpio2".to_owned()],
    };
    Template::render("gpio", &context)
}

// #[get("/toggle/<pin>", format = "json")]
// pub fn toggle(pin: u64) -> String {
// format!("Toggle,pin {}!", pin)
// }

// #[post("/<name>")]
// pub fn set(name: String) -> String {
// format!("Hello, {} year old named ", name)
// }
