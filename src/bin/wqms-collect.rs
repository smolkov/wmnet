use firebase_rs::*;
use std::path::{Path, PathBuf};
// use wqms::config::*;
use chrono::{DateTime, Datelike, Utc};
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
    let channels = ws.channels();
    // let fb = wqms::fb::setup(&ws)?;

    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    let interval = args.interval();
    let ticks = tick(interval);
    log::info!(
        "Collect channels[{}] interval {}[sec]",
        channels.path().display(),
        interval.as_secs(),
    );
    let mut now: DateTime<Utc> = Utc::now();
    let mut path = channels
        .path()
        .join(format!("{}.csv", now.format("%Y%m%d%H%M")));
    let mut wtr = csv::Writer::from_path(path).unwrap();
    let tox = channels.new_channel("TOX", "%")?;
    let dos = channels.new_channel("DOS", "mg/l")?;
    let jumo_ph = channels.new_channel("ph", "pH")?;
    let jumo_ec = channels.new_channel("ec", "mg/l")?;
    let jumo_orp = channels.new_channel("orp", "mg/l")?;
    let jumo_dos = channels.new_channel("dos", "mh/l")?;
    let tur = channels.new_channel("trubung", "%")?;
    // https://<DATABASE_NAME>.firebaseio.com/users/ada/name.json?access_token=<ACCESS_TOKEN>
    loop {
        select! {
            recv(ticks) -> _ => {
                log::info!("Collect data to csv");
                if let Err(e) = wtr.write_record(&[&format!("{}",Utc::now().format("%Y.%m.%d-%H:%M:%S")),&tox.value(), &dos.value(), &jumo_ph.value(),&jumo_ec.value(),&jumo_orp.value(),&jumo_dos.value(),&tur.value()]) {
                    log::error!("Collect csv data - {}",e);
                }
                wtr.flush()?;
                if Utc::now().day() != now.day() {
                    path = channels.path().join(format!( "{}.csv", Utc::now().format("%Y%m%d%H%M")));
                    wtr = csv::Writer::from_path(path).unwrap();
                    now = Utc::now();
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
