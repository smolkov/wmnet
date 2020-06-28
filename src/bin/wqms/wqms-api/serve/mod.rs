mod msg;
mod gpio;
mod config;
mod catch;

// pub struct ServerConfig(rocket::config::Config);


use std::collections::HashMap;
// use rocket::request::Request;
// use rocket::response::content;
use rocket::{get,routes,catchers};
use rocket_contrib::{templates::Template, serve::StaticFiles};
use rocket::{State};
use serde::{Serialize};
use log::info;
use crate::Workspace;



#[get("/")]
fn index(state: State<'_, Workspace>) -> Template {
    let mut map = HashMap::new();
    map.insert("name", "Title");
    map.insert("body", "Hello, world!");
    Template::render("index",&map)
}

#[get("/gpio")]
fn gpio(state: State<'_, Workspace>) -> Template {
    let mut map = HashMap::new();
    map.insert("name", "Title");
    map.insert("body", "Hello, world!");
    Template::render("gpio",&map)
}



pub fn start( ws:Workspace ) {
    info!("✨ May your hopes and dreams become reality ✨");
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index,gpio])
        .mount("/api/gpio",routes![ gpio::status, gpio::new])
        .mount("/api/msg",routes![msg::get,msg::create ])
        .attach(Template::fairing())
        .manage(ws)
        .register(catchers![ catch::not_found, catch::internal_server_error, catch::unprocessable_entity ]).
        launch();
}

#[cfg(test)]
mod test {
    // use super::rocket;
    // use rocket::local::Client;
    // use rocket::http::Status;

    // #[test]
    // fn test_hello() {
        // let client = Client::new(rocket()).unwrap();
        // let mut response = client.get("/").dispatch();
        // assert_eq!(response.status(), Status::Ok);
        // assert_eq!(response.body_string(), Some("Hello, world!".into()));
    // }
}