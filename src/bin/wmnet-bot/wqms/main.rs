mod setting;
mod wifi;
use wmnet::cli::Args;
// use wmnet::{Config, Result, Workspace};



#[paw::main]
fn main(args: Args) -> wmnet::Result<()> {
    let _ws = wmnet::wms::default();
    
    Ok(())
}