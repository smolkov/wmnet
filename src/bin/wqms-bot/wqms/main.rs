mod setting;
mod wifi;
use wqms::cli::Args;
// use wqms::{Config, Result, Workspace};



#[paw::main]
fn main(args: Args) -> wqms::Result<()> {
    let _ws = wqms::ws::default();
    
    Ok(())
}