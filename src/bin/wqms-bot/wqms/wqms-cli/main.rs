use wqms::cli::Args;
// use wqms::{Config, Result, Workspace};
use wqms::Result;

#[paw::main]
fn main(args: Args) -> Result<()> {
    let _ws = wqms::ws::default();
    // let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    Ok(())
}
