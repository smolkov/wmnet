// use firebase_rs::*;
use std::path::{Path, PathBuf};
// use wqms::config::*;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use wqms::iface::{Chan, Class};
use wqms::Result;
// fn provide_data(config: &Config) -> Result<()> {
// Ok(())
// }

use crossbeam::channel::{bounded, select, tick, Receiver};

///Analog digital conventer
#[derive(Debug, StructOpt)]
#[structopt(name = "", about = "🧰trubung interface usage.")]
pub struct Args {
    ///🗁 simulate trubung measurement
    #[structopt(long = "debug")]
    debug: bool,
    //⏱ interval in seconds
    #[structopt(short = "i", long = "interval", default_value = "1")]
    interval: u64,
}

/// 🔧 Activate debug mode
impl Args {
    /// Access the directory name.
    #[inline]
    pub fn debug(&self) -> bool {
        self.debug
    }

    #[inline]
    pub fn interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.interval)
    }
}

fn ctrl_channel() -> Result<Receiver<()>> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })
    .unwrap();

    Ok(receiver)
}

fn simulate(trubung: &mut Vec<f32>) {
    trubung.push(45.0);
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub timestamp: u64,
    pub status: u8,
    pub tox: f64,
    pub bod: f64,
}
#[paw::main]
fn main(args: Args) -> Result<()> {
    // let path = format!("stations/{}", cfg.fb.uid);
    // let station = fb.at(&path).unwrap();
    // let res = station.get().unwrap();
    // println!("{:?}", &res);
    // get I2C device back
    let ws = wqms::ws::setup()?;
    let channels = ws.channels()?;
    let fb = wqms::fb::setup(&ws)?;

    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    let interval = args.interval();
    let ticks = tick(interval);
    log::info!(
        "Collect channels[{}] interval {}[sec]",
        channels.path().display(),
        interval.as_secs(),
    );
    let station: &'static str = "afrika1";
    let url: &'static str = "https://wqms-fb.firebaseio.com";
    let api: &'static str = "1:276128813099:web:a190a252dcf7f1dc00da6e";
    // https://<DATABASE_NAME>.firebaseio.com/users/ada/name.json?access_token=<ACCESS_TOKEN>
    loop {
        select! {
            recv(ticks) -> _ => {
                log::info!("Collect data to firebase");
                // match Firebase::auth(url, api) {
                    // Ok(fb) => {
                        // let messages =fb.at("/api/messages").ok().unwrap();
                        // let res = messages.push("{\"name\":\"David\",\"message\":\"Hello from Rust\"}").ok().unwrap();
                        // println!("Response body: {:?}", res.body);
                        // println!("Response code: {:?}", res.code);
                        // let users = fb.at("users").unwrap();
                        // let res = users.push("{\"username\": \"test\"}").unwrap();
                        // let data = fb.at(station).unwrap();
                        // let res = data.push("{\"timestamp\": \"2020.06.26 16:15:25\",\"status\": 0, \"bod\": 0, \"tox\": 0}").unwrap();
                        // println!("Response body: {:?}", res.body);
                        // println!("Response code: {:?}", res.code);
                    // },
                    // Err(e) => {
                        // log::error!("Collect data to firebase {} - {:?}",fb.url(),e);
                    // }
                // }
                if args.debug() {
                } else {
                }
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Abort!");
                break;
            }
        }
    }
    Ok(())
}
