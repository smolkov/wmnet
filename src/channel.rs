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
use chrono::{DateTime, Utc};
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufWriter;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;
const MEASUREMENTS: &str = "measurments";
const VALUE: &str = "value";
const AVERAGE_INTERVAL: &str = "average_interval";
const MEASUREMENT_INTERVAL: &str = "measurement_interval";
const OUTLIERS: &str = "outliers";
const MAXCV: &str = "maxcv";

#[derive(Debug, Serialize, Deserialize)]
pub struct ChanState {
    pub name: String,
    pub unit: String,
    pub value: f32,
    pub data: Vec<f32>,
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
            value: 0.0,
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
    const DATAS: &'static str = "datas.csv";
    /// get channel value
    fn value(&self) -> String {
        fs::read_to_string(self.path().join(VALUE)).unwrap_or("nil".to_owned())
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
    /// Average interval
    /// default 60 seconds
    fn average_interval(&self) -> std::time::Duration {
        if let Ok(sec) = fs::read_to_string(self.path().join(AVERAGE_INTERVAL)) {
            if let Ok(sec) = sec.parse::<u64>() {
                return std::time::Duration::from_secs(sec);
            }
        }
        std::time::Duration::from_secs(60)
    }
    /// Measurement interval
    /// default 1000 milliseconds
    fn measurement_interval(&self) -> std::time::Duration {
        if let Ok(millis) = fs::read_to_string(self.path().join(MEASUREMENT_INTERVAL)) {
            if let Ok(millis) = millis.parse::<u64>() {
                return std::time::Duration::from_millis(millis);
            }
        }
        std::time::Duration::from_millis(1000)
    }
    /// Outliers
    /// default 0
    fn outliers(&self) -> u16 {
        if let Ok(outliers) = fs::read_to_string(&self.path().join("outliers")) {
            if let Ok(outliers) = outliers.parse::<u16>() {
                return outliers;
            }
        }
        0
    }
    /// Max CV
    /// default :2.5
    fn maxcv(&self) -> f32 {
        if let Ok(maxcv) = fs::read_to_string(self.path().join(MAXCV)) {
            if let Ok(maxcv) = maxcv.parse::<f32>() {
                return maxcv;
            }
        }
        2.5
    }
    fn unit(&self) -> String {
        fs::read_to_string(self.path().join("unit")).unwrap_or("--".to_owned())
    }
    /// set channel value
    fn set_value(&self, value: f32) -> Result<()> {
        fs::write(self.path().join(VALUE), format!("{}", value).as_bytes())?;
        Ok(())
    }
    /// change unit
    fn set_unit(&self, unit: &str) -> Result<()> {
        fs::write(self.path().join("unit"), unit.as_bytes())?;
        Ok(())
    }
    /// change average interval value
    fn set_average_interval(&self, seconds: u64) -> Result<()> {
        fs::write(
            self.path().join(AVERAGE_INTERVAL),
            format!("{}", seconds).as_bytes(),
        )?;
        Ok(())
    }
    /// change measurement interval value
    fn set_measurement_interval(&self, millis: u64) -> Result<()> {
        fs::write(
            self.path().join(MEASUREMENT_INTERVAL),
            format!("{}", millis).as_bytes(),
        )?;
        Ok(())
    }
    /// change outliers counter
    fn set_outliers(&self, outliers: u16) -> Result<()> {
        fs::write(
            self.path().join(OUTLIERS),
            format!("{}", outliers).as_bytes(),
        )?;
        Ok(())
    }
    /// change max cv
    fn set_maxcv(&self, maxcv: f32) -> Result<()> {
        fs::write(self.path().join(MAXCV), format!("{}", maxcv).as_bytes())?;
        Ok(())
    }
    fn info(&self) -> ShortInfo {
        let label = self.label();
        let unit = self.unit();
        let value = self.value();
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
    fn push_data(&self, value: f32) -> Result<()> {
        let path = self.path().join("data");
        log::info!("{} push new value", self.path().display());
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        let time = fs::metadata(&path)?.created()?.elapsed().unwrap();
        let buffer = std::io::LineWriter::new(file);
        let mut wtr = csv::Writer::from_writer(buffer);
        //TODO: csv fehler fur Error implementieren.
        let data = Data {
            time: time.as_millis() as u64,
            value: value,
        };
        if let Err(e) = wtr.serialize(data) {
            log::error!("CHANNEL[{}] serialize data - {}", self.path().display(), e);
        } else if let Err(e) = wtr.flush() {
            log::error!("CHANNEL[{}] write data - {}", self.path().display(), e);
        };
        Ok(())
    }
    fn measurements(&self) -> u32 {
        if let Ok(count) = fs::read_to_string(self.path().join(MEASUREMENTS)) {
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
        if !path.is_file() {
            log::warn!("arhived file is not exist {}", path.display());
            return Ok(());
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
            self.measurements(),
            name
        ));
        fs::copy(&path, &history_path)?; // Copy foo.txt to bar.txt
        fs::remove_file(&path)?;
        Ok(())
    }
    fn next(&self) -> Result<()> {
        let datas = self.path().join(Self::DATAS);
        println!("next measurement in {}", self.path().display());
        fs::write(
            self.path().join(MEASUREMENTS),
            format!("{}", self.measurements()).as_bytes(),
        )?;
        if datas.is_file() {
            self.archive(&datas)?;
        }
        fs::File::create(&datas)?;
        Ok(())
    }
    fn calculate(&self, data: &Vec<f32>) -> Result<()> {
        if data.len() == 0 {
            fs::write(self.path().join(VALUE), "nil".as_bytes())?;
            return Ok(());
        }
        let mut sum: f32 = 0.0;
        for value in data.as_slice() {
            sum = sum + value;
        }
        let count = match data.len() {
            positive if positive > 0 => positive,
            _ => 1,
        };
        let mean = sum / count as f32;
        let value = mean;
        // self.push_data(data)?;
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
    pub fn measurements(&self) -> u32 {
        let path = self.path().join(MEASUREMENTS);
        if !path.is_file() {
            if let Err(e) = fs::write(&path, format!("{}", 0).as_bytes()) {
                log::error!("CHANNELS[{}]: measurements number - {}", path.display(), e);
            }
        }
        if let Ok(number) = fs::read_to_string(&path) {
            if let Ok(n) = number.parse::<u32>() {
                return n;
            }
        }
        0
    }
    pub fn next_measurement(&self) -> Result<()> {
        for ch in self.list()? {
            ch.next()?;
        }
        fs::write(
            self.path().join(MEASUREMENTS),
            format!("{}", self.measurements() + 1).as_bytes(),
        )?;
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
