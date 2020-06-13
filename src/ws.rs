use crate::config::Config;
use crate::Result;
use crate::{channel::Channels, inky::Inky, network::Network, web::Web};
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
    pub fn inky(&self) -> Inky {
        crate::inky::open(self)
    }
}

pub fn setup() -> Result<Workspace> {
    let dir = crate::workdir();
    log::info!("workspace[{}] setup", dir.display());
    if dir.is_relative() {
        if let Err(err) = dir.canonicalize() {
            log::error!("workspace:{} need absolute path - {}", dir.display(), err);
        }
    }
    let ws = Workspace {
        path: dir.to_path_buf(),
    };
    if !ws.path.is_dir() {
        ws.setup()?;

        // fs::create_dir_all(&ws.path)?;
        let config = Config::default();
        ws.write_config(&config)?;

        // if let Err(err) = network::setup(&ws) {
        //     log::error!("workspace[{}] setup network - {}", ws.path.display(), err);
        // }
        // if let Err(err) = web::setup(&ws) {
        //     log::error!("workspace[{}] setup network - {}", ws.path.display(), err);
        // }
        // if let Err(err) = channel::setup(&ws) {
        //     log::error!(
        //         "workspace[{}] new channel superviser - {}",
        //         ws.path.display(),
        //         err
        //     );
        // }
        if let Err(err) = Repository::init(&ws.path) {
            log::error!("workspace[{}] init git error - {}", ws.path.display(), err)
        }
    }
    log::info!("workspace[{}] setup channels", dir.display());
    crate::channel::setup(&ws)?;
    log::info!("workspace[{}] setup network", dir.display());
    crate::network::setup(&ws)?;
    crate::inky::setup(&ws)?;
    crate::web::setup(&ws)?;
    Ok(ws)
}
