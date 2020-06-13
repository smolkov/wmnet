


use rocket::{get,post};


#[get("/")]
pub fn list(name: String) -> String {
    format!("Hello, {} year old name!", name)
}

#[get("/<name>")]
pub fn get(name: String) -> String {
    format!("Hello, {} year old name!", name)
}

#[post("/<name>")]
pub fn create(name: String) -> String {
    format!("Hello, {} year old named ",name) 
}