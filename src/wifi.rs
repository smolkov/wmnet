// use crate::network;
use crate::Result;
use crate::Workspace;
use serde::{Deserialize, Serialize};
// use std::sync::mpsc::{self, channel};
// use std::thread;
use crate::iface::Class;
// use crossbeam::channel::{select, tick};
// use std::fmt;
use crate::util::name_from_path;
use std::fs;
use std::path::{Path, PathBuf};
// use systemstat::{Platform, System};
const DIR: &'static str = "wifi";
// const INTERVAL: &'static str = "refresh_interval";
const IFACE: &'static str = "iface";
const SSID: &'static str = ".ssid";
const KEY: &'static str = ".key";
const CONF: &'static str = ".conf";

const WIRELESS: &'static str = "wireless";

#[derive(Debug, Serialize, Deserialize)]
pub struct Wlan{
    pub mac: String,
    pub ssid: String,
    pub channel: String,
    pub signal_level: String,
    pub security: String,
    pub psk: String,
}


pub struct Wifi {
    path: PathBuf,
}

impl Class for Wifi {
    const META: &'static str = "wifi";
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Wifi {

    pub fn check(&self) -> Result<()> {
        // if cfg!(target_os = "linux") && !which::which("wpa_passphrase").is_ok() {
        // std::process::Command::new("apt")
        // .arg("install")
        // .arg("wpasupplicant")
        // .output()?;
        // }
        Ok(())
    }
    pub fn ssid(&self) -> String {
        fs::read_to_string(self.path().join(SSID)).unwrap_or("NOSET".to_owned())
    }
    pub fn key(&self) -> String {
        fs::read_to_string(self.path().join(KEY)).unwrap_or("NOSET".to_owned())
    }
    pub fn credentials(&self, ssid: &str, key: &str) -> Result<()> {
        let output = std::process::Command::new("wpa_passphrase")
            .arg(ssid)
            .arg(key)
            .output()?;
        fs::write(self.path().join(SSID), ssid.as_bytes())?;
        fs::write(self.path().join(KEY), key.as_bytes())?;
        fs::write(self.path().join(CONF), output.stdout)?;
        Ok(())
    }
    pub fn interfaces(&self) -> Result<Vec<String>> {
        let mut list: Vec<String> = Vec::new();
        for entry in fs::read_dir("/sys/class/net")? {
            let entry = entry?;
            let path = entry.path();
            if path.join(WIRELESS).is_dir() {
                list.push(name_from_path(&path));
            }
        }
        Ok(list)
    }

    pub fn interface(&self) -> String {
        fs::read_to_string(self.path().join(IFACE)).unwrap_or("none".to_owned())
    }
    pub fn change_interface(&self, iface: &str) -> Result<()> {
        if self.interface() != iface {
            fs::write(self.path().join(IFACE), iface)?;
        }
        Ok(())
    }
    pub fn connect(&self) -> Result<()> {
        let cfg = self.path().join(CONF);
        std::process::Command::new("wpa_supplicant")
            .current_dir(&self.path())
            .arg("-B")
            .arg("-i")
            .arg(&self.interface())
            .arg("-c")
            .arg(&cfg)
            .spawn()?;
        Ok(())
    }
    pub fn scan_networks(&self) -> Result<String> {
        let output = std::process::Command::new("iwlist")
            .arg(&self.interface())
            .arg("scan")
            .output()?;
        let list = format!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(list)
    }
   
}

/// Open network directory
pub fn open(ws: &Workspace) -> Wifi {
    let path = ws.rootdir().join(DIR);
    Wifi { path }
}

/// Setup Wifi
pub fn setup(ws: &Workspace) -> Result<Wifi> {
    let path = ws.rootdir().join(DIR);
    let wifi = Wifi {
        path: path.to_path_buf(),
    };
    if !wifi.path.is_dir() {
        wifi.setup()?;
        fs::write(wifi.path().join(SSID), "wqms".as_bytes())?;
        fs::write(wifi.path().join(KEY), "123456789".as_bytes())?;
        wifi.check()?;
    }
    Ok(wifi)
}

pub fn scan() -> Vec<Wlan> {
    let mut wlans: Vec<Wlan> = Vec::new();
    let nets = wifiscanner::scan().unwrap();
    println!("{:?}",&nets);
    for w in nets.iter() {
        wlans.push(Wlan{
            mac:w.mac.to_owned(),
            ssid:w.ssid.to_owned(),
            channel: w.channel.to_owned(),
            signal_level: w.signal_level.to_owned(),
            security: w.security.to_owned(),
            psk:"".to_owned(),
        })
    }
    wlans
}


pub fn connect(wlan:&Wlan) -> Result<()> {
    let output = std::process::Command::new("wpa_passphrase")
        .arg(&wlan.ssid)
        .arg(&wlan.psk)
        .output()?;
    // fs::write(self.path().join(SSID), ssid.as_bytes())?;
    // fs::write(self.path().join(KEY), key.as_bytes())?;
    // fs::write(self.path().join(CONF), output.stdout)?;
    Ok(())
}