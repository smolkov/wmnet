use crate::Result;
use crate::Workspace;
pub use crate::{Class, Property};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct InkyChan {
    name: String,
    unit: String,
    value: String,
}

#[derive(Debug)]
pub struct InkyState {
    chs: [InkyChan; 4],
    hostname: String,
    online: String,
}
