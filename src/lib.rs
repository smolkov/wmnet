pub mod channel;
pub mod cli;
pub mod config;
pub mod error;
pub mod fb;
pub mod interface;
pub mod logger;
pub mod network;
pub mod setting;
pub mod station;
pub mod store;
pub mod thingspeak;
pub mod telegram;
pub mod util;
pub mod web;
pub mod wifi;
pub mod ws;
pub mod jumo;
pub mod nitri;
pub mod analog;
pub use error::Error;
/// iotnode library.
///
///
pub use interface::{Class, Property, Statistic};
pub type Result<T> = std::result::Result<T, Error>;
pub use config::Config;
pub use ws::Workspace;
pub use thingspeak::ThingSpeak;


pub mod iface {
    pub use super::channel::{Channel, Channels};
    pub use super::network::Network;
    pub use super::setting::Settings;
    pub use super::web::Web;
    pub use super::wifi::Wifi;
    pub use super::{Class, Property};
}

