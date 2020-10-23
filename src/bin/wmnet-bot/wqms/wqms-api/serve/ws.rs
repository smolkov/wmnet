use rocket::{get,post};



#[get("/list")]
pub fn status(name: String) -> String {
    format!("Hello, {} year old name!", name)
}

#[post("/<name>")]
pub fn set(name: String) -> String {
    format!("Hello, {} year old named ",name) 
}