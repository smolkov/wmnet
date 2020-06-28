use lazy_static::lazy_static;
use regex::Regex;
use serial::prelude::*;
use std::fs;
use std::io::prelude::*;
use wqms::Workspace;
// use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;
// use wqms::channel::Data;
use wqms::iface::Chan;
use wqms::interface::*;
use wqms::{config::Config, ws, Result};
lazy_static! {
    // N 0.0384                0.0000        0.0000      0.00        0.0000       25764             997.2           0
    // TT.MM.JJJJ;hh.mm.ss;000000.00;000000.00;E1870_L1_M1>
    // static ref NITRITOX: Regex = Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1,5}.\d{1}) \d{1}").unwrap();
    static ref NITRITOX: Regex = Regex::new(r"d{2}.d{2}.d{4};d{2}.d{2}.d{2};(?P<tox>\d{6}.\d{2});(?P<dos>\d{6}.\d{2});\s").unwrap();
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

fn serial_port(ws: &Workspace) -> PathBuf {
    let path = ws.rootdir().join("nitritox");
    if !path.is_dir() {
        if let Err(e) = fs::create_dir(&path) {
            log::error!("NITRITOX:init directory {}", e);
        }
    }
    path.join("serial")
}

fn simulate_nitritox(tox: &mut Vec<f32>, dos: &mut Vec<f32>) {
    tox.push(50.0);
    dos.push(124.0);
}
fn simulate_jumo(
    ws: &Workspace,
    ph: &mut Vec<f32>,
    ec: &mut Vec<f32>,
    orp: &mut Vec<f32>,
    dos: &mut Vec<f32>,
) -> Result<()> {
    ph.push(6.5);
    ec.push(124.0);
    orp.push(124.0);
    dos.push(124.0);
    Ok(())
}
fn simulate_kit(ws: &Workspace, dulling: &mut Vec<f32>) -> Result<()> {
    dulling.push(170.0);
    Ok(())
}
const CMD: &str = "D\r\n";
fn nitritox_rs232(ws: &Workspace, tox: &mut Vec<f32>, dos: &mut Vec<f32>) -> std::io::Result<()> {
    log::info!("NITRITOX: collect data");
    let mut buf: Vec<u8> = (0..255).collect();
    let mut port = serial::open(&serial_port(ws))?;
    port.configure(&SETTINGS).unwrap();
    port.set_timeout(Duration::from_secs(2)).unwrap();
    if let Err(e) = port.write(CMD.as_bytes()) {
        log::error!("NITRITOX:collect data write CMD - {}", e);
    } else {
        std::thread::sleep(std::time::Duration::from_millis(10));
        match port.read(&mut buf[..]) {
            Ok(_) => {
                let data_str = std::str::from_utf8(&buf).unwrap();
                let data = NITRITOX.captures(data_str).and_then(|cap| {
                    let tox_res = cap
                        .name("tox")
                        .map(|fsr| fsr.as_str().parse::<f32>().unwrap_or(0.0))
                        .unwrap();
                    let dos_res = cap
                        .name("dos")
                        .map(|dig| dig.as_str().parse::<f32>().unwrap_or(0.0))
                        .unwrap();
                    Some((tox_res, dos_res))
                });
                if let Some((tox_res, dos_res)) = data {
                    tox.push(tox_res);
                    dos.push(dos_res);
                };
            }
            Err(e) => log::error!("NITRITOX: collect data read RS232 - {}", e),
        }
    };
    Ok(())
}
// fn jumo(ph: &mut Vec<f32>, ec: &mut Vec<f32>, orp: &mut Vec<f32>, dos: &mut Vec<f32>) {
// ph.push(6.5);
// ec.push(124.0);
// orp.push(124.0);
// dos.push(124.0);
// }

#[paw::main]
fn main(args: Args) -> Result<()> {
    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    let ws = ws::setup()?;
    let channels = ws.channels();
    let config = ws.read_config().unwrap_or(Config::default());
    log::info!("setup TOX channel");
    let tox = channels.new_channel("TOX", "%")?;
    log::info!("setup DOS channel");
    let dos = channels.new_channel("DOS", "mg/l")?;
    let jumo_ph = channels.new_channel("ph", "pH")?;
    let jumo_ec = channels.new_channel("ec", "mg/l")?;
    let jumo_orp = channels.new_channel("orp", "mg/l")?;
    let jumo_dos = channels.new_channel("dos", "mh/l")?;
    let interval = Duration::from_secs(config.interval());
    let average = Duration::from_secs(config.average_time());
    let _outlier = config.outliers();
    let ticks = tick(interval);
    let calculate = tick(average);
    log::info!(
        "Run interval {} average time {}",
        config.interval(),
        config.average_time()
    );
    log::info!("nitritox:run measurement");
    let mut tox_data: Vec<f32> = Vec::new();
    let mut dos_data: Vec<f32> = Vec::new();
    let mut ph_data: Vec<f32> = Vec::new();
    let mut orp_data: Vec<f32> = Vec::new();
    let mut ec_data: Vec<f32> = Vec::new();
    let mut do_data: Vec<f32> = Vec::new();
    let mut dulling_data: Vec<f32> = Vec::new();
    let mut start_measurement = std::time::Instant::now();
    loop {
        select! {
            recv(ticks) -> _ => {
                if args.debug() {
                    simulate_nitritox(&mut tox_data, &mut dos_data);
                    simulate_jumo(&ws,&mut ph_data, &mut orp_data, &mut ec_data, &mut do_data)?;
                    simulate_kit(&ws,&mut dulling_data)?;
                } else {
                    if let Err(e) = nitritox_rs232(&ws,&mut tox_data,&mut dos_data) {
                        log::error!("Nitritox: collect rs232 - {}",e);
                    }
                    if let Err(e) = simulate_jumo(&ws,&mut ph_data, &mut orp_data, &mut ec_data, &mut do_data) {
                        log::error!("JUMO: collect data - {}",e);
                    }
                    if let Err(e) = simulate_kit(&ws,&mut dulling_data) {
                        log::error!("KIT: collect data - {}",e);
                    }
                }
                if start_measurement.elapsed().as_secs() > config.average_time()  {
                    log::info!("CALCULATE DATA!");
                    dos.calculate(&dos_data)?;
                    tox.calculate(&tox_data)?;
                    jumo_ph.calculate(&ph_data)?;
                    jumo_ec.calculate(&ec_data)?;
                    jumo_orp.calculate(&orp_data)?;
                    jumo_dos.calculate(&do_data)?;
                    tox_data.clear();
                    dos_data.clear();
                    ph_data.clear();
                    ec_data.clear();
                    orp_data.clear();
                    do_data.clear();
                    start_measurement = std::time::Instant::now();
                    if let Err(e) = ws.channels().next_measurement() {
                        log::error!("CHANNELS: next measurement - {}", e);
                    }
                }


            }
            recv(calculate) -> _ => {
                log::info!("NITRITOX calculate result!");
                 if let Err(e) = tox.next() {
                    log::error!("CHANNEL[{}] new measurement - {}", tox.path().display(), e);
                }
                if let Err(e) = dos.next() {
                    log::error!("CHANNEL[{}] new measurement - {}", dos.path().display(), e);
                };
                dos.calculate(&dos_data)?;
                tox.calculate(&tox_data)?;
                tox_data.clear();
                dos_data.clear();
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
