use std::path::{Path};
use telegram_bot::*;
use telegram_bot::{InputFileUpload};
use wmnet::{
    iface::Class,
    wms::Workspace,
};
use std::fs;
use chrono::{DateTime, Utc}; 
use glob::glob;


/// list 
fn list(wms:&Workspace,path:&Path) -> wmnet::Result<String> {
    let mut data = String::new();
    data.push_str("data:\n");
    for entry in glob(format!("{}/**/*",path.display()).as_str()).unwrap(){
        let p = entry.unwrap();
        if !p.is_dir() && p.extension().is_none(){
            let path = wms.truncate(&p);
            if wmnet::util::hiddens(&path)==0 {
                data.push_str(format!("{}",path.display()).as_str());
                data.push('\n');
            }
       }
    }
    Ok(data)
}

/// ls 
pub async fn ls(api: Api, message: Message)  -> Result<(), Error> {
    let wms = wmnet::wms::default();
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        let data = match cmd.len() {
            1 => list(&wms,wms.rootdir()).unwrap_or("ls cannot access".to_owned()),
            2 => list(&wms,wms.rootdir().join(cmd[1]).as_path()).unwrap_or("ls cannot access".to_owned()),
            _ => list(&wms,wms.rootdir()).unwrap_or("ls cannot access".to_owned()),
        };
        api.send(message.chat.text(data.as_str())).await?;
    }
    Ok(())
}

/// list csv
fn list_csv(path:&Path) -> wmnet::Result<String> {
    let mut data = String::new();
    data.push_str("Measurement csv files:\n");
    for entry in glob(format!("{}/*.csv",path.display()).as_str()).unwrap() {
        let p = entry.unwrap();
        let path = wmnet::util::truncate_prefix(&p,path);
        data.push_str(format!("{}\n",path.display()).as_str());
    }
    Ok(data)
}
/// csv 
pub async fn csv(api: Api, message:Message) -> Result<(),Error> {
    let wms = wmnet::wms::default();
    let channels = wms.channels().unwrap();
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        match cmd.len() {
           2 => {
               let path = channels.path().join(cmd[1]);
                api.send(message.chat.text(list_csv(&path).unwrap_or("ls csv cannot access".to_owned()).as_str())).await?; 
            }, 
            _ => {
                let data =format!("Measurement data:\n{}",list_csv(channels.path()).unwrap_or("ls csv cannot access".to_owned()));
                api.send(message.chat.text(data.as_str())).await?;  
            },
        };
    }
    Ok(())
}

/// csv 
pub async fn download(api: Api, message:Message) -> Result<(),Error> {
    let channels = wmnet::wms::default().channels().unwrap();
    println!("csv");
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        match cmd.len() {
           2 => {
                let path = if cmd[1].ends_with(".csv"){
                    channels.path().join(cmd[1])
                } else {
                    channels.path().join(format!("{}.csv",cmd[1])) 
                };
                if path.is_file() {
                    let data = fs::read_to_string(&path).unwrap();
                    let file = InputFileUpload::with_data(data,"data.csv");
                    api.send(message.from.document(&file).caption("Send to user")).await?; 
                }else {
                    csv(api,message).await?;
                }

            }, 
            _ => {
                api.send(message.chat.text(format!("dowonload cmd wrong format /download YYYMMDD for example:`/download 20200911` to download data `11 Sep. 2020`"))).await?;
            },
        };
    }
    Ok(())
}
/// csv 
pub async fn dlast(api: Api, message:Message) -> Result<(),Error> {
    let channels = wmnet::wms::default().channels().unwrap();
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