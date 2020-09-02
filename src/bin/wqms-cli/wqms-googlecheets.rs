
// use std::path::PathBuf;
// use async_std::task;
// use wqms::Workspace;
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    femme::with_level(log::LevelFilter::Info);
    let _ = wqms::ws::setup();
    let uri = "https://docs.google.com/spreadsheets/d/1Cb3wgsn1X5n7zcU18bTjj-jo22wPNHmSbi3TaCmJyQQ/values/Sheet1!A1:E1:append?valueInputOption=USER_ENTERED".to_owned();
    // https://docs.google.com/spreadsheets/d/e/2PACX-1vQIff5E94sveWjmDWobgKQ3t5yfjQDYFcsfUChgKzDI42nb5OvfwWldxHL6ZyVdrDe74ErVqzyp1GRB/pubhtm
    let data = serde_json::json!({
        "range": "Sheet1!A1:E1",
        "majorDimension": "ROWS",
        "values": [
            ["3/15/2016","nil", "nil","nil","nil"]
        ],
    });
    let res = surf::put(&uri).body_json(&data).unwrap().await.unwrap();
    Ok(())
}
