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

const APIKEY: &str = "apikey";
const INTERVAL: &str = "interval";

/// Fb fs interface
#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleCheets {
    path: PathBuf,
}
impl Class for GoogleCheets {
    const META: &'static str = "googlecheets";
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Property for GoogleCheets {}

impl GoogleCheets {
    /// apikey
    pub fn apikey(&self) -> String {
        fs::read_to_string(self.path.join(APIKEY)).unwrap_or("nokey".to_owned())
    }
    /// station uuid in firebase
    pub fn uuid(&self) -> String {
        fs::read_to_string(self.path.join(UUID)).unwrap_or("nokey".to_owned())
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

pub fn open(ws: &Workspace) -> GoogleCheets {
    let path = ws.rootdir().join(GoogleCheets::META);
    GoogleCheets { path }
}

pub fn setup(ws: &Workspace) -> Result<GoogleCheets> {
    let path = ws.rootdir().join(GoogleCheets::META);
    let fb = GoogleCheets { path };
    if !fb.path.is_dir() {
        fb.setup()?;
    }
    Ok(fb)
}
