// use super::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Station {
    pub name: String,
    pub uid: String,
}
