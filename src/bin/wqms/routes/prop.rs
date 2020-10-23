use tide::{Body,Response};
// use crate::models::Prop;
use handlebars::to_json;
use serde_json::value::Map;
use tide_handlebars::prelude::*;


pub async fn index(req: crate::Request) -> tide::Result {
    let state = &req.state();
    let props = wqms::prop::list(&state.ws);
   
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Property"));
    data.insert("parent".to_string(), to_json("layouts/main"));
    data.insert("props".to_string(), to_json(&props));
    Ok(hb.render_response_ext("props/list", &data, "html")?)
}

pub async fn list(req: crate::Request) -> tide::Result {
    let state = &req.state();
    let props = wqms::prop::list(&state.ws);
    Ok(Response::builder(tide::StatusCode::Ok)
    .body(Body::from_json(&props)?)
    .build()) 
}
pub async fn set(req: crate::Request) -> tide::Result {
    // let prop = req.param("n")?;
    let prop = req.param::<String>("city_id")?;
    Ok(Response::builder(tide::StatusCode::Ok)
    .body(Body::from_string(format!("Property {} get!",prop)))
    .build()) 
}
pub async fn get(req: crate::Request) -> tide::Result {
    let prop = req.param::<String>("city_id")?;
    Ok(Response::builder(tide::StatusCode::Ok)
    .body(Body::from_string(format!("Property {} get!",prop)))
    .build()) 
}




pub async fn edit(req: crate::Request) -> tide::Result {
    let state = &req.state();
    let id = req.param::<String>("prop_id")?;

    let prop = wqms::prop::get(&state.ws,&id);

    // let doc = City::find_one(&db.clone(), filter, None).await?;
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Property"));
    data.insert("parent".to_string(), to_json("layouts/main"));
    data.insert(
        "action".to_string(),
        to_json(format!("/props/{}/edit", id)),
    );
    data.insert("prop".to_string(), to_json(&prop));
    Ok(hb.render_response_ext("props/form", &data, "html")?)
}

pub async fn update(_req: crate::Request) -> tide::Result {
    // let mut city: City = req.body_form().await?;
    // let state = &req.state();
    // let db = &state.client.database("test");
    // let id = req.param::<String>("city_id")?;
    // city.id = Some(ObjectId::with_string(&id).unwrap());

    // city.save(&db.clone(), None).await?;

    // let city_id = city.id.unwrap();

    Ok(tide::Redirect::new(format!("/props")).into())
}