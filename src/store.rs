use crate::workdir;
use crate::Result;
use sled::Config;
// use std::collections::HashMap;

use once_cell::sync::OnceCell;
// use std::collections::HashMap;

static KVS: OnceCell<sled::Db> = OnceCell::new();
// Same, but completely without macros
pub fn kvs() -> &'static sled::Db {
    static INSTANCE: OnceCell<sled::Db> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let config = Config::new().temporary(false).path(workdir().join("kvs"));
        let db = config.open().unwrap();
        db
    })
}

pub fn setup(ws: &crate::Workspace) -> Result<()> {
    let config = Config::new()
        .temporary(false)
        .path(ws.rootdir().join("kvs2"));
    let db = config.open().unwrap();
    KVS.set(db).unwrap();
    Ok(())
}
