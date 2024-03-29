// use crate::network;
use crate::Result;
use crate::Workspace;
use serde::{Deserialize, Serialize};
// use std::sync::mpsc::{self, channel};
// use std::thread;
use crate::iface::{Class, Property};
// use crossbeam::channel::{select, tick};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
// use systemstat::{Platform, System};
/// IPFILE name
pub const NETDIR: &'static str = "net";
pub const INTERVAL: &'static str = "refresh_interval";
pub const IFACE: &'static str = "iface";
pub const HOST: &'static str = "host";
pub const STATUS: &'static str = "status";

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum State {
    Offline,
    Online,
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            0 => State::Offline,
            1 => State::Online,
            _ => State::Offline,
        }
    }
}

impl From<State> for u8 {
    fn from(state: State) -> u8 {
        state.into()
    }
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        match value {
            "offline" => State::Offline,
            "online" => State::Online,
            _ => State::Offline,
        }
    }
}

impl From<String> for State {
    fn from(value: String) -> Self {
        State::from(value.as_str())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Offline => return write!(f, "offline"),
            State::Online => return write!(f, "online"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NetInfo {
    pub status: State,
    pub addres: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NetStatus {
    pub state: State,
    pub info: NetInfo,
}

pub struct Network {
    path: PathBuf,
}
impl Class for Network {
    const META: &'static str = "network";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Property for Network {}
// impl Lan for Network {}

impl Network {
    pub fn new(wms: &Workspace) -> Result<Network> {
        // Network::setup(&wms.rootdir().join(NETDIR));
        let path = wms.rootdir().join(NETDIR);
        let net = Network {
            path: path.to_path_buf(),
        };
        if !net.path.is_dir() {
            fs::create_dir_all(&path)?;
        }
        Ok(net)
    }
    ///read refresh interval in second
    pub fn get_interval(&self) -> std::time::Duration {
        let sec = fs::read_to_string(self.path.join(INTERVAL))
            .unwrap_or("5".to_owned())
            .parse::<u64>()
            .unwrap_or(5);
        std::time::Duration::from_secs(sec)
    }
    ///read refresh interval in second
    pub fn set_interval(&self, seconds: u64) -> Result<()> {
        fs::write(self.path.join(INTERVAL), format!("{}", seconds).as_bytes())?;
        Ok(())
    }
    ///read refresh interval in second
    // pub fn get_interface(&self) -> String {
    // let interface = fs::read_to_string(self.path.join(IFACE)).unwrap_or(find_interface());
    // interface
    // }
    ///read refresh interval in second
    pub fn set_interface(&self, interface: &str) -> Result<()> {
        fs::write(self.path.join(IFACE), interface.as_bytes())?;
        Ok(())
    }
    
}

/// Open network directory
pub fn open(wms: &Workspace) -> Network {
    let path = wms.rootdir().join(NETDIR);
    Network { path }
}
/// Setup Network
pub fn setup(wms: &Workspace) -> Result<Network> {
    let path = wms.rootdir().join(NETDIR);
    let net = Network {
        path: path.to_path_buf(),
    };
    if !path.is_dir() {
        net.setup()?;
        net.set_interval(5)?;
        // net.set_wpasupplicant("wmnet", "SeiBereit")?;
        // net.set_wpa("wmnet-setup".to_owned(),"SeiBereit".to_owned())?;
    }
    // static ref SENDER:mpsc::Sender<State> = {
    // let (sender, receiver) = channel();
    // thread::spawn(move || {
    // let network = Network{path:path.clone()};
    // let mut ticks = tick(network.get_interval());
    // let mut ip = String::from("none");
    // loop {
    // select! {
    // recv(ticks) -> _ => {
    // network.ping();
    // }
    // }
    // }
    // });
    // sender.clone()
    // };}
    Ok(net)
}

pub fn status() -> String {
    match std::process::Command::new("ping")
    .arg("-c 1")
    .arg("google.de")
    .status()
    {
        Ok(status) => {
            if status.success() {
                " online".to_owned()
            } else {
                "offline".to_owned()
            }
        }
        Err(err) => {
            log::error!("update networt state - {}", err);
            "offline".to_owned()
        }
    } 
}
pub fn hostname() -> String {
    let hostname = match std::process::Command::new("hostname").arg("-I").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap_or("none".to_owned()),
        Err(_) => String::from("none"),
    };
    hostname
}