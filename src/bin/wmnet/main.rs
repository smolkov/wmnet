// use std::{env, time::Duration};
// use tide::{sessions::SessionMiddleware, Redirect};

// pub mod records;
// mod templates;
use handlebars::Handlebars;
// use tide_handlebars::prelude::*;
use async_std::sync::Arc;
mod app;
mod utils;
mod routes;
mod models;
use async_std::task;
use tide::{sessions::SessionMiddleware, Redirect};

#[derive(Clone)]
pub struct State {
    // db: SqlitePool,
    wms: wmnet::wms::Workspace,
    client: redis::Client,
    registry: Arc<Handlebars<'static>>,
}

impl State {
    // pub async fn rendner(&self,name:&str,data: &serde_json::Value) -> Result<tide::Body>    {
          /*
         * In debug mode, reload the templates on ever render to avoid
         * needing a restart
         */
        // let hb = self.registry;
        // let view = self.registry.render(name, data)?;
        // Ok(tide::Body::from_string(view))
    // }
}

pub type Request = tide::Request<State>;

// async fn db_connection() -> tide::Result<SqlitePool> {
//     let database_url = env::var("DATABASE_URL")?;
//     Ok(SqlitePool::new(&database_url).await?)
// }

// async fn build_session_middleware(
//     db: SqlitePool,
// ) -> tide::Result<SessionMiddleware<SqliteSessionStore>> {
//     let session_store = SqliteSessionStore::from_client(db);
//     session_store.migrate().await?;
//     session_store.spawn_cleanup_task(Duration::from_secs(60 * 15));
//     let session_secret = env::var("TIDE_SECRET").unwrap();
//     Ok(SessionMiddleware::new(
//         session_store,
//         session_secret.as_bytes(),
//     ))
// }
#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let wms = wmnet::wms::default();
    // create tunnel
    task::spawn(async {
        match wmnet::ssh::tunnel().await {
            Ok(_) => log::info!("tunnel ok "),
            Err(e) => log::warn!("tunnel error {}",e)
        }
    });
    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "./www/views").unwrap();
    let registry = Arc::new(hb);
    let mut app = tide::with_state(State {wms,client,registry});
    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        std::env::var("TIDE_SECRET")
            .expect(
                "Please provide a TIDE_SECRET value of at \
                      least 32 bytes in order to run this example",
            )
            .as_bytes(),
    )); 

   // Redirect hackers to YouTube.
    app.at("/.env").get(tide::Redirect::new("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));

    // welcome
    app.at("/").get(tide::Redirect::new("/welcome"));
    app.at("/welcome").get(routes::welcome);

    let mut channs = app.at("/channels");
    channs.at("/").get(routes::channel::index);
    channs.at("/:chan_id")
        .get(routes::channel::edit)
        .post(routes::channel::update);
        // .post(routes::channel::update);

    let mut props = app.at("/props");
    props.at("/").get(routes::prop::index);
    props.at("/:prop_id")
        .get(routes::prop::edit)
        .post(routes::prop::update);
    
    let mut wifi = app.at("/wifi");
    wifi.at("/").get(routes::wifi::index);
    wifi.at("/:ssid")
            .get(routes::wifi::edit)
            .post(routes::wifi::update);
    // app.at("/").get(Redirect::new("/dashboard"));

    // app.at("/welcome").get(routes::welcome);
    let mut api = app.at("/api");
    let mut property = api.at("/prop");
    property
        .get(routes::prop::list)
        .at("/:prop_id")
        .get(routes::prop::get)
        .post(routes::prop::set);

    
        
    let mut channel = api.at("/channel");
    channel
        .get(routes::channel::list)
        .at("/:id")
        .post(routes::channel::set)
        .get(routes::channel::get);

    let mut wifi = api.at("/wifi");
    wifi
        .get(routes::wifi::list)
        .at("/connect").post(routes::wifi::connect);
    // articles.at("/new").get(routes::articles::new);
    // articles
        // .at("/:article_id")
        // .get(routes::articles::show)
        // .delete(routes::articles::delete)
        // .put(routes::articles::update)
        // .post(routes::articles::update);
    app.at("/public").serve_dir("www/public/")?;
    app.listen("127.0.0.1:8000").await?;
    Ok(())
}
