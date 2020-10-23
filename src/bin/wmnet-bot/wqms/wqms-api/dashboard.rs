use rocket::get;
// use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;
use wmnet::channel::ChanState;

#[derive(Serialize)]
pub struct TeraContext {
    channels: Vec<ChanState>,
}

#[get("/")]
pub fn index() -> Template {
    let context = TeraContext {
        channels: vec![
            ChanState::new("TOC"),
            ChanState::new("DOS"),
            ChanState::new("pH"),
            ChanState::new("EC"),
            ChanState::new("ORC"),
            ChanState::new("DO"),
        ],
    };
    Template::render("index", &context)
}
