use crate::Result;
use crate::{Class, Property, Workspace};
// use serde::{Deserialize, Serialize};
// use std::sync::mpsc::{self, channel};
// use std::thread;

// use crossbeam::channel::{select, tick};
// use std::fmt;
// use std::fs;
use std::path::{Path, PathBuf};
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

pub struct Settings {
    path: PathBuf,
}

impl Class for Settings {
    const META: &'static str = "settings";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Property for Settings {}



pub fn setup(ws: &Workspace) -> Result<Settings> {
    let path = ws.rootdir().join(Settings::META);
    let settings = Settings {
        path: path.to_path_buf(),
    };
    if !path.is_dir() {
        settings.setup()?;
    }
    Ok(settings)
}


