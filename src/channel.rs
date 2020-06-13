use crate::Result;
use crate::Workspace;
use crate::{Class, Property};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::fmt::Display;
use std::fs;
// use std::fs::FileType;
// use std::io::prelude::*;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use std::sync::Mutex;

#[derive(Debug)]
pub struct ChanState {
    name: String,
    unit: String,
    data: Vec<f32>,
    start: Option<SystemTime>,
    last: Option<SystemTime>,
    mean: Option<f64>,
    cv: Option<f32>,
}

impl ChanState {
    pub fn new(name: &str) -> ChanState {
        ChanState {
            name: name.to_owned(),
            unit: "--".to_owned(),
            data: Vec::new(),
            start: None,
            last: None,
            mean: None,
            cv: None,
        }
    }
    pub fn add_value(&mut self, value: f32) {
        self.data.push(value);
        if self.start.is_none() {
            self.start = Some(SystemTime::now());
            self.last = None;
        }
        self.last = Some(SystemTime::now());
        self.data.push(value);
    }
}

lazy_static! {
    static ref CHANNELS_STATS: Mutex<HashMap<String, ChanState>> = {
        #[allow(unused_mut)]
        let mut m: HashMap<String, ChanState> = HashMap::new();
        m.insert("TOX".to_owned(), ChanState::new("TOX"));
        m.insert("DOS".to_owned(), ChanState::new("DOS"));
        m.insert("PH".to_owned(), ChanState::new("PH"));
        m.insert("EC".to_owned(), ChanState::new("EC"));
        m.insert("ORP".to_owned(), ChanState::new("ORP"));
        m.insert("DO".to_owned(), ChanState::new("DO"));
        Mutex::new(m)
    };
}
///Data
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub time: u64,
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortInfo {
    pub label: String,
    pub unit: String,
    pub value: String,
}

pub trait Chan: Class {
    const NUMBER: u16 = 0;
    // const NAME: &'static str = "chan";
    // const META: &'static str = "channels";
    const DATAS: &'static str = "datas";
    /// get channel value
    fn value(&self) -> Option<f32> {
        if let Ok(value) = fs::read_to_string(self.path().join("value")) {
            if let Ok(v) = value.parse::<f32>() {
                return Some(v);
            }
        }
        None
    }
    fn number(&self) -> u16 {
        let path = self.path().join("number");
        if !path.is_file() {
            if let Err(e) = fs::write(&path, format!("{}", Self::NUMBER).as_bytes()) {
                log::error!("channel[{}] create number - {}", path.display(), e);
            }
        }
        if let Ok(number) = fs::read_to_string(&path) {
            if let Ok(n) = number.parse::<u16>() {
                return n;
            }
        }
        0
    }
    fn unit(&self) -> String {
        fs::read_to_string(self.path().join("unit")).unwrap_or("--".to_owned())
    }
    /// set channel value
    fn set_value(&self, value: f32) -> Result<()> {
        fs::write(self.path().join("value"), format!("{}", value).as_bytes())?;
        Ok(())
    }
    fn set_unit(&self, unit: &str) -> Result<()> {
        fs::write(self.path().join("unit"), unit.as_bytes())?;
        Ok(())
    }
    fn info(&self) -> ShortInfo {
        let label = self.label();
        let unit = self.unit();
        let value = match self.value() {
            Some(val) => format!("{}", val),
            None => "--".to_owned(),
        };
        ShortInfo { label, unit, value }
    }
    fn data(&self) -> Vec<Data> {
        let mut signal: Vec<Data> = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
            .from_path(&self.path().join(Self::DATAS))
            .unwrap();
        for record in rdr.deserialize() {
            let row: Data = record.unwrap();
            signal.push(row);
        }
        signal
    }
    fn push_data(&self, signal: f32) -> Result<()> {
        let time = fs::metadata(Self::DATAS)?.created()?.elapsed().unwrap();
        let mut wtr = csv::Writer::from_path(&self.path().join(Self::DATAS)).unwrap();
        wtr.serialize(Data {
            time: time.as_secs(),
            value: signal,
        })
        .unwrap();
        wtr.flush().unwrap();
        Ok(())
    }
    fn counts(&self) -> u32 {
        if let Ok(count) = fs::read_to_string(self.path().join("count")) {
            if let Ok(c) = count.parse::<u32>() {
                return c;
            }
        }
        0
    }
    fn archive(&self, path: &Path) -> Result<()> {
        let history = self.path().join(".history");
        if !history.is_dir() {
            fs::create_dir_all(&history)?;
        }
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("unknown")
            .to_owned();
        let now: DateTime<Utc> = Utc::now();
        let history_path = history.join(format!(
            "{}-{}-{}",
            now.format("%Y%m%d%H%M"),
            self.counts(),
            name
        ));
        fs::copy(&path, &history_path)?; // Copy foo.txt to bar.txt
        fs::remove_file(&path)?;
        Ok(())
    }
    fn next(&self) -> Result<()> {
        let datas = self.path().join(Self::DATAS);
        fs::write(
            self.path().join("count"),
            format!("{}", self.counts()).as_bytes(),
        )?;
        self.archive(&datas)?;
        fs::File::create(&datas)?;
        Ok(())
    }
    fn calculate(&self) -> Result<()> {
        let datas = self.data();
        let mut sum: f32 = 0.0;
        for row in datas.as_slice() {
            sum = sum + row.value;
        }
        let count = match datas.len() {
            positive if positive > 0 => positive,
            _ => 1,
        };
        let mean = sum / count as f32;
        let value = mean;
        self.set_value(value)?;
        Ok(())
    }
}

