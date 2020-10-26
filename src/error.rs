#![allow(unused_variables)]

use failure::Fail;
use std::error::Error as StdError;

// use std::io::ErrorKind;
// use std::num::ParseFloatError;
// use std::num::ParseIntError;
// use std::path::Path;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "wms error - {}", 0)]
    Message (String),
    #[fail(display = "interface - {}", msg)]
    Interface { msg: String },
    #[fail(display = "working station error - {}",0)]
    BadWorkstation(String),
    #[fail(display = "anyhow error")]
    Anyhow(anyhow::Error)
}

// impl StdError for Error;

// impl From<&'static str> for Error {
//     fn from(descr: &'static str) -> Error {
//         Error::Message(descr.to_owned())
//     }
// }

// impl From<String> for Error {
//     fn from(descr: String) -> Error {
//         Error::Message(descr)
//     }
// }

pub fn bad_workstation(msg: String) -> Error {
    Error::BadWorkstation (msg)
}



impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(error: E) -> Self {
        Self::Anyhow(error.into())
    }
}

// impl AsRef<dyn StdError + Send + Sync> for Error {
//     fn as_ref(&self) -> &(dyn StdError + Send + Sync + 'static) {
//         self.as_ref()
//     }
// }

// impl AsRef<dyn StdError> for Error {
//     fn as_ref(&self) -> &(dyn StdError + 'static) {
//         self.as_ref()
//     }
// }

impl From<Error> for Box<dyn StdError + Send + Sync + 'static> {
    fn from(error: Error) -> Self {
        error.into()
    }
}

impl From<Error> for Box<dyn StdError + 'static> {
    fn from(error: Error) -> Self {
        Box::<dyn StdError + Send + Sync>::from(error)
    }
}