// use std::path::PathBuf;
use wqms::{ws, Result, Workspace};

fn main() -> Result<()> {
    let _ = ws::setup();
    // let fb = Firebase::auth(&cfg.fb.url, &cfg.fb.key).unwrap();
    // let path = format!("stations/{}", cfg.fb.uid);
    // let station = fb.at(&path).unwrap();
    // let res = station.get().unwrap();
    // println!("{:?}", &res);
    Ok(())
}
