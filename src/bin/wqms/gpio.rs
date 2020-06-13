use rocket::{get, post};

#[get("/", format = "json")]
pub fn status() -> String {
    format!("Hello, year old name!")
}

#[get("/toggle/<pin>", format = "json")]
pub fn toggle(pin: u64) -> String {
    format!("Toggle,pin {}!", pin)
}

// #[post("/<name>")]
// pub fn set(name: String) -> String {
// format!("Hello, {} year old named ", name)
// }
