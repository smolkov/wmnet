use wmnet::cli::Args;
// use wmnet::{Config, Result, Workspace};
use wmnet::Result;

#[paw::main]
fn main(args: Args) -> Result<()> {
    let _ws = wmnet::wms::default();
    // let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    Ok(())
}
