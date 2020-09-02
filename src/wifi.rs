// use crate::network;
use crate::Result;
use crate::Workspace;
// use serde::{Deserialize, Serialize};
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

pub struct Wifi {
    path: PathBuf,
}

impl Class for Wifi {
    const META: &'static str = "wifi";
    fn path(&self) -> &Path {
        &self.path
    }
}

pub trait Wpa: Class {
    fn check(&self) -> Result<()> {
        // if cfg!(target_os = "linux") && !which::which("wpa_passphrase").is_ok() {
        // std::process::Command::new("apt")
        // .arg("install")
        // .arg("wpasupplicant")
        // .output()?;
        // }
        Ok(())
    }
    fn ssid(&self) -> String {
        fs::read_to_string(self.path().join(SSID)).unwrap_or("NOSET".to_owned())
    }
    fn key(&self) -> String {
        fs::read_to_string(self.path().join(KEY)).unwrap_or("NOSET".to_owned())
    }
    fn credentials(&self, ssid: &str, key: &str) -> Result<()> {
        let output = std::process::Command::new("wpa_passphrase")
            .arg(ssid)
            .arg(key)
            .output()?;
        fs::write(self.path().join(SSID), ssid.as_bytes())?;
        fs::write(self.path().join(KEY), key.as_bytes())?;
        fs::write(self.path().join(CONF), output.stdout)?;
        Ok(())
    }
    fn interfaces(&self) -> Result<Vec<String>> {
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

    fn interface(&self) -> String {
        fs::read_to_string(self.path().join(IFACE)).unwrap_or("none".to_owned())
    }
    fn change_interface(&self, iface: &str) -> Result<()> {
        if self.interface() != iface {
            fs::write(self.path().join(IFACE), iface)?;
        }
        Ok(())
    }
    fn connect(&self) -> Result<()> {
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
    fn scan_networks(&self) -> Result<()> {
        let output = std::process::Command::new("iwlist")
            .arg(&self.interface())
            .arg("scan")
            .output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}

// impl Lan for Network {}
impl Wpa for Wifi {}

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
