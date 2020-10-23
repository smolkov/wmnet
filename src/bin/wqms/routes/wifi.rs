// use tide::{Body,Response};
// use crate::models::Prop;
use handlebars::to_json;
use serde_json::value::Map;
use tide_handlebars::prelude::*;
// use wqms::wifi::Wlan;

pub async fn index(req: crate::Request) -> tide::Result {
    log::info!("Tracing Started");
    let state = &req.state();
    let nets = wqms::wifi::scan();
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Wifi setting"));
    data.insert("parent".to_string(), to_json("layouts/main"));

    data.insert("nets".to_string(), to_json(&nets));
    Ok(hb.render_response_ext("wifi/list", &data, "html")?)
}

pub async fn edit(req: crate::Request) -> tide::Result {
    let state = &req.state();
    let id = req.param::<String>("ssid")?;

    // let wlans = wqms::wifi::scan();
    // let doc = City::find_one(&db.clone(), filter, None).await?;
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Wifi"));
    data.insert("parent".to_string(), to_json("layouts/main"));
    data.insert(
        "action".to_string(),
        to_json(format!("/wifi/{}/conect", id)),
    );
    // data.insert("prop".to_string(), to_json(&wlans));
    Ok(hb.render_response_ext("wifi/form", &data, "html")?)
}

pub async fn update(_req: crate::Request) -> tide::Result {
    // let mut city: City = req.body_form().await?;
    // let state = &req.state();
    // let db = &state.client.database("test");
    // let id = req.param::<String>("city_id")?;
    // city.id = Some(ObjectId::with_string(&id).unwrap());

    // city.save(&db.clone(), None).await?;

    // let city_id = city.id.unwrap();

    Ok(tide::Redirect::new(format!("/wifi")).into())
}