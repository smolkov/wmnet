use crate::Result;
use crate::{Workspace};
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{Utc};
use glob::glob;
// use serde::{Deserialize, Serialize};
// use std::sync::mpsc::{self, channel};
// use std::thread;

// use crossbeam::channel::{select, tick};
// use std::fmt;
// use std::fs;
use std::path::{PathBuf};
// use systemstat::{Platform, System};
/// IPFILE name

/// The core abstraction in Cargo for working with a workspace of crates.
///
/// A workspace is often created very early on and then threaded through all
/// other functions. It's typically through this object that the current
/// package is loaded and/or learned about.
/// Station mode
///
///
const TSWKEY: &str = "RQ1HTKE735B65NVI";
const TSRKEY: &str = "XZUIDN95GI2ZOSBX";
const TSCHANNEL: &str = "1114700";
const WKEY: &str = ".wkey";
const RKEY: &str = ".rkey";
const CHANNEL: &str = "channel";


#[derive(Debug, Serialize, Deserialize)]
pub struct TSData {
    pub created_at: String,
    pub field1: Option<f32>,
    pub field2: Option<f32>,
    pub field3: Option<f32>,
    pub field4: Option<f32>,
    pub field5: Option<f32>,
    pub field6: Option<f32>,
    pub field7: Option<f32>,
    pub field8: Option<f32>,
    pub latitude: f32,
    pub longitude: f32,
    pub elevation: String,
    pub status: String
}

impl TSData {
    pub fn new() -> TSData {
        let now = Utc::now();
        log::info!("NEW TIMESTAMP:{}",now.to_rfc3339());
        TSData{
            created_at: now.to_rfc3339(),
            field1: None,
            field2: None,
            field3: None,
            field4: None,
            field5: None,
            field6: None,
            field7: None,
            field8: None,
            latitude: 52.4629112,
            longitude: 13.4692855,
            elevation: "".to_owned(),
            status: "".to_owned()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TSMessage {
    pub write_api_key: String,
    pub updates: Vec<TSData>,
}

pub struct ThingSpeak {
    path: PathBuf,
}

impl ThingSpeak {
    pub fn wkey(&self) -> String {
        fs::read_to_string(self.path.join(WKEY)).unwrap_or(TSWKEY.to_owned())
    }
    pub fn rkey(&self) -> String {
        fs::read_to_string(self.path.join(RKEY)).unwrap_or(TSRKEY.to_owned())
    }
    pub fn channel(&self) -> String {
        fs::read_to_string(self.path.join(CHANNEL)).unwrap_or(TSCHANNEL.to_owned())
    }
    /// change wkey
    pub fn set_wkey(&self, key: &str) -> &Self {
        if let Err(_e) = fs::write(self.path.join(WKEY), key.as_bytes()) {
            log::error!("thingspeak change api write key failed");
        }
        self
    }
    /// change rkey
    pub fn set_rkey(&self, key: &str) -> &Self {
        if let Err(_) = fs::write(self.path.join(RKEY), key.as_bytes()) {
            log::error!("thingspeak change api read key failed");
        }
        self
    }
    /// change channel
    pub fn set_channel(&self, channel: &str) -> &Self {
        if let Err(_e) = fs::write(self.path.join(CHANNEL),channel.as_bytes()){
            log::error!("thingspeak change channel failed");
        }
        self
    }
    pub fn update_url(&self) -> String {
        format!("https://api.thingspeak.com/channels/{}/bulk_update.json",self.channel().trim())
    }
    pub fn transmit(&self) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        for entry in glob(format!("{}/*.json",self.path.display()).as_str()).unwrap() {
            let path = entry.unwrap();
            let data_str =  fs::read_to_string(&path)?;
            match serde_json::from_str::<TSData>(&data_str) {
                Ok(data) => {
                    let msg = TSMessage{
                        write_api_key: self.wkey(),
                        updates: vec![data],
                    };
                    let res = client.post(self.update_url().as_str()).json(&msg).send()?;
                    log::info!("Response Status: {}",res.status());
                    if res.status().is_success(){
                        fs::remove_file(&path)?;
                    }else {
                        break;
                    }
                },
                Err(e) => {
                    log::error!("TS {} entcode failed - {}",path.display(),e);
                    fs::remove_file(&path)?;
                }
            }
            
            // let data_str = serde_json::to_string(&msg)?;
            // println!("TS {} TRANSMIT:{}",path.display(),data_str);
            //
        }
        Ok(())
    }

    // pub fn settings(&self) -> Result<()>{
    //     //
    //     // curl -X GET "https://api.thingspeak.com/channels/1114700.json?api_key=XZUIDN95GI2ZOSBX" -H "accept: application/json"
    //     let client = reqwest::blocking::Client::new();
    //     let res = client.post(self.update_url().as_str()).json(&msg).send()?;
 
    // }
    pub fn publish(&self,data:TSData) -> Result<()> {
        let now = Utc::now();
        let path = self.path.join(format!("{}.json", now.format("%Y%m%dT%H%M%S")));
        let data_str = serde_json::to_string(&data)?;
        fs::write(path, data_str.as_bytes())?;
        println!("TS PUBLISH:{}",data_str);
        self.transmit()
    }
}
pub fn setup(ws: &Workspace) -> Result<ThingSpeak> {
    let path = ws.rootdir().join("thingspeak");
    let thingspeak = ThingSpeak {
        path: path.to_path_buf(),
    };
    if !thingspeak.path.is_dir() {
        log::info!("Create new thingsspeak directory {}",thingspeak.path.as_path().display());
        fs::create_dir_all(&thingspeak.path)?;
        thingspeak.set_wkey("3KBW4UF0N9KGJHU8").set_rkey("8MOUY0P3OFF9CECP").set_channel("1125745");

    }
    Ok(thingspeak)
}

