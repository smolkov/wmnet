#![allow(unused_variables)]

use failure::Fail;
use ron;
use std::fmt;
use std::io;
use toml;

// use std::io::ErrorKind;
use std::num::ParseFloatError;
use std::num::ParseIntError;
// use std::path::Path;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "io error - {}", err)]
    IOError { err: io::Error },
    #[fail(display = "redis error - {}", err)]
    RedisError { err: redis::RedisError },
    #[fail(display = "str utf8 error - {}", err)]
    Utf8Error { err: std::str::Utf8Error },
    #[fail(display = "bad json error - {}", err)]
    BadJsonData { err: serde_json::Error },
    #[fail(display = "bad toml error - {}", err)]
    BadTomlData { err: toml::de::Error },
    #[fail(display = "serialize toml error - {}", err)]
    SerializeTomlError { err: toml::ser::Error },
    #[fail(display = "bad ron error - {}", err)]
    RonError { err: ron::Error },
    #[fail(display = "fmt error - {}", err)]
    FormatError { err: fmt::Error },
    #[fail(display = "async error - {}", err)]
    AsyncError { err: std::io::Error },
    #[fail(display = "interface - {}", msg)]
    Interface { msg: String },
    #[fail(display = "working station error - {}", msg)]
    BadWorkstation { msg: String },
    #[fail(display = "parce int error - {}", err)]
    ConvertInt { err: std::num::ParseIntError },
    #[fail(display = "parse float error - {}", err)]
    ConvertFloat { err: ParseFloatError },
    #[fail(display = "request error - {}", err)]
    RequestError { err:  reqwest::Error },
   
}

pub fn bad_workstation(msg: String) -> Error {
    Error::BadWorkstation { msg }
}

pub fn parce_data(msg: String) -> Error {
    Error::BadWorkstation { msg }
}

// pub fn driver_timeout(msg:String) -> Error {
// Error::DriverTimeout{msg}
// }

impl From<io::Error> for Error {
    fn from(kind: io::Error) -> Error {
        Error::IOError { err: kind }
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Utf8Error { err }
    }
}

impl From<redis::RedisError> for Error {
    fn from(error: redis::RedisError) -> Error {
        Error::RedisError { err: error }
    }
}

impl From<fmt::Error> for Error {
    fn from(kind: fmt::Error) -> Error {
        Error::FormatError { err: kind }
    }
}
// impl From<Error> for Error {
//     fn from(larerr:Error) -> Error {
//         Error::new(ErrorKind::Other, format!("can error - {}",larerr))
//     }
// }
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ConvertInt { err }
    }
}
impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Error {
        Error::ConvertFloat { err }
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::BadJsonData { err }
    }
}
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::BadTomlData { err }
    }
}
impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::SerializeTomlError { err }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::RequestError { err }
    }
}