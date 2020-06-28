use lazy_static::lazy_static;
use regex::Regex;
use serial::prelude::*;
// use std::fs;
use std::io::prelude::*;
use wqms::Workspace;
// use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;
// use wqms::channel::Data;
use wqms::iface::Chan;
// use wqms::interface::*;
use wqms::{config::Config, ws, Result};
lazy_static! {
    // N 0.0384                0.0000        0.0000      0.00        0.0000       25764             997.2           0
    // TT.MM.JJJJ;hh.mm.ss;000000.00;000000.00;E1870_L1_M1>
    // static ref NITRITOX: Regex = Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1,5}.\d{1}) \d{1}").unwrap();
    static ref JUMO: Regex = Regex::new(r"d{2}.d{2}.d{4};d{2}.d{2}.d{2};(?P<tox>\d{6}.\d{2});(?P<dos>\d{6}.\d{2});\s").unwrap();
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
    port: PathBuf,
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

// fn serial_port(ws: &Workspace) -> PathBuf {
//     let path = ws.rootdir().join("nitritox");
//     if !path.is_dir() {
//         if let Err(e) = fs::create_dir(&path) {
//             log::error!("NITRITOX:init directory {}", e);
//         }
//     }
//     path.join("serial")
// }

fn simulate_jumo(
    ws: &Workspace,
    ph: &mut Vec<f32>,
    ec: &mut Vec<f32>,
    orp: &mut Vec<f32>,
    dos: &mut Vec<f32>,
    temp: &mut Vec<f32>,
) -> Result<()> {
    ph.push(6.5);
    ec.push(124.0);
    orp.push(124.0);
    dos.push(124.0);
    temp.push(24.0);
    Ok(())
}
// fn jumo(ph: &mut Vec<f32>, ec: &mut Vec<f32>, orp: &mut Vec<f32>, dos: &mut Vec<f32>) {
// ph.push(6.5);
// ec.push(124.0);
// orp.push(124.0);
// dos.push(124.0);
// }
const DEVICE_ID: u8 = 1;
use libmodbus_rs::{Modbus, ModbusClient, ModbusRTU, ModbusTCP};
use std::mem::size_of;
use time::PreciseTime;
const G_MSEC_PER_SEC: i64 = 1_000;
#[paw::main]
fn main(args: Args) -> Result<()> {
    // let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    // let ws = ws::setup()?;
    // let channels = ws.channels();
    // let config = ws.read_config().unwrap_or(Config::default());
    // let ph = channels.new_channel("ph", "pH")?;
    // let ec = channels.new_channel("ec", "mg/l")?;
    // let orp = channels.new_channel("orp", "mg/l")?;
    // let dos = channels.new_channel("do", "mh/l")?;
    // let temp = channels.new_channel("C", "°")?;
    // let _outlier = config.outliers();
    // let ticks = tick(args.interval());
    // let mut ph_data: Vec<f32> = Vec::new();
    // let mut orp_data: Vec<f32> = Vec::new();
    // let mut ec_data: Vec<f32> = Vec::new();
    // let mut do_data: Vec<f32> = Vec::new();
    // let mut temp_data: Vec<f32> = Vec::new();
    let mut modbus =
        Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1).expect("Could not create RTU context");
    modbus.set_slave(DEVICE_ID).expect("Could not set slave id");
    // let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1).unwrap();
    // modbus.set_slave(YOUR_DEVICE_ID);

    match modbus.connect() {
        Ok(_) => {}
        Err(e) => println!("Connect error: {}", e),
    }
    /* Allocate and initialize the memory to store the status */
    let mut tab_bit = vec![0u8; Modbus::MAX_READ_BITS as usize * size_of::<u8>()];

    /* Allocate and initialize the memory to store the registers */
    let mut tab_reg = vec![0u16; Modbus::MAX_READ_REGISTERS as usize * size_of::<u16>()];
    let n_loop = 100;
    println!("READ BITS\n");

    let nb_points = Modbus::MAX_READ_BITS as u16;
    let start = PreciseTime::now();
    // for _ in 0..n_loop {
    let rc = modbus.read_bits(1, nb_points, &mut tab_bit);
    if rc.is_err() {
        println!("ERROR {:?}", rc);
        // break;
    }
    // }
    let end = PreciseTime::now();
    let elapsed = start.to(end).num_milliseconds();

    let rate = (n_loop * nb_points as i64) * G_MSEC_PER_SEC / elapsed;
    println!("Transfert rate in points/seconds:");
    println!("* {} points/s", rate);
    println!();

    let bytes = n_loop * (nb_points as i64 / 8) + (if nb_points % 8 == 0 { 1 } else { 0 });
    let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    println!("Values:");
    println!("* {} x {} values", n_loop, nb_points);
    println!("* {:.3} ms for {} bytes", elapsed, bytes);
    println!("* {} KiB/s", rate);
    println!();

    /* TCP: Query and reponse header and values */
    // let bytes = 12 + 9 + (nb_points as i64 / 8) + (if nb_points % 8 == 0 { 1 } else { 0 });
    // println!("Values and TCP Modbus overhead:");
    // println!("* {} x {} bytes", n_loop, bytes);
    // let bytes = n_loop * bytes;
    // let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    // println!("* {:.3} ms for {} bytes", elapsed, bytes);
    // println!("* {} KiB/s", rate);
    // println!("\n");

    println!("READ REGISTERS\n");

    let nb_points = Modbus::MAX_READ_REGISTERS as u16;
    let start = PreciseTime::now();
    // for _ in 0..n_loop {
    let rc = modbus.read_registers(1, nb_points, &mut tab_reg);
    if rc.is_err() {
        println!("ERROR {:?}", rc);
        // break;
    }
    // }
    let end = PreciseTime::now();
    let elapsed = start.to(end).num_milliseconds();

    let rate = (n_loop * nb_points as i64) * G_MSEC_PER_SEC / elapsed;
    println!("Transfert rate in points/seconds:");
    println!("* {} registers/s", rate);
    println!();

    let bytes = n_loop * nb_points as i64 * size_of::<u16>() as i64;
    let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    println!("Values:");
    println!("* {} x {} values", n_loop, nb_points);
    println!("* {:.3} ms for {} bytes", elapsed, bytes);
    println!("* {} KiB/s", rate);
    println!("");

    /* TCP:Query and reponse header and values */
    // let bytes = 12 + 9 + (nb_points as i64 * size_of::<u16>() as i64);
    // println!("Values and TCP Modbus overhead:");
    // println!("* {} x {} bytes", n_loop, bytes);
    // let bytes = n_loop * bytes;
    // let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    // println!("* {:.3} ms for {} bytes", elapsed, bytes);
    // println!("* {} KiB/s", rate);
    // println!("\n");

    println!("WRITE AND READ REGISTERS\n");

    // let nb_points = Modbus::MAX_WR_WRITE_REGISTERS as u16;
    // let start = PreciseTime::now();
    // for _ in 0..n_loop {
    //     let rc = modbus.write_and_read_registers(
    //         0,
    //         nb_points,
    //         &tab_reg.clone(), // FIXME: this clone costs to much
    //         0,
    //         nb_points,
    //         &mut tab_reg,
    //     );
    //     if rc.is_err() {
    //         println!("ERROR {:?}", rc)
    //     }
    // }
    // let end = PreciseTime::now();
    // let elapsed = start.to(end).num_milliseconds();

    // let rate = (n_loop * nb_points as i64) * G_MSEC_PER_SEC / elapsed;
    // println!("Transfert rate in points/seconds:");
    // println!("* {} registers/s", rate);
    // println!("");

    // let bytes = n_loop * nb_points as i64 * size_of::<u16>() as i64;
    // let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    // println!("Values:");
    // println!("* {} x {} values", n_loop, nb_points);
    // println!("* {:.3} ms for {} bytes", elapsed, bytes);
    // println!("* {} KiB/s", rate);
    // println!("");

    /* TCP:Query and reponse header and values */
    // let bytes = 12 + 9 + (nb_points as i64 * size_of::<u16>() as i64);
    // println!("Values and TCP Modbus overhead:");
    // println!("* {} x {} bytes", n_loop, bytes);
    // let bytes = n_loop * bytes;
    // let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    // println!("* {:.3} ms for {} bytes", elapsed, bytes);
    // println!("* {} KiB/s", rate);
    // println!("");
    // println!("Modbus mod {}", modbus.rtu_get_serial_mode().unwrap());
    // let mb_mapping = ModbusMapping::new(Modbus::MAX_READ_BITS, 0, Modbus::MAX_READ_REGISTERS, 0)
    // .expect("Failed to allocate the mapping");

    // loop {
    // let mut query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH];

    // match modbus.receive(&mut query) {
    // Ok(rc) => modbus.reply(&query, rc, &mb_mapping),
    // Err(err) => {
    // println!("Quit the loop: {}", err);
    // }
    // }
    // .unwrap();
    // }
    // let mut port = serial::open("/dev/ttyUSB0").unwrap();
    // port.configure(&SETTINGS).unwrap();
    // port.set_timeout(Duration::from_secs(2)).unwrap();
    // let mut dulling_data: Vec<f32> = Vec::new();
    // let mut start_measurement = std::time::Instant::now();
    // log::info!(
    //     "JUMO measurement interval {}[msec] averaging time {}[sec]",
    //     config.interval(),
    //     config.average_time()
    // );
    // loop {
    //     select! {
    //         recv(ticks) -> _ => {
    //             if args.debug() {
    //                 simulate_jumo(&ws,&mut ph_data, &mut orp_data, &mut ec_data, &mut do_data,&mut temp_data)?;
    //             } else {
    //                 let mut buf: Vec<u8> = (0..255).collect();
    //                 match port.read(&mut buf[..]) {
    //                     Ok(_) => {
    //                         println!("DATA:{:?}",&buf);
    //                         let data_str = String::from_utf8(buf).unwrap();
    //                         println!("STR:{}",data_str);
    //                     },
    //                     Err(e) => {
    //                         println!("READ ERROR - {}",e);
    //                     }
    //                 };
    //                 if start_measurement.elapsed().as_secs() > config.average_time()  {
    //                     log::info!("CALCULATE DATA!");
    //                     ph.calculate(&ph_data)?;
    //                     ec.calculate(&ec_data)?;
    //                     orp.calculate(&orp_data)?;
    //                     dos.calculate(&do_data)?;
    //                     temp.calculate(&do_data)?;
    //                     ph_data.clear();
    //                     ec_data.clear();
    //                     orp_data.clear();
    //                     do_data.clear();
    //                     temp_data.clear();
    //                     start_measurement = std::time::Instant::now();
    //                     if let Err(e) = ws.channels().next_measurement() {
    //                         log::error!("CHANNELS: next measurement - {}", e);
    //                     }
    //                 }
    //             }
    //         },
    //         recv(ctrl_c_events) -> _ => {
    //             println!();
    //             println!("Abort!");
    //             break;
    //         }
    //     }
    // }
    Ok(())
}
