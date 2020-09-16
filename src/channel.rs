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
use rand_distr::{Normal, Distribution};
use rand::thread_rng;
use std::sync::Mutex;
use std::time::SystemTime;

const UNIT: &'static str = "unit";
const VALUE: &'static str = "value";
const STATUS: &'static str = "status";
const DATAS: &'static str = "data.csv";
const SLOPE: &'static str = "slope";
const INTERCEPT: &'static str = "intercept";

const AVERAGE_INTERVAL: &str = "average_interval";
const MEASUREMENT_INTERVAL: &str = "measurement_interval";
const OUTLIERS: &str = "outliers";
const MAXCV: &str = "maxcv";
const SCALE: &str = "scale";
const MIN: &str = "min";
const MAX: &str = "max";

pub const NITRI_TOX: &str = "01-tox";
pub const NITRI_DOS: &str = "02-dos";
pub const JUMO_PH: &str = "03-ph";
pub const JUMO_COND: &str = "04-cond";
pub const JUMO_ORP: &str = "05-orp";
pub const JUMO_TEMP: &str = "06-dos";
pub const AIN_TUR: &str = "07-tur";

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    pub slope: f32,
    pub intercept: f32,
}

impl Scale {
    pub fn new(from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> Scale {
        Scale {
            slope: 1.0,
            intercept: 0.0,
        }
    }
    pub fn scale(&self,value:f32) -> f32 {
        value*self.slope + self.intercept
    }
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

// lazy_static! {
//     static ref CHANNELS_STATS: Mutex<HashMap<String, ChanState>> = {
//         #[allow(unused_mut)]
//         let mut m: HashMap<String, ChanState> = HashMap::new();
//         m.insert("TOX".to_owned(), ChanState::new("TOX"));
//         m.insert("DOS".to_owned(), ChanState::new("DOS"));
//         m.insert("PH".to_owned(), ChanState::new("PH"));
//         m.insert("EC".to_owned(), ChanState::new("EC"));
//         m.insert("ORP".to_owned(), ChanState::new("ORP"));
//         m.insert("DO".to_owned(), ChanState::new("DO"));
//         Mutex::new(m)
//     };
// }
///Data
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub time: u64,
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ShortInfo {
    pub label: String,
    pub unit: String,
    pub value: String,
}

impl ShortInfo {
    pub fn new(label:&str,value:&str) -> ShortInfo {
        ShortInfo{
            label: label.to_owned(),
            unit: "--".to_owned(),
            value: value.to_owned(),
        }    
    }
}
/// Channel fs interface
pub struct Channel {
    path: PathBuf,
    data: Vec<f32>,
}
impl Class for Channel {
    const META: &'static str = "channel";
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Channel {
    pub fn is_channel(_path:&Path) -> bool {
       true 
    }
    pub fn new(path: &Path) -> Channel {
        if !path.is_dir() {
            if let Err(e) = fs::create_dir_all(path) {
                log::error!("create channel {} - {}", path.display(), e);
            }
        }
        Channel {
            path: path.to_path_buf(),
            data: Vec::new()
        }
    }
    fn id(&self) -> String {
        if let Some(name) = self.path.file_name() {
            if let Some(s) = name.to_str() {
                return s.to_owned()
            }
        }
        return "null".to_owned()

    }
    pub fn status(&self)-> String {
        fs::read_to_string(self.path.join(STATUS)).unwrap_or("E".to_owned())
    }

    pub fn unit(&self) -> String {
        fs::read_to_string(self.path.join(UNIT)).unwrap_or("--".to_owned())
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn value(&self) -> String {
        fs::read_to_string(self.path().join(VALUE)).unwrap_or("nil".to_owned())
    }
    pub fn last_value(&self) -> Option<f32> {
        let value = self.value();
        if let Ok(val) = value.parse::<f32>() {
            Some(val as f32)
        } else {
            None
        }
    }
    pub fn slope(&self) -> f32 {
        match fs::read_to_string(self.path().join(SLOPE)) {
            Ok(m) => {
                if let Ok(val) = m.parse::<f32>() {
                    val
                } else {
                    1.0 as f32
                }
            }
            Err(_) => 1.0 as f32,
        }
    }

    pub fn intercept(&self) -> f32 {
        match fs::read_to_string(self.path().join(INTERCEPT)) {
            Ok(m) => {
                if let Ok(val) = m.parse::<f32>() {
                    val
                } else {
                    0.0 as f32
                }
            }
            Err(_) => 0.0 as f32,
        }
    }
    pub fn scale(&self) -> Scale {
        Scale {
            slope: self.slope(),
            intercept: self.intercept(),
        }
    }
    pub fn range(&self) -> (f32, f32) {
        let min: f32 = match fs::read_to_string(self.path().join(MIN)) {
            Ok(m) => {
                if let Ok(val) = m.parse::<f32>() {
                    val as f32
                } else {
                    0.0 as f32
                }
            }
            Err(_) => 0.0 as f32,
        };
        let max = match fs::read_to_string(self.path().join(MAX)) {
            Ok(m) => {
                if let Ok(val) = m.parse::<f32>() {
                    val
                } else {
                    0.0 as f32
                }
            }
            Err(_) => 0.0 as f32,
        };
        (min, max)
    }
   
    /// set channel value
    pub fn set_value(&self, value: f32) -> Result<()> {
        fs::write(self.path.join(VALUE), format!("{}", value).trim().as_bytes())?;
        Ok(())
    }
    /// change unit
    pub fn set_unit(&self, unit: &str) -> Result<()> {
        fs::write(self.path.join(UNIT), unit.trim().as_bytes())?;
        Ok(())
    }
    pub fn set_slope(&self,slope:&str) -> Result<()> {
        fs::write(self.path().join(SLOPE), slope.trim().as_bytes())?;
        Ok(())
    }
    pub fn set_intercept(&self,intercept:&str) -> Result<()> {
        fs::write(self.path().join(INTERCEPT), intercept.trim().as_bytes())?; 
        Ok(())
    }
    pub fn set_scale(&self, scale: Scale) -> Result<()> {
        self.set_slope(format!("{}",scale.slope).as_str())?;
        self.set_intercept(format!("{}",scale.intercept).as_str())?;
        Ok(())
    }
    pub fn set_min(&self,max:&str) -> Result<()> {
        fs::write(self.path().join(MAX), max.trim().as_bytes())?;
        Ok(())
    }
    pub fn set_max(&self,max:&str) -> Result<()> {
        fs::write(self.path().join(MAX), max.trim().as_bytes())?;
        Ok(())
    }
    pub fn set_range(&self, min: f32, max: f32) -> Result<()> {
        self.set_min(format!("{}",min).as_str())?;
        self.set_max(format!("{}",max).as_str())?;
        Ok(())
    }
    
    pub fn push_data(&mut self, data: &str) -> Result<()> {
        let value: f32 = data.parse::<f32>()?;
        self.data.push(self.scale().scale(value));
        Ok(())
    }
    pub fn push_value(&mut self,value:f32) -> Result<()> {
        self.data.push(self.scale().scale(value));
        Ok(())
    }
    pub fn calculate(&mut self) -> Result<()> {
        if self.data.len() == 0 {
            fs::write(self.path.join(VALUE), "nil".as_bytes())?;
            return Ok(());
        }
        let mut sum: f32 = 0.0;
        for value in self.data.as_slice() {
            sum = sum + value;
        }
        let count = match self.data.len() {
            positive if positive > 0 => positive,
            _ => 1,
        };
        let mean = sum / count as f32;
        let value = mean;
        self.set_value(value)?;
        self.history()?;
        self.clear();
        Ok(())
    }
    pub fn history(&self) -> Result<()> {
        let history = self.path().join("history");
        if !history.is_dir() {
            fs::create_dir_all(&history)?;
        }
        let now: DateTime<Utc> = Utc::now();
        let path = history.join(format!( "{}.csv",now.format("%Y%m%d")));
        
        // if let Err(e) = wtr.write_record(&[ "timestamp", "tox", "dos", "ph", "orp", "cond","temp", "tur"]) {
            // log::error!("csv write header data - {}", e);
        // }
        let mut last = fs::read_to_string(&path).unwrap_or("".to_owned());
        last.push_str(format!("{},{}\n",now,self.value()).as_str());
        println!("next measurement in {}", self.path().display());
        fs::write(&path,last.as_bytes())?;
        Ok(())
    }
    pub fn markdown(&self) -> String {
        format!("*{}:* `{}` [{}] *{}*\n",self.label(),self.value(),self.unit(),self.status())
    }
    pub fn link_value(&self,path:&Path) -> Result<()> {
        symlink(self.path.join(VALUE), &path)?;
        Ok(())
    }
    pub fn link_label(&self,path:&Path) -> Result<()> {
        symlink(self.path.join(Self::LABEL), &path)?;
        Ok(())
    }
    pub fn link_unit(&self,path:&Path) -> Result<()> {
        symlink(self.path.join(UNIT), &path)?;
        Ok(())
    }
    pub fn link_info(&self,directory:&Path) -> Result<()> {
        self.link_value(directory.join(format!("{}.value",self.id())).as_path())?;
        self.link_label(directory.join(format!("{}.label",self.id())).as_path())?;
        self.link_unit(directory.join(format!("{}.unit",self.id())).as_path())?;
        Ok(())
    }
    /// Channel short info
    pub fn info(&self) -> ShortInfo {
        let label = self.label();
        let unit = self.unit();
        let value = self.value();
        ShortInfo { label, unit, value }
    }
}

/// Channel
///
pub struct Channels {
    path: PathBuf,
    pub list: Vec<Channel>,
}

impl Class for Channels {
    const META: &'static str = "";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Property for Channels {}
const LAST: &'static str = "last";
 
impl Channels {
    pub fn new(path:&Path) -> Result<Channels> {
        if !path.is_dir() {
            fs::create_dir_all(path)?;
        }
        let mut list: Vec<Channel> = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                list.push(Channel::new(&path));
            }
        }
        let chs = Channels { 
            path : path.to_path_buf(),
            list: list,
        };
        Ok(chs)
    }
    pub fn list(&self) -> Result<Vec<Channel>> {
        let mut list: Vec<Channel> = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                list.push(Channel::new(&path));
            }
        } 
        Ok(list)
    }
    pub fn find(&self, id: &str) -> Option<Channel> {
        let path = self.path.join(id);
        if path.is_dir() {
            return Some(Channel::new(&path));
        }
        None
    }
    pub fn get_info(&self, id: &str) -> Option<ShortInfo> {
        let path = self.path.join(id);
        if path.is_dir() {
            return Some(Channel::new(&path).info());
        }
        None
    }
    pub fn infos(&self) -> Result<Vec<ShortInfo>> {
        let mut chsinfo = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let ch = Channel::new(&path);
                chsinfo.push(ch.info());
            }
        }
        Ok(chsinfo)
    }
    pub fn lastid(&self) -> u64 {
        let path = self.path().join(LAST);
        if !path.is_file() {
            if let Err(e) = fs::write(&path, format!("{}", 0).trim().as_bytes()) {
                log::error!("CHANNELS[{}]: last channel number - {}", path.display(), e);
            }
        }
        if let Ok(number) = fs::read_to_string(&path) {
            if let Ok(n) = number.parse::<u64>() {
                return n;
            }
        }
        0 
    }
    pub fn nextid(&self) -> Result<u64> {
        let id = self.lastid() + 1;
        fs::write(self.path().join(LAST), format!("{}", id).trim().as_bytes())?;
        Ok(id)
    }
    pub fn add(&self, label: &str, unit: &str) -> Result<Channel> {
        let path = self.path.join( format!("{}",self.nextid()?));
        let channel = Channel {
            path: path.to_path_buf(),
            data:Vec::new(),
        };
        if !path.is_dir() {
            log::info!("create new channel {} {} {}",path.as_path().display(),label,unit);
            channel.setup()?;
            channel.set_label(label)?;
            channel.set_unit(unit)?;
        }
        Ok(channel)
    }
    pub fn add_link(&self,channel:Channel) -> Result<Channel> {
        let path = self.path.join( format!("{}",self.nextid()?));
        symlink(&channel.path, &path)?;
        Ok(Channel{
            path: path,
            data:Vec::new(),
        })
    }

    /// Measurement interval
    /// default 1000 milliseconds
    pub fn measurement_interval(&self) -> std::time::Duration {
        if let Ok(millis) = fs::read_to_string(self.path().join(MEASUREMENT_INTERVAL)) {
            if let Ok(millis) = millis.parse::<u64>() {
                return std::time::Duration::from_millis(millis);
            }
        }
        std::time::Duration::from_millis(1000)
    }
    /// Averaging interval
    /// default 60 sec
     pub fn average_interval(&self) -> std::time::Duration {
        if let Ok(interval) = fs::read_to_string(self.path().join(AVERAGE_INTERVAL)) {
            if let Ok(interval) = interval.parse::<u64>() {
                return std::time::Duration::from_secs(interval);
            }
        }
        std::time::Duration::from_secs(60)
    }
    /// Outliers
    /// default 0
    pub fn outliers(&self) -> u16 {
        if let Ok(outliers) = fs::read_to_string(&self.path().join("outliers")) {
            if let Ok(outliers) = outliers.parse::<u16>() {
                return outliers;
            }
        }
        0
    }
    /// Max CV
    /// default :2.5
    pub fn maxcv(&self) -> f32 {
        if let Ok(maxcv) = fs::read_to_string(self.path().join(MAXCV)) {
            if let Ok(maxcv) = maxcv.parse::<f32>() {
                return maxcv;
            }
        }
        2.5
    }
    /// change average interval value
    pub fn set_average_interval(&self, seconds: u64) -> Result<()> {
        fs::write(
            self.path().join(AVERAGE_INTERVAL),
            format!("{}", seconds).as_bytes(),
        )?;
        Ok(())
    }
    /// change measurement interval value
    pub fn set_measurement_interval(&self, millis: u64) -> Result<()> {
        fs::write(
            self.path().join(MEASUREMENT_INTERVAL),
            format!("{}", millis).as_bytes(),
        )?;
        Ok(())
    }
    /// change outliers counter
    pub fn set_outliers(&self, outliers: u16) -> Result<()> {
        fs::write(
            self.path().join(OUTLIERS),
            format!("{}", outliers).as_bytes(),
        )?;
        Ok(())
    }
    /// change max cv
    pub fn set_maxcv(&self, maxcv: f32) -> Result<()> {
        fs::write(self.path().join(MAXCV), format!("{}", maxcv).as_bytes())?;
        Ok(())
    }
    /// channels collection history
    pub fn history(&self)  -> Result<()> {
        let now: DateTime<Utc> = Utc::now();
        let path = self.path().join(format!( "{}.csv",now.format("%Y%m%d")));
        if !path.is_file() {
            let mut head = String::new();
            head.push_str("timestap");
            for chan in self.list.as_slice() {
                head.push(',');
                head.push_str(chan.label().as_str());
            }
            head.push('\n');
            fs::write(&path,head.as_bytes())?; 
        }
        let mut last = fs::read_to_string(&path).unwrap_or("".to_owned());
        last.push_str(format!("{}",now).as_str());
        for chan in self.list.as_slice() {
            last.push(',');
            last.push_str(chan.value().as_str());
        } 
        last.push('\n');
        fs::write(&path,last.as_bytes())?; 
        Ok(())
    }
   
    pub fn markdown(&self) -> Result<String> {
        let mut md = String::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && Channel::is_channel(&path) {
                md.push_str(Channel::new(&path).markdown().as_str());
            }
        } 
        Ok(md)
    }
}

pub fn setup(ws: &Workspace) -> Result<Channels> {
    let path = ws.rootdir().join("channels");
    Channels::new(&path)
}