/// Channel fs interface
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    path: PathBuf,
}
impl Class for Channel {
    const META: &'static str = "chan";
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Property for Channel {}
impl Chan for Channel {}

impl Channel {
    pub fn new(path: &Path) -> Channel {
        if !path.is_dir() {
            if let Err(e) = fs::create_dir_all(path) {
                log::error!("create channel {} - {}", path.display(), e);
            }
        }
        Channel {
            path: path.to_path_buf(),
        }
    }
    pub fn create(path: &Path, name: &str, unit: &str) -> Result<Channel> {
        let channel = Channel {
            path: path.to_path_buf(),
        };
        if !path.is_dir() {
            channel.setup()?;
            channel.set_label(name)?;
            channel.set_unit(unit)?;
        }
        Ok(channel)
    }
}

/// Channel
///
#[derive(Debug)]
pub struct Channels {
    path: PathBuf,
}

impl Class for Channels {
    const META: &'static str = "channels";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Property for Channels {}

impl Channels {
    pub fn list(&self) -> Result<Vec<Channel>> {
        let mut chs: Vec<Channel> = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            chs.push(Channel::new(&path));
        }
        Ok(chs)
    }
    pub fn get(&self, name: &str) -> Option<Channel> {
        let path = self.path.join(name);
        if path.is_dir() {
            return Some(Channel::new(&path));
        }
        None
    }
    pub fn get_info(&self, name: &str) -> Option<ShortInfo> {
        let path = self.path.join(name);
        if path.is_dir() {
            return Some(Channel::new(&path).info());
        }
        None
    }
    pub fn list_info(&self) -> Result<Vec<ShortInfo>> {
        let mut chsinfo = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            let ch = Channel::new(&path);
            chsinfo.push(ch.info());
        }
        Ok(chsinfo)
    }
    pub fn new_channel(&self, name: &str, unit: &str) -> Result<Channel> {
        let path = self.path.join(name);
        let new_chan = Channel::create(&path, name, unit)?;
        Ok(new_chan)
    }
    pub fn link_channel(&self, channel: Channel) -> Result<()> {
        let link_path = self.path.join(channel.get_name());
        symlink(&channel.path, &link_path)?;
        Ok(())
    }
    pub fn unlink_channel(&self, channel: Channel) -> Result<()> {
        let link_path = self.path.join(channel.get_name());
        fs::remove_file(link_path)?;
        Ok(())
    }
    pub fn next_measurement(&self) -> Result<()> {
        for ch in self.list()? {
            ch.next()?;
        }
        Ok(())
    }
}

pub fn open(ws: &Workspace) -> Channels {
    let path = ws.rootdir().join(Channels::META);
    Channels { path }
}

pub fn setup(ws: &Workspace) -> Result<Channels> {
    let path = ws.rootdir().join(Channels::META);
    let chs = Channels { path };
    if !chs.path.is_dir() {
        chs.setup()?;
    }

    Ok(chs)
}
