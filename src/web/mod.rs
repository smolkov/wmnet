use crate::Result;
use crate::Workspace;
use crate::{Class, Property};
use git2::Repository;
// use serde::{Deserialize, Serialize};
// use std::sync::mpsc::{self, channel};
// use std::thread;

// use crossbeam::channel::{select, tick};
// use std::fmt;
use std::fs;
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

pub struct Www {
    pub token: String,
    pub url: String,
}

pub struct Web {
    path: PathBuf,
}
impl Class for Web {
    const META: &'static str = "web";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Property for Web {}

impl Web {
    pub fn new(ws: &Workspace) -> Result<Web> {
        let web = Web {
            path: ws.rootdir().join(Web::META),
        };
        Ok(web)
    }
    pub fn www(&self) -> PathBuf {
        self.path.join("www")
    }
    pub fn upgrade(&self) -> Result<()> {
        Ok(())
    }
}
pub fn open(ws: &Workspace) -> Web {
    let web = Web {
        path: ws.rootdir().join(Web::META),
    };
    web
}

pub fn setup(ws: &Workspace) -> Result<Web> {
    let path = ws.rootdir().join(Web::META);
    let web = Web {
        path: path.to_path_buf(),
    };
    if !web.path.is_dir() {
        web.setup()?;
        let wwwdir = web.www();
        fs::create_dir_all(&wwwdir)?;
        if let Err(err) = Repository::init(&wwwdir) {
            log::error!(
                "workspace[{}] init git repository {} - {}",
                ws.rootdir().display(),
                wwwdir.display(),
                err
            )
        }

        // net.set_wpa("wqms-setup".to_owned(),"SeiBereit".to_owned())?;
    }
    Ok(web)
}
