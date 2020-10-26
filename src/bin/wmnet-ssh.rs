// use std::{env, time::Duration};
// use tide::{sessions::SessionMiddleware, Redirect};

// pub mod records;
// mod templates;
// use tide_handlebars::prelude::*;
// use async_std::sync::Arc;
// use async_std::task;
// use std::io::Read;
use wmnet::Result;

// use ssh2::Session;
// use std::net::TcpStream;

#[async_std::main]
async fn main() -> Result<()> {
    wmnet::rpi::setup().await?;
    Ok(())
}
