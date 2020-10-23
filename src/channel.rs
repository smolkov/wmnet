use crate::Result;
use crate::Workspace;
use crate::{Class, Property};
// use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fmt::Display;
use std::fs;
use std::time::{Duration, SystemTime};

// use std::fs::FileType;
// use std::io::prelude::*;
use chrono::{DateTime, Utc};
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufWriter;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

const UNIT: &'static str = "unit";
const VALUE: &'static str = "value";
const STATUS: &'static str = "status";
// const DATAS: &'static str = "data.csv";
const SLOPE: &'static str = "slope";
const INTERCEPT: &'static str = "intercept";

const AVERAGE_INTERVAL: &str = "average_interval";
const MEASUREMENT_INTERVAL: &str = "measurement_interval";
const OUTLIERS: &str = "outliers";
const MAXCV: &str = "maxcv";
// const SCALE: &str = "scale";
const MIN: &str = "min";
const MAX: &str = "max";

const SIGNAL: &str = "signal.csv";

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
    // pub fn new(from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> Scale {
    //     Scale {
    //         slope: 1.0,
    //         intercept: 0.0,
    //     }
    // }
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
pub struct ChanInfo {
    pub id: String,
    pub label: String,
    pub unit: String,
    pub value: String,
}

impl ChanInfo {
    pub fn new(label:&str,value:&str) -> ChanInfo {
        ChanInfo{
            id: "none".to_owned(),
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
    pub fn create(path :&Path, id:&str,label:&str,unit:&str) -> Channel {
        let ch  = Channel {
            path: path.join(id).to_path_buf(),
            data: Vec::new()
        };
        if !ch.path().is_dir() {
            if let Err(e) = ch.init() {
                log::error!("channel {} init failed - {}", path.display(), e);
            }else{
                ch.set_label(label).set_unit(unit);
            }
        }
        ch
    }
    pub fn new(path: &Path) -> Channel {
        let ch  = Channel {
            path: path.to_path_buf(),
            data: Vec::new()
        };
        if let Err(e) = ch.init() {
            log::error!("channel {} init failed - {}", path.display(), e);
        }
        ch

    }
    fn id(&self) -> String {
        if let Some(name) = self.path.file_name() {
            if let Some(s) = name.to_str() {
                return s.to_owned()
            }
        }
        return "null".to_owned()

    }
    pub fn init(&self) -> Result<()> {
        if !self.path().is_dir() {
            if let Err(e) = fs::create_dir_all(self.path()) {
                log::error!("channel {} create directory - {}", self.path().display(), e);
            }else {
                fs::write(self.path().join(VALUE),"none".as_bytes())?;
                fs::write(self.path().join(UNIT), "--".as_bytes())?;
                fs::write(self.path().join(SLOPE), "1.0".as_bytes())?;
                fs::write(self.path().join(INTERCEPT), "0.0".as_bytes())?; 
                fs::write(self.path().join(MAX), "0.0".as_bytes())?;
                fs::write(self.path().join(MAX), "500.0".as_bytes())?;
                fs::write(self.path().join(STATUS), "I".as_bytes())?; 
            }
        }
        Ok(())
    }
    pub fn clear(&mut self) -> Result<()> {
        fs::write(self.path.join(VALUE), "none".as_bytes())?;
        self.clear_data();
        Ok(())
    }
    pub fn status(&self)-> String {
        fs::read_to_string(self.path.join(STATUS)).unwrap_or("E".to_owned())
    }

    pub fn unit(&self) -> String {
        fs::read_to_string(self.path.join(UNIT)).unwrap_or("--".to_owned())
    }
   
    pub fn value(&self) -> String {
        fs::read_to_string(self.path().join(VALUE)).unwrap_or("none".to_owned())
    }
    pub fn last_value(&self) -> Option<f32> {
        let value = self.value();
        if let Ok(val) = value.parse::<f32>() {
            Some(val as f32)
        } else {
            None
        }
    }
    pub fn get_value(&self) -> Option<f32> {
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
    fn set_value(&self, value: f32) -> &Channel {
        if let Err(e) = fs::write(self.path.join(VALUE), format!("{}", value).trim().as_bytes()) {
            log::error!("channel {} change value to {} failed - {}",self.path().display(),value,e);
            self.set_status("E");
        }
        self
    }
     /// set channel value
     pub fn set_status(&self, status:&str) -> &Channel{
        if let Err(e) = fs::write(self.path().join(STATUS), status.trim().as_bytes()) {
            log::error!("channel {} change status to {} failed - {}",self.path().display(),status,e);
        }
        self
    }
    /// change unit
    pub fn set_unit(&self, unit: &str) -> &Channel{
        if let Err(e) = fs::write(self.path.join(UNIT), unit.trim().as_bytes()) {
            log::error!("channel {} change unit to {} failed - {}",self.path().display(),unit,e);
        }
        self
    }
    pub fn set_slope(&self,slope:&str) -> &Channel {
        if let Err(e) = fs::write(self.path().join(SLOPE), slope.trim().as_bytes()) {
            log::error!("channel {} change slope to {} failed - {}",self.path().display(),slope,e);
        }
        self
    }
    pub fn set_intercept(&self,intercept:&str) -> &Channel{
        if let Err(e) = fs::write(self.path().join(INTERCEPT), intercept.trim().as_bytes()) {
            log::error!("channel {} change intercept to {} failed - {}",self.path().display(),intercept,e);
        }
        self
    }
    pub fn set_scale(&self, scale: Scale) -> &Channel {
        self.set_slope(format!("{}",scale.slope).as_str());
        self.set_intercept(format!("{}",scale.intercept).as_str());
        self
    }
    pub fn set_min(&self,min:&str) -> &Channel {
        if let Err(e) = fs::write(self.path().join(MIN), min.trim().as_bytes()){
            log::error!("channel {} change intercept to {} failed - {}",self.path().display(),min,e);
        }
        self
    }
    pub fn set_max(&self,max:&str) -> &Channel {
        if let Err(e) = fs::write(self.path().join(MAX), max.trim().as_bytes()) {
            log::error!("channel {} change intercept to {} failed - {}",self.path().display(),max,e);
        }
        self
    }
    pub fn set_range(&self, min: f32, max: f32) -> &Channel {
        self.set_min(format!("{}",min).as_str());
        self.set_max(format!("{}",max).as_str());
        self
    }
    pub fn clear_data(&mut self) {
        let path = self.path().join(SIGNAL);
        if path.is_file() {
            if let Err(e) = fs::remove_file(&path) {
                log::error!("channel {} clear data failed - {}",self.path().display(),e);
            }
        }
        self.data.clear();
    }
    pub fn push_data(&mut self, data: &str) -> Result<()>  {
        match data.parse::<f32>() {
            Ok(value) => {
                self.push_value(value)?;
            } ,
            Err(e) => {

                log::error!("channel {} push value failed - {}",self.path().display(),e);
            } 
        }
        Ok(())
    }
    pub fn push_value(&mut self,value:f32) -> Result<()> {
        let path = self.path().join("signal.csv");
        // if let Err(e) = wtr.write_record(&[ "timestamp", "tox", "dos", "ph", "orp", "cond","temp", "tur"]) {
            // log::error!("csv write header data - {}", e);
        // }

        let ctime = if let Ok(metadata) = fs::metadata(&path) {
            metadata.created().unwrap_or(SystemTime::now())
        }else {
            SystemTime::now() 
        };
        let mut last = fs::read_to_string(&path).unwrap_or("".to_owned());
        let diff = SystemTime::now().duration_since(ctime).unwrap_or(Duration::from_millis(0));
        last.push_str(format!("{},{}\n",diff.as_millis(),self.scale().scale(value)).as_str());
        fs::write(&path,last.as_bytes())?;
        self.data.push(self.scale().scale(value));
        Ok(())
    }
    pub fn signal(&self) -> Vec<Data> {
        let path = self.path().join("signal.csv");
        let mut signal:Vec<Data> = Vec::new();
        let string =  fs::read_to_string(&path).unwrap_or("".to_owned());
        let data:Vec<&str> = string.split('\n').collect();
        for val in data {
            let v:Vec<&str> = val.split(',').collect();
            if v.len() >1 {
                if let Ok(value) = v[1].parse::<f32>() {
                    let time = v[0].parse::<u64>().unwrap_or(0);
                    signal.push(Data{time,value});
                }
            }
        }
        signal
    }
    pub fn history(&self) -> Result<()> {
        let now: DateTime<Utc> = Utc::now();
        let path = self.path().join(format!( "{}.csv",now.format("%Y%m%d")));
        // if let Err(e) = wtr.write_record(&[ "timestamp", "tox", "dos", "ph", "orp", "cond","temp", "tur"]) {
            // log::error!("csv write header data - {}", e);
        // }
        let mut last = fs::read_to_string(&path).unwrap_or("".to_owned());
        last.push_str(format!("{},{}\n",now,self.value()).as_str());
        println!("next measurement in {}", self.path().display());
        fs::write(&path,last.as_bytes())?;
        Ok(())
    }
    pub fn calculate(&mut self) -> Result<()> {
        let sig = self.signal();
        if sig.len() == 0 {
            self.set_status("E");
            fs::write(self.path.join(VALUE), "none".as_bytes())?;
        } else {
            let mut sum: f32 = 0.0;
            for data in sig.as_slice() {
                sum = sum + data.value;
                // println!("VAL:{}",data.value);
            }
            let count = match sig.len() {
                positive if positive > 0 => positive,
                _ => 1,
            };
            let mean = sum / count as f32;
            let value = mean;
            println!("{} VALUE:{} COUNT:{}",self.id(),value,count);
            self.set_value(value);
            self.set_status("M");
        }
       // self.history()?;
        self.clear_data();
        Ok(())
    }
    pub fn markdown(&self) -> String {
        format!("*{}:* `{}` [{}]   *{}*\n",self.label(),self.value(),self.unit(),self.status())
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
    pub fn info(&self) -> ChanInfo {
        let label = self.label();
        let unit = self.unit();
        let value = self.value();
        let id = self.id();
        ChanInfo { id,label, unit, value }
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
            fs::write(path.join(MEASUREMENT_INTERVAL),"1000".as_bytes())?;
            fs::write(path.join(AVERAGE_INTERVAL),"600".as_bytes())?;
        }
        let interval = path.join(MEASUREMENT_INTERVAL);
        if !interval.is_file() {
            fs::write(interval,"1000".as_bytes())?;
            fs::write(path.join(AVERAGE_INTERVAL),"600".as_bytes())?;
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
    pub fn reset(&self) -> Result<()> {
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let ch = Channel::new(&path);
                ch.set_status("I");
            }
        } 
        Ok(())
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
    pub fn find_value(&self,id:&str) -> Option<f32> {
        if let Some(ch) = self.find(id) {
            ch.get_value()
        }else {
            None
        }
    }
    pub fn create(&self,id:&str) -> Channel {
        let path = self.path.join(id);
        Channel::new(&path)
    }
    pub fn get_info(&self, id: &str) -> Option<ChanInfo> {
        let path = self.path.join(id);
        if path.is_dir() {
            return Some(Channel::new(&path).info());
        }
        None
    }
    pub fn infos(&self) -> Result<Vec<ChanInfo>> {
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
            channel.set_label(label);
            channel.set_unit(unit);
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
    /// Status
    pub fn status(&self)-> String {
        fs::read_to_string(self.path.join(STATUS)).unwrap_or("E".to_owned())
    }
     /// set channel value
    pub fn set_status(&self, status:&str) -> &Self{
        if let Err(e) = fs::write(self.path().join(STATUS), status.trim().as_bytes()) {
            log::error!("channels {} change status to {} failed - {}",self.path().display(),status,e);
        }
        self
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
        std::time::Duration::from_secs(600)
    }
    /// Outliers
    /// default 0
    pub fn outliers(&self) -> u16 {
        if let Ok(outliers) = fs::read_to_string(&self.path().join(OUTLIERS)) {
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
    pub fn calculate(&mut self) -> Result<()> {
        let mut status = String::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let mut ch = Channel::new(&path);
                if let Err(e) = ch.calculate() {
                    log::error!("{} calculate failed {}",ch.path().display(),e);
                }
                status.push_str(ch.status().as_str().trim());
            }
        }
        self.set_status(status.as_str());
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
