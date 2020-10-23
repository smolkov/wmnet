use tide::{Body,Response};
// use crate::models::Prop;
// use glob::glob;
// use wmnet::channel::Chan
// use async_std::stream::StreamExt;
use handlebars::to_json;
use serde_json::value::Map;
// use tide::Request;
use tide_handlebars::prelude::*;

pub async fn index(req: crate::Request) -> tide::Result {
    log::info!("Tracing Started");
    let state = &req.state();
    let chans = state.wms.channels().unwrap().infos().unwrap();
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Channels"));
    data.insert("parent".to_string(), to_json("layouts/main"));

    data.insert("channels".to_string(), to_json(&chans));
    Ok(hb.render_response_ext("channels/list", &data, "html")?)
}

pub async fn list(req: crate::Request) -> tide::Result {
    let state = &req.state();
    let chans = state.wms.channels().unwrap().infos().unwrap();
    Ok(Response::builder(tide::StatusCode::Ok)
    .body(Body::from_json(&chans)?)
    .build()) 
}
pub async fn set(req: crate::Request) -> tide::Result {
    // let prop = req.param("n")?;
    let prop = req.param::<String>("channel")?;
    Ok(Response::builder(tide::StatusCode::Ok)
    .body(Body::from_string(format!("Channel {} set!",prop)))
    .build()) 
}
pub async fn get(req: crate::Request) -> tide::Result {
    let prop = req.param::<String>("channel")?;
    Ok(Response::builder(tide::StatusCode::Ok)
    .body(Body::from_string(format!("Channel {} get!",prop)))
    .build()) 
}



pub async fn edit(req: crate::Request) -> tide::Result {
    let state = &req.state();
    let id = req.param::<String>("chan_id")?;

    let chan = state.wms.channels().unwrap().get_info(&id).unwrap();

    // let doc = City::find_one(&db.clone(), filter, None).await?;
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Channels"));
    data.insert("parent".to_string(), to_json("layouts/main"));
    data.insert(
        "action".to_string(),
        to_json(format!("/channels/{}/edit", id)),
    );
    data.insert("channel".to_string(), to_json(&chan));
    Ok(hb.render_response_ext("channels/form", &data, "html")?)
}

pub async fn update(_req: crate::Request) -> tide::Result {
    // let mut city: City = req.body_form().await?;
    // let state = &req.state();
    // let db = &state.client.database("test");
    // let id = req.param::<String>("city_id")?;
    // city.id = Some(ObjectId::with_string(&id).unwrap());

    // city.save(&db.clone(), None).await?;

    // let city_id = city.id.unwrap();

    Ok(tide::Redirect::new(format!("/channels")).into())
}