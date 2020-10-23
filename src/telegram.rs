use crate::Result;
use crate::{Workspace};
use std::fs;
// use std::path::{Path};
// use serde::{Deserialize, Serialize};
// use chrono::{Utc};

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
const APIKEY: &str = "none:none";



pub struct Telegram {
    path: PathBuf,
}
// pub struct Cmd {
    // path: PathBuf,

// }

impl Telegram {
    pub fn token(&self) -> String {
        fs::read_to_string(self.path.join(".token")).unwrap_or(APIKEY.to_owned())
    }
    pub fn set_token(&self,token:&str) -> &Self {
        if let Err(e) = fs::write(self.path.join(".token"), token.trim().as_bytes()) {
            log::error!("TELEGRAM BOT change token failed - {}",e);
        }
        self
    }
    pub fn state(&self) -> PathBuf {
        let path = self.path.join("state");
        if path.is_dir() {
            return path;
        }
        self.path.to_path_buf()
    }
   
    pub fn leave(&self) -> Result<()> {
        let path = self.path.join("state");
        std::fs::remove_file(path)?;
        Ok(())
    }
}

pub fn setup(wms: &Workspace) -> Result<Telegram> {
    let path = wms.rootdir().join("telegram");
    let telegram = Telegram {
        path: path.to_path_buf(),
    };
    if !telegram.path.is_dir() {
        log::info!("Create telegram bot directory {}",telegram.path.as_path().display());
        fs::create_dir_all(&telegram.path)?;
        telegram.set_token(APIKEY);
    }
    Ok(telegram)
}


pub fn open() -> Result<Telegram> {
    setup(&crate::wms::default())
}