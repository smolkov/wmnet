//! WMS logging infrastructure.

use crate::Workspace;

pub fn setup(_ws: &Workspace,level:log::LevelFilter){
    femme::with_level(level);
}
pub fn debug() {
    femme::with_level(log::LevelFilter::Debug);
}
pub fn trace() {
    femme::with_level(log::LevelFilter::Trace);
}

