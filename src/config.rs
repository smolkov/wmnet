use super::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
// use std::str::FromStr;

pub const DEFAULT_CONF: &str = include_str!("../wqms.toml");
const TSWKEY: &str = "RQ1HTKE735B65NVI";
const TSRKEY: &str = "XZUIDN95GI2ZOSBX";
const TSCHANNEL: &str = "1114700";

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseConfig {
    pub url: String,
    pub key: String,
    pub uid: String,
}

impl Default for FirebaseConfig {
    fn default() -> Self {
        FirebaseConfig {
            key: "-".to_owned(),
            url: "-".to_owned(),
            uid: ".".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TingsSpeakConfig {
    pub channel: String,
    pub rkey: String,
    pub wkey: String,
}

impl Default for TingsSpeakConfig {
    fn default() -> Self {
        TingsSpeakConfig {
            channel: TSCHANNEL.to_owned(), 
            rkey: TSRKEY.to_owned(), 
            wkey: TSWKEY.to_owned(), 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub token: String,
    pub url: String,
}

impl Default for GitConfig {
    fn default() -> Self {
        GitConfig {
            token: "-".to_owned(),
            url: "-".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticConfig {
    /// Collect interval time in second
    pub interval: u64,
    /// Count of outlier
    pub outlier: u8,
    /// Korelationkoefizent
    pub cv: u8,
}

impl Default for StatisticConfig {
    fn default() -> Self {
        StatisticConfig {
            interval: 30,
            outlier: 0,
            cv: 2,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}
impl ServerConfig {
    // pub fn port(&self) -> u16 {
    // self.port
    // }
}
impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig { port: 8088 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub interface: String,
    pub thingspeak: TingsSpeakConfig,
    pub statistic: StatisticConfig,
    pub server: ServerConfig,
    pub fb: Option<FirebaseConfig>,
    pub git: Option<GitConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "noname".to_owned(),
            interface: "wlp2s0".to_owned(),
            thingspeak: TingsSpeakConfig::default(),
            statistic: StatisticConfig::default(),
            server: ServerConfig::default(),
            fb: Some(FirebaseConfig::default()),
            git: None,
        }
    }
}

pub fn bad_working_staton_directory_error(path: &Path) -> Error {
    Error::BadWorkstation {
        msg: format!(
            r#"
A configuration {} file is not exists!
to create new working station run `awm new sation-name`
rerun last command in `station-name` directory
"#,
            path.display()
        ),
    }
}

impl Config {
    /// Creates a new config instance.
    ///
    /// This is typically used for tests or other special cases. `default` is
    /// preferred otherwise.
    ///
    /// This does only minimal initialization. In particular, it does not load
    /// any config files from disk. Those will be loaded lazily as-needed.
    pub fn read(path: &Path) -> Result<Config> {
        if !path.is_file() {
            return Err(bad_working_staton_directory_error(&path));
        }
        let toml_str = fs::read_to_string(&path)?;
        // let config: Config::from(&toml_str)?;
        let config: Config = toml::from_str(&toml_str)?;
        Ok(config)
    }
    pub fn write(&self, path: &Path) -> Result<()> {
        let toml_str = toml::to_string(self)?;
        fs::write(path, toml_str)?;
        Ok(())
    }
}

// impl FromStr for Config {
//     type Err = Error;
//     /// Load a GPIO configuration for the provided toml string
//     fn from_str(config: &str) -> Result<Self> {
//         let cfg = toml::from_str(&config);
//         match cfg {
//             Ok(cfg) => {
//                 let val_config: Config = toml::from_str(&config).unwrap();
//                 // val_config.validate()?;
//                 Ok(cfg)
//             }
//             Err(e) => Err(Error::ParserErrors(e)),
//         }
//     }
// }
