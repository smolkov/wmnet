#![feature(proc_macro_hygiene, decl_macro, never_type)]

mod catch;
mod gpio;
// mod graphql;
mod chan;
mod db;
mod msg;
mod net;
mod prop;
mod web;
// use std::fs::File;
// use std::io::LineWriter;
// use std::io;;
// use std::io::prelude::*;
use wqms::Result;
// use std::path::{PathBuf,Path};
/// serve
// pub fn serve(args: &Args, opt: &Serve) -> Result<()> {
// let ws = automata::open();
// serve::start(ws);
// Ok(())
// }
// use db::SqliteConn;
// use rocket::request::Request;
// use rocket::response::content;
use rocket::{catchers, routes};
use rocket_contrib::{serve::StaticFiles, templates::Template};
// use wqms::cli::*;
// use tera::Context;

// use rocket_contrib::{templates::Template, serve::StaticFiles};
// use rocket::State;
// use serde_json::json;
// use serde::{Serialize};
// use rocket_include_tera::{tera_resources_initialize, tera_response, TeraResponse};

fn main() -> Result<()> {
    let ws = wqms::ws::setup()?;
    let web = ws.web();
    log::info!("✨ May your hopes and dreams become reality ✨");
    rocket::ignite()
        .mount("/", StaticFiles::from(web.www().as_path()))
        // .mount("/", routes![web::index])
        // .mount("/tera", StaticFiles::from("templates/tera/"))
        .mount("/api/chan", routes![chan::list, chan::get])
        .mount("/api/prop", routes![prop::list, prop::set, prop::get])
        .mount("/api/net", routes![net::status, net::wpa_new, net::wpa])
        .mount("/api/gpio", routes![gpio::status])
        .mount("/api/msg", routes![msg::get, msg::create])
        .attach(Template::fairing())
        .manage(ws)
        .register(catchers![
            catch::not_found,
            catch::internal_server_error,
            catch::unprocessable_entity
        ])
        .launch();
    // println!("");
    // match args.commands() {
    // Cmd::Init(opt) => setup(&args, &opt)?,
    // Cmd::Serve(opt) => serve(&args, &opt)?,
    // };
    Ok(())
}
