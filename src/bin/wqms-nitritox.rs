use lazy_static::lazy_static;
use regex::Regex;
use serial::prelude::*;
use std::io;
use std::io::prelude::*;
// use std::path::{Path, PathBuf};
use std::time::Duration;
use structopt::StructOpt;
use wqms::iface::Chan;
use wqms::{config::Config, ws, Result};

lazy_static! {
    // N 0.0384                0.0000        0.0000      0.00        0.0000       25764             997.2           0
    static ref NITRITOX: Regex = Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1,5}.\d{1}) \d{1}").unwrap();
}

use crossbeam::channel::{bounded, select, tick, Receiver};

///Edinburgh sensor command argument
#[derive(Debug, StructOpt)]
#[structopt(name = "nitritox", about = "🧰nitritox rs232 interface usage.")]
pub struct Args {
    ///🗁 simulate nitritox measurement
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    ///🔌 hardware connection address
    #[structopt(short = "p", long = "port", default_value = "/dev/ttyUSB0")]
    port: String,
    //⏱ interval in seconds
    #[structopt(short = "i", long = "interval", default_value = "5")]
    interval: u64,
    //⏱ averaging interval in seconds
    #[structopt(short = "a", long = "average", default_value = "3600")]
    average: u64,
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
    #[inline]
    pub fn average(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.average)
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
const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

pub fn simulate() -> (f32, f32) {
    (50.0, 120.0)
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    let ws = ws::setup()?;
    let channels = ws.channels();
    let config = ws.read_config().unwrap_or(Config::default());
    // let driver = mio::driver::start(args.directory())?;
    // let config: Config = confy::load("edinburg")?;
    // println!("{:#?}", config);
    // let channels = channel::Registry::new(&ws::workdir())?;
    let tox = channels.new_channel("TOX", "%")?;
    let dos = channels.new_channel("DOS", "mg/l")?;
    tox.next()?;
    dos.next()?;
    let interval = std::time::Duration::from_secs(config.interval());
    let average = std::time::Duration::from_secs(config.average_time());
    let _outlier = config.outliers();
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    let ticks = tick(interval);
    let calculate = tick(average);
    let mut port = serial::open(&args.port).unwrap();
    port.configure(&SETTINGS).unwrap();
    port.set_timeout(Duration::from_secs(2)).unwrap();
    let cmd: &'static str = "D\r\n";
    // let mut tox_data: Vec<f32> = Vec::new();
    // let mut dos_data: Vec<f32> = Vec::new();
    loop {
        select! {
            recv(ticks) -> _ => {
                if args.debug() {
                    let (tox_res,dos_res) = simulate();
                    tox.push_data(tox_res)?;
                    dos.push_data(dos_res)?;
                } else {
                    // let mut buf: Vec<u8> = (0..255).collect();
                    writeln!(handle, "write!")?; // add `?` if you care about errors here
                    port.write(cmd.as_bytes())?;
                    writeln!(handle, "read!")?; // add `?` if you care about errors here
                    // match port.read(&mut buf[..]) {
                    //     Ok(_) => {
                    //         let data_str = std::str::from_utf8(&buf).unwrap();
                    //         let data = NITRITOX.captures(data_str).and_then(|cap| {
                    //             let tox_res = cap.name("tox").map(|fsr| fsr.as_str().parse::<f32>().unwrap_or(0.0)).unwrap();
                    //             let dos_res = cap.name("dos").map(|dig| dig.as_str().parse::<f32>().unwrap_or(0.0)).unwrap();
                    //             Some((tox_res, dos_res))
                    //         });
                    //         if let Some((tox_res,dos_res)) = data {
                    //             tox.push_data(tox_res)?;
                    //             dos.push_data(dos_res)?;
                    //         };
                    //     },
                    //     Err(e) => writeln!(handle,"{:?}",e);
                    //     // writeln!(handle,"{:?}",&buf)?; // add `?` if you care about errors here
                    // }
                }
            }
            recv(calculate) -> _ => {
                println!("Calculate!");
                tox.calculate()?;
                dos.calculate()?;
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
