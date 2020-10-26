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



#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct WifiConnect {
    pub ssid: String,
    pub psk: String,
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Wlan {
    pub mac: String,
    pub ssid: String,
    pub channel: String,
    pub quality: String,
    pub security: String,
    pub frequency: String,
    pub psk: String,
}

impl Wlan {
    pub fn new() -> Wlan {
        Wlan {
            mac: "".to_owned(),
            ssid: "-".to_owned(),
            channel: "-".to_owned(),
            quality: "-".to_owned(),
            security: "wpa2".to_owned(),
            frequency: "0".to_owned(),
            psk: "--".to_owned(),
        }
    }
    pub fn set_mac(&mut self, mac: &str) {
        self.mac = mac.to_owned();
    }
    pub fn set_ssid(&mut self, ssid: &str) {
        self.ssid = ssid.trim_end_matches('"').trim_start_matches('"').to_owned();
    }
    pub fn set_channel(&mut self, channel: &str) {
        self.channel = channel.to_owned();
    }
    pub fn set_quality(&mut self, quality: &str) {
        self.quality = quality.to_owned();
    }
    pub fn set_frequency(&mut self, frequency: &str) {
        self.frequency = frequency.to_owned();
    }
    pub fn set_security(&mut self, security: &str){
        self.security = security.to_owned();
    }
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
pub fn open(wms: &Workspace) -> Wifi {
    let path = wms.rootdir().join(DIR);
    Wifi { path }
}

/// Setup Wifi
pub fn setup(wms: &Workspace) -> Result<Wifi> {
    let path = wms.rootdir().join(DIR);
    let wifi = Wifi {
        path: path.to_path_buf(),
    };
    if !wifi.path.is_dir() {
        wifi.setup()?;
        fs::write(wifi.path().join(SSID), "wmnet".as_bytes())?;
        fs::write(wifi.path().join(KEY), "123456789".as_bytes())?;
        wifi.check()?;
    }
    Ok(wifi)
}
// Cell 01 - Address: 34:2C:C4:6B:07:99
// Channel:48
// Frequency:5.24 GHz (Channel 48)
// Quality=70/70  Signal level=-33 dBm  
// Encryption key:on
// ESSID:"LippenbaerCentralleBerlin"
// Bit Rates:6 Mb/s; 9 Mb/s; 12 Mb/s; 18 Mb/s; 24 Mb/s
//           36 Mb/s; 48 Mb/s; 54 Mb/s
// Mode:Master
// Extra:tsf=000000d1024bdb12
// Extra: Last beacon: 2668ms ago
// IE: Unknown: 00194C697070656E6261657243656E7472616C6C654265726C696E
// IE: Unknown: 01088C129824B048606C
// IE: Unknown: 030130
// IE: Unknown: 070A45552024081764081E00
// IE: IEEE 802.11i/WPA2 Version 1
//     Group Cipher : TKIP
//     Pairwise Ciphers (2) : CCMP TKIP
//     Authentication Suites (1) : PSK
// IE: WPA Version 1
//     Group Cipher : TKIP
//     Pairwise Ciphers (1) : TKIP
//     Authentication Suites (1) : PSK
// IE: Unknown: 2D1A6F0017FFFFFFFF00000000000058020100000000001004870100
// IE: Unknown: 3D1630070400000000000000000000000000000000000000
// IE: Unknown: 7F0900000002000000C001
// IE: Unknown: BF0C30088B03AAFF1806AAFF1806
// IE: Unknown: C005012A00FCFF

pub fn scan(iface:&str) -> Result<Vec<Wlan>> {
    // sudo iwlist wlan0 scan
    let mut wlans: Vec<Wlan> = Vec::new();
    let output = std::process::Command::new("iwlist")
        .arg(iface)
        .arg("scan")
        .output()?;
    let list = String::from_utf8_lossy(&output.stdout);
    // println!("SCAN{}", &list);
    let data: Vec<&str> = list.split('\n').collect();
    let mut wlan = Wlan::new();
    for s in data {
        let par = s.trim_start().to_owned();
        // println!("Par:{}",&par);
        let val: Vec<&str> = par.splitn(2,':').collect();
        if par.starts_with("Cell") {
            if val.len()>1 {
                wlan.set_mac(val[1].trim_start());
            } 
            if wlan.ssid != "-" {
                wlans.push(wlan.clone());
            }
            wlan = Wlan::new();
        } else if par.starts_with("Quality") {
            wlan.set_quality(s);
        }
        else {
            if val.len() > 1 {
                match val[0] {
                    "ESSID" => wlan.set_ssid(val[1]),
                    "Channel" => wlan.set_channel(val[1]),
                    "Frequency" => wlan.set_frequency(val[1]),
                    _ => (),
                }
            }
        }
        
    }
    Ok(wlans)
}

pub fn connect(wlan: &WifiConnect) -> Result<()> {
    let output = std::process::Command::new("wpa_passphrase")
        .arg(&wlan.ssid)
        .arg(&wlan.psk)
        .output()?;
    log::info!("wpa credential:{}",String::from_utf8_lossy(&output.stdout));
    // fs::write(self.path().join(SSID), ssid.as_bytes())?;
    // fs::write(self.path().join(KEY), key.as_bytes())?;
    // fs::write(self.path().join(CONF), output.stdout)?;
    Ok(())
}

pub fn current() -> Result<Wlan> {
    let wlan = Wlan::new();
    Ok(wlan) 
}