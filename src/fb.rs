use crate::Result;
use crate::Workspace;
use crate::{Class, Property};
use serde::{Deserialize, Serialize};
use std::time::Duration;
// use std::fmt::Display;
use std::fs;
// use std::fs::FileType;
// use std::io::prelude::*;
// use chrono::{DateTime, Utc};
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufWriter;
// use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
// use std::sync::Mutex;
// use std::time::SystemTime;

const APIKEY: &str = ".apikey";
const UUID: &str = "uuid";
const URL: &str = "url";
const INTERVAL: &str = "interval";

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

/// Fb fs interface
#[derive(Debug, Serialize, Deserialize)]
pub struct Fb {
    path: PathBuf,
}
impl Class for Fb {
    const META: &'static str = "firebase";
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Property for Fb {}

impl Fb {
    /// apikey
    pub fn apikey(&self) -> String {
        fs::read_to_string(self.path.join(APIKEY))
            .unwrap_or("AIzaSyAIn-DA1KrMa6jaIiR6w-EO7SQ9cMXffw8".to_owned())
    }
    /// station uuid in firebase
    pub fn uuid(&self) -> String {
        fs::read_to_string(self.path.join(UUID)).unwrap_or("nokey".to_owned())
    }
    /// apikey
    pub fn url(&self) -> String {
        fs::read_to_string(self.path.join(URL)).unwrap_or("".to_owned())
    }
    /// Collect interval
    /// default 60 seconds
    pub fn interval(&self) -> Duration {
        if let Ok(sec) = fs::read_to_string(self.path.join(INTERVAL)) {
            if let Ok(sec) = sec.parse::<u64>() {
                return Duration::from_secs(sec);
            }
        }
        Duration::from_secs(60)
    }
}

pub fn open(ws: &Workspace) -> Fb {
    let path = ws.rootdir().join(Fb::META);
    Fb { path }
}

pub fn setup(ws: &Workspace) -> Result<Fb> {
    let path = ws.rootdir().join(Fb::META);
    let fb = Fb { path };
    if !fb.path.is_dir() {
        fb.setup()?;
    }
    Ok(fb)
}
