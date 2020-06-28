use async_std::task;
use serde::{Deserialize, Serialize};
use tide::prelude::*;
use tide::{Request, Response, StatusCode};
use wqms::wifi::Wpa;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credent {
    pub ssid: String,
    pub key: String,
    pub iface: String,
}

pub fn api() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").post(|mut req: Request<_>| async move {
        let credent: Credent = req.body_json().await?;
        let wifi = wqms::ws::open().wifi();
        if let Err(_) = wifi.change_interface(&credent.iface) {
            Ok(Response::new(404))
        } else if let Err(_) = wifi.credentials(&credent.ssid, &credent.key) {
            Ok(Response::new(400))
        } else if let Err(_) = wifi.connect() {
            Ok(Response::new(400))
        } else {
            Ok(Response::new(200))
        }
    });
    api.at("/").get(|_| async {
        let wifi = wqms::ws::open().wifi();
        Ok(json!({
            "ssid":wifi.ssid(),
            "key":wifi.key(),
            "iface": wifi.interface(),
        }))
    });
    api
}
