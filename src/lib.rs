pub mod channel;
pub mod cli;
pub mod config;
pub mod error;
pub mod fb;
pub mod inky;
pub mod interface;
pub mod logger;
pub mod network;
pub mod setting;
pub mod station;
pub mod store;
pub mod util;
pub mod web;
pub mod wifi;
pub mod ws;
pub use error::Error;
/// iotnode library.
///
///
pub use interface::{Class, Property, Statistic};
pub type Result<T> = std::result::Result<T, Error>;
pub use config::Config;
pub use ws::Workspace;

pub const WQMS_PATH: &'static str = "WQMS_DIR";
use std::env;
use std::path::PathBuf;
pub fn workdir() -> PathBuf {
    if let Ok(wqmsdir) = env::var(WQMS_PATH) {
        return PathBuf::from(wqmsdir);
    } else if let Ok(homedir) = env::var("HOME") {
        return PathBuf::from(homedir).join(".wqms");
    }
    PathBuf::from("./.wqms")
}

pub fn setup() -> Result<Workspace> {
    let workspace = ws::setup()?;
    // store::setup(&workspace)?;
    Ok(workspace)
}

pub mod iface {
    pub use super::channel::{Chan, Channel, Channels};
    pub use super::network::Network;
    pub use super::setting::Settings;
    pub use super::web::Web;
    pub use super::wifi::Wifi;
    pub use super::{Class, Property};
}
