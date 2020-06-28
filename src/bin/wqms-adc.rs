use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use nb::block;

use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use lazy_static::lazy_static;
use regex::Regex;
// use std::fs;
// use std::io::prelude::*;
// use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
// use std::time::Duration;
use structopt::StructOpt;
use wqms::iface::{Chan, Class};
use wqms::Result;

// lazy_static! {
// N 0.0384                0.0000        0.0000      0.00        0.0000       25764             997.2           0
// TT.MM.JJJJ;hh.mm.ss;000000.00;000000.00;E1870_L1_M1>
// static ref NITRITOX: Regex = Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1,5}.\d{1}) \d{1}").unwrap();
// static ref NITRITOX: Regex = Regex::new(r"d{2}.d{2}.d{4};d{2}.d{2}.d{2};(?P<tox>\d{6}.\d{2});(?P<dos>\d{6}.\d{2});\s").unwrap();
// }

use crossbeam::channel::{bounded, select, tick, Receiver};

///Analog digital conventer
#[derive(Debug, StructOpt)]
#[structopt(name = "adc-converter", about = "🧰trubung interface usage.")]
pub struct Args {
    ///🗁 simulate trubung measurement
    #[structopt(long = "debug")]
    debug: bool,
    ///🔌 device path
    #[structopt(short = "p", long = "device", default_value = "/dev/i2c-1")]
    device: PathBuf,
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
    /// Access the directory name.
    #[inline]
    pub fn device(&self) -> &Path {
        &self.device
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

fn simulate(trubung: &mut Vec<f32>) {
    trubung.push(45.0);
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1013(dev, address);
    // get I2C device back
    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    let ws = wqms::ws::setup()?;
    let channels = ws.channels();
    // let config = ws.read_config().unwrap_or(Config::default());
    log::info!("Setup Trubung channel");
    let tur = channels.new_channel("trubung", "%")?;
    let mut average = tur.average_interval();
    let interval = tur.measurement_interval();
    let ticks = tick(interval);
    let mut a0a1_data: Vec<f32> = Vec::new();
    let mut a0a3_data: Vec<f32> = Vec::new();
    let mut a1a3_data: Vec<f32> = Vec::new();
    let mut a2a3_data: Vec<f32> = Vec::new();
    let mut start_measurement = std::time::Instant::now();
    log::info!(
        "Trubung measurement interval {}[millis] average {}[sec]",
        interval.as_millis(),
        average.as_secs()
    );
    loop {
        select! {
            recv(ticks) -> _ => {
                if args.debug() {
                    simulate(&mut a0a1_data);
                } else {
                    match  block!(adc.read(&mut channel::DifferentialA0A1)) {
                        Ok(value) => {
                                a0a1_data.push(value as f32);
                                log::info!("A0A1:{}",value);
                            },
                        Err(e) => log::error!("Read DifferentialA0A1 - {:?}",e),
                    };
                    // match  block!(adc.read(&mut channel::DifferentialA0A3)) {
                    //     Ok(value) => {
                    //             a0a1_data.push(value as f32);
                    //             log::info!("DifferentialA0A3:{}",value);
                    //         },
                    //     Err(e) => log::error!("Read DifferentialA0A3 - {:?}",e),
                    // }
                    // match  block!(adc.read(&mut channel::DifferentialA1A3)) {
                    //     Ok(value) => {
                    //             a0a1_data.push(value as f32);
                    //             log::info!("DifferentialA1A3:{}",value);
                    //         },
                    //     Err(e) => log::error!("Read DifferentialA1A3 - {:?}",e),
                    // }
                    // match  block!(adc.read(&mut channel::DifferentialA2A3)) {
                    //     Ok(value) => {
                    //             a0a1_data.push(value as f32);
                    //             log::info!("DifferentialA2A3:{}",value);
                    //         },
                    //     Err(e) => log::error!("Read DifferentialA2A3 - {:?}",e),
                    // }
                }
                if start_measurement.elapsed().as_secs() > average.as_secs()  {
                    log::info!("CALCULATE TRUBUNG!");
                    if let Err(e) = tur.calculate(&a0a1_data) {
                        log::error!("CHANNELS[{}]: calculate - {}",tur.path().display(), e);
                    }
                    start_measurement = std::time::Instant::now();
                    average = tur.average_interval();
                    if let Err(e) = tur.next() {
                        log::error!("CHANNELS[{}]: next measurement - {}",tur.path().display(), e);
                    }
                    a0a1_data.clear();
                }


            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Abort!");
                break;
            }
        }
    }
    let _dev = adc.destroy_ads1013();
    Ok(())
}
