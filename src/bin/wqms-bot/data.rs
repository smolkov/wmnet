use std::path::Path;
use telegram_bot::*;
use telegram_bot::{InputFileUpload};
use wqms::iface::Class;
use std::fs;
use chrono::{DateTime, Utc}; 
use glob::glob;

/// csv 
pub async fn tagesgang(api: Api, message:Message) -> Result<(),Error> {
    let channels = wqms::ws::default().channels().unwrap();
    let now: DateTime<Utc> = Utc::now();
    let path = channels.path().join(format!( "{}.csv",now.format("%Y%m%d")));
    if path.is_file() {
        let data = fs::read_to_string(&path).unwrap();
        let file = InputFileUpload::with_data(data,format!( "{}.csv",now.format("%Y%m%d")).as_str());
        api.send(message.from.document(&file).caption("Send to user")).await?; 
    }else {
        csv(api,message).await?;
    }
    Ok(())
}