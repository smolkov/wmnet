use crate::Result;
use crate::{Workspace};
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{Utc};

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
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub elevation: Option<String>,
    pub status: Option<String>
}

impl TSData {
    pub fn new() -> TSData {
        let now = Utc::now();
        log::info!("NEW TIMESTAMP:{}",now);
        TSData{
            created_at: format!("{}",now),
            field1: None,
            field2: None,
            field3: None,
            field4: None,
            field5: None,
            field6: None,
            field7: None,
            field8: None,
            latitude: None,
            longitude: None,
            elevation: None,
            status: None
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
    pub fn set_wkey(&self, key: &str) -> Result<()> {
        fs::write(self.path.join(WKEY), key.as_bytes())?;
        Ok(())
    }
    /// change rkey
    pub fn set_rkey(&self, key: &str) -> Result<()> {
        fs::write(self.path.join(RKEY), key.as_bytes())?;
        Ok(())
    }
    /// change channel
    pub fn set_channel(&self, channel: &str) -> Result<()> {
        fs::write(self.path.join(CHANNEL), channel.as_bytes())?;
        Ok(())
    }
    pub fn update_url(&self) -> String {
        format!("https://api.thingspeak.com/channels/{}/bulk_update.json",self.channel().trim())
    }
    pub fn transmit(&self) -> Result<()> {
        let client = reqwest::blocking::Client::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            let data:TSData = serde_json::from_str(&fs::read_to_string(&path)?)?;
            let msg = TSMessage{
                write_api_key: self.wkey(),
                updates: vec![data],
            };
            let res = client.post(self.update_url().as_str()).json(&msg).send()?;
            log::info!("Response Status: {}",res.status());
            fs::remove_file(&path)?;
        }
        Ok(())
    }
    pub fn publish(&self,data:TSData) -> Result<()> {
        let now = Utc::now();
        let path = self.path.join(format!("{}.json", now.format("%Y%m%dT%H%M%S")));
        let data_str = serde_json::to_string(&data)?;
        fs::write(path, data_str.as_bytes())?;
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
    }
    Ok(thingspeak)
}

