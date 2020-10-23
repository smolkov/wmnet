use crate::config::Config;
use crate::Result;
use crate::{channel::Channels,ThingSpeak,telegram::Telegram, web::Web, wifi::Wifi};
use git2::Repository;
use std::env;

// use serde::{Deserialize, Serialize};
// use std::fmt;
// use crate::{Class, Property};
use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE: &str = "wmnet.toml";
pub const DEVNAME: &str = "name";

#[derive(Clone)]
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
    pub fn name(&self) -> String {
        fs::read_to_string(self.path.join(DEVNAME)).unwrap_or("unknown".to_owned())
    }
    pub fn set_name(&self,device: &str) -> Result<()> {
        fs::write(self.path.join(DEVNAME), device.trim().as_bytes())?;
        Ok(())
    }
    pub fn truncate(&self,path:&Path) -> PathBuf {
        if let Ok(p)= path.strip_prefix(&self.path) {
            p.to_path_buf()
        }else {
            path.to_path_buf()
        }
    }
    pub fn config(&self) -> Config {
        Config::read(&self.path.join("wmnet.toml")).unwrap_or(Config::default())
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
    pub fn wifi(&self) -> Wifi {
        crate::wifi::open(self)
    }
    pub fn thingspeak(&self) -> Result<ThingSpeak> {
        crate::thingspeak::setup(self)
    }
    pub fn telegram(&self) -> Result<Telegram> {
        crate::telegram::setup(self)
    }
    pub fn latitude(&self) -> f32 {
        let lat = fs::read_to_string(self.path.join("latitude")).unwrap_or("0.0".to_owned());
        lat.parse::<f32>().unwrap_or(0.0)
    }
    pub fn longitude(&self) -> f32 {
        let lat = fs::read_to_string(self.path.join("longitude")).unwrap_or("0.0".to_owned());
        lat.parse::<f32>().unwrap_or(0.0)
    }
    pub fn set_longitude(&self,long:&str) -> &Self {
        if let Err(e) = fs::write(self.path.join("longitude"), long.trim().as_bytes()) {
            log::error!("WS set longetude failed - {}",e);
        }
        self
    }
    pub fn set_latitude(&self,lat:&str) -> &Self {
        if let Err(e) = fs::write(self.path.join("latitude"), lat.trim().as_bytes()) {
            log::error!("WS set longetude failed - {}",e);
        }
        self
    }
}

const WQMS_PATH: &'static str = "WQMS_DIR";
pub fn rootpath() -> PathBuf {
    if let Ok(wqmsdir) = env::var(WQMS_PATH) {
        return PathBuf::from(wqmsdir);
    } else if let Ok(homedir) = env::var("HOME") {
        return PathBuf::from(homedir).join(".wmnet");
    }
    PathBuf::from("./.wmnet")
}


/// Setup new workspace
pub fn setup(wms: &mut Workspace) -> Result<()>{
    wms.canonicalize()?;
    if !wms.path.is_dir() {
        log::info!("workspace[{}] setup", wms.path.display());
        fs::create_dir_all(&wms.path)?;
        let _config = Config::default();
        // wms.write_config(&config)?;
        if let Err(err) = Repository::init(&wms.path) {
            log::error!("workspace[{}] init git error - {}", wms.path.display(), err)
        }
    }
    // crate::channel::setup(wms)?;
    // crate::network::setup(wms)?;
    // crate::wifi::setup(wms)?;
    // crate::web::setup(wms)?;
    Ok(())
}

pub fn open(path:&Path) -> Workspace {
    let mut wms = Workspace {
        path: path.to_path_buf(),
    }; 
    if let Err(e) = setup(&mut wms)  {
        log::error!("setup workspace error {}",e);
    }
    wms
}

pub fn default() ->  Workspace {
    let mut wms = Workspace {
        path:  rootpath(),
    }; 
    if let Err(e) = setup(&mut wms)  {
        log::error!("setup workspace error {}",e);
    }
    wms
}
