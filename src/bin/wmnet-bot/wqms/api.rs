use tide::{Body, Request, Response, StatusCode};

/// firebase
pub fn channels() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").get(|_| async {
        let channels = wmnet::wms::default().channels().unwrap();
        let infos = channels.list_info().unwrap();
        let mut res = Response::new(200);
        res.set_body(Body::from_json(&infos)?);
        Ok(res)
    });
    api.at("/").post(|_| async { Ok("Goodbye, world") });
    api
}
// use wmnet::Workspace;
async fn run_api() -> Result<(), std::io::Error> {

    tide::log::start();
    let _ = wmnet::wms::default();
    let mut app = tide::new();
    app.at("/static").serve_dir("/home/sascha/.wmnet/web/www/")?;
    app.at("/api/wifi").nest(wifi::api());
    app.at("/api/firebase").nest(setting::firebase());
    app.at("/api/stats").nest(setting::statistic());
    app.at("/api/channel").nest(channels());
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

pub async fn not_found(_req: Request<()>) -> tide::Result {
    let res = Response::new(StatusCode::NotFound);
    // res.render_html(|o| Ok(templates::notfound(o)?))?;
    Ok(res)
}
