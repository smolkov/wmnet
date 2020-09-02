use crate::config::Config;
use crate::Result;
use crate::{channel::Channels,ThingSpeak,telegram::Telegram, network::Network, web::Web, wifi::Wifi};
use git2::Repository;
use std::env;

// use serde::{Deserialize, Serialize};
// use std::fmt;
// use crate::{Class, Property};
// use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE: &str = "wqms.toml";

pub struct Workspace {
    path: PathBuf,
}

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
    pub fn config(&self) -> Config {
        Config::read(&self.path.join("wqms.toml")).unwrap_or(Config::default())
    }
    pub fn write_config(&self, config: &Config) -> Result<()> {
        config.write(&self.path.join(".toml"))?;
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
    pub fn channels(&self) -> Result<Channels> {
        crate::channel::setup(self)
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
    pub fn thingspeak(&self) -> Result<ThingSpeak> {
        crate::thingspeak::setup(self)
    }
    pub fn telegram(&self) -> Result<Telegram> {
        crate::telegram::setup(self)
    }
}

const WQMS_PATH: &'static str = "WQMS_DIR";
pub fn rootpath() -> PathBuf {
    if let Ok(wqmsdir) = env::var(WQMS_PATH) {
        return PathBuf::from(wqmsdir);
    } else if let Ok(homedir) = env::var("HOME") {
        return PathBuf::from(homedir).join(".wqms");
    }
    PathBuf::from("./.wqms")
}


/// Setup new workspace
pub fn setup(ws: &mut Workspace) -> Result<()>{
    ws.canonicalize()?;
    if !ws.path.is_dir() {
        log::info!("workspace[{}] setup", ws.path.display());
        let config = Config::default();
        ws.write_config(&config)?;
        if let Err(err) = Repository::init(&ws.path) {
            log::error!("workspace[{}] init git error - {}", ws.path.display(), err)
        }
    }
    // crate::channel::setup(ws)?;
    // crate::network::setup(ws)?;
    // crate::wifi::setup(ws)?;
    // crate::web::setup(ws)?;
    Ok(())
}

pub fn open(path:&Path) -> Workspace {
    let mut ws = Workspace {
        path: path.to_path_buf(),
    }; 
    if let Err(e) = setup(&mut ws)  {
        log::error!("setup workspace error {}",e);
    }
    ws
}

pub fn root() ->  Workspace {
    let mut ws = Workspace {
        path:  rootpath(),
    }; 
    if let Err(e) = setup(&mut ws)  {
        log::error!("setup workspace error {}",e);
    }
    ws
}
