#![feature(proc_macro_hygiene, decl_macro, never_type)]

mod catch;
mod gpio;
// mod graphql;
// mod chan;
mod dashboard;
mod history;
mod setting;
mod wifi;
use rocket_prometheus::PrometheusMetrics;

// use std::fs::File;
// use std::io::LineWriter;
// use std::io;;
// use std::io::prelude::*;
use wmnet::Result;
// use std::path::{PathBuf,Path};
/// serve
// pub fn serve(args: &Args, opt: &Serve) -> Result<()> {
// let wms = automata::open();
// serve::start(wms);
// Ok(())
// }
// use db::SqliteConn;
// use rocket::request::Request;
// use rocket::response::content;
use rocket::{catchers, routes};
use rocket_contrib::{serve::StaticFiles, templates::Template};
// use wmnet::cli::*;
// use tera::Context;

// use rocket_contrib::{templates::Template, serve::StaticFiles};
// use rocket::State;
// use serde_json::json;
// use serde::{Serialize};
// use rocket_include_tera::{tera_resources_initialize, tera_response, TeraResponse};

fn main() -> Result<()> {
    let wms = wmnet::wms::setup()?;
    let prometheus = PrometheusMetrics::new();
    log::info!("✨ May your hopes and dreams become reality ✨");
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![dashboard::index])
        .mount("/wifi", routes![wifi::index, wifi::update])
        .mount("/setting", routes![setting::index, wifi::update])
        .mount("/history", routes![history::index])
        .mount("/gpio", routes![gpio::index])
        .mount("/metrics", prometheus)
        // .mount("/api/chan", routes![chan::list, chan::get])
        // .mount("/api/gpio", routes![gpio::status])
        // .mount("/api/msg", routes![msg::get, msg::create])
        .attach(Template::fairing())
        .manage(wms)
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
