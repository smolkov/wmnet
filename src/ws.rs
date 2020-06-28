use crate::config::Config;
use crate::Result;
use crate::{channel::Channels, network::Network, web::Web, wifi::Wifi};
use git2::Repository;
// use serde::{Deserialize, Serialize};
// use std::fmt;
use crate::{Class, Property};
// use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE: &str = "wqms.toml";

pub struct Workspace {
    path: PathBuf,
}

impl Class for Workspace {
    const META: &'static str = "workspace";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Property for Workspace {}

impl Workspace {
    pub fn canonicalize(&mut self) -> Result<()> {
        if self.path.is_relative() {
            if let Err(err) = self.path.canonicalize() {
                log::error!(
                    "workspace:{} need absolute path - {}",
                    self.path.display(),
                    err
                );
            }
        }
        Ok(())
    }
    pub fn read_config(&self) -> Result<Config> {
        let cfg = Config::load(&self.path.join("config"))?;
        Ok(cfg)
    }
    pub fn write_config(&self, config: &Config) -> Result<()> {
        config.save(&self.path.join(CONFIG_FILE))?;
        Ok(())
    }
    pub fn rootdir(&self) -> &Path {
        &self.path
    }
    pub fn workspace(&self) -> Workspace {
        Workspace {
            path: self.path.to_path_buf(),
        }
    }
    pub fn channels(&self) -> Channels {
        crate::channel::open(self)
    }
    pub fn web(&self) -> Web {
        crate::web::open(self)
    }
    pub fn network(&self) -> Network {
        crate::network::open(self)
    }
    pub fn wifi(&self) -> Wifi {
        crate::wifi::open(self)
    }
}

pub fn setup() -> Result<Workspace> {
    let dir = crate::workdir();
    let mut ws = Workspace {
        path: dir.to_path_buf(),
    };
    crate::logger::setup(&ws)?;
    log::info!("workspace[{}] setup", dir.display());
    ws.canonicalize()?;
    if !ws.path.is_dir() {
        ws.setup()?;
        let config = Config::default();
        ws.write_config(&config)?;
        if let Err(err) = Repository::init(&ws.path) {
            log::error!("workspace[{}] init git error - {}", ws.path.display(), err)
        }
    }
    log::info!("workspace[{}] setup channels", dir.display());
    crate::channel::setup(&ws)?;
    log::info!("workspace[{}] setup network", dir.display());
    crate::network::setup(&ws)?;
    log::info!("workspace[{}] setup wifi", dir.display());
    crate::wifi::setup(&ws)?;
    log::info!("workspace[{}] setup web", dir.display());
    crate::web::setup(&ws)?;
    Ok(ws)
}

pub fn open() -> Workspace {
    let dir = crate::workdir();
    Workspace {
        path: dir.to_path_buf(),
    }
}
