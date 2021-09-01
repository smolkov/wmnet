use serial::prelude::*;
// use std::fs;
use std::io::prelude::*;
// use wmnet::Workspace;
use wmnet::thingspeak::*;
// use std::io::prelude::*;
// use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;
// use wmnet::channel::Data;
// use wmnet::channel::*;
// use wmnet::store::Store;
// use std::io::prelude::*;
use wmnet::{wms, Result};
use wmnet::jumo::Jumo;
use wmnet::nitri::Nitri;
use wmnet::channel::Channel;
use wmnet::iface::Class;
// use wmnet::analog::Analog;
use crossbeam::channel::{bounded, select, tick, Receiver};
// use chrono::{DateTime, Datelike, Utc};
// use serde::{Deserialize, Serialize};

// use rand_distr::{Normal, Distribution};
// use rand::thread_rng;



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

const CMD: &str = "D\r\n";
fn nitritox_rs232(nitri: &Nitri) -> std::io::Result<()> {
    // log::info!("NITRITOX: collect data");
    let mut buf: Vec<u8> = vec![0; 1000];
    let mut port = serial::open(nitri.uart().as_str())?;
    port.configure(&SETTINGS).unwrap();
    port.set_timeout(Duration::from_secs(4))?;
    port.write(CMD.as_bytes())?;
    std::thread::sleep(std::time::Duration::from_millis(2000));
    port.read(&mut buf[..])?;
    if let Err(e)=nitri.decode(&buf) {
        log::error!("Nitritox decode recived data failed - {}",e);
    }
    Ok(())
}


fn jumo_modbus(jumo:&Jumo) -> std::io::Result<()> {
    use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};

    let modbus = Modbus::new_tcp(jumo.addr().as_str(),jumo.port()).expect("Could not create TCP context");
    if let Err(e) = modbus.connect() {
        log::error!("Connection failed: {}", e);
    }
    let mut tab_reg = vec![0u16; 32];
    if let Err(e) = modbus.read_registers(wmnet::jumo::ORP_REG,3, &mut tab_reg) {
        jumo.set_orp_value("none").unwrap();
        log::error!("read register 0x164D {}", e); 
    } else {
        jumo.set_orp(tab_reg[0],tab_reg[1]).expect("jumo set orp error");
        // log::info!("READ 0x164D: ORP: {}",jumo.orp());
    }
    if let Err(e) = modbus.read_registers(0x164F,3, &mut tab_reg) {
        jumo.set_ph_value("none").unwrap();
        log::error!("read register 0x164F {}", e); 
    }else {
        jumo.set_ph(tab_reg[0],tab_reg[1]).expect("jumo set orp error");
        // log::info!("READ 0x164F: PH: {}",jumo.ph());
    }
    if let Err(e) = modbus.read_registers(wmnet::jumo::EC_REG,3, &mut tab_reg) {
        jumo.set_ec_value("none").unwrap();
        log::error!("read register 0x1651 {}", e); 
    }else {
        jumo.set_ec(tab_reg[0],tab_reg[1]).expect("jumo set orp error");
        // log::info!("READ 0x1651: LEITFAHIGKEIT: {}",jumo.ec());
    }
    if let Err(e) = modbus.read_registers(0x16BB,3, &mut tab_reg) {
        log::error!("read register 0x16BB {}", e); 
        jumo.set_temp_value("none").unwrap();
    } else {
        jumo.set_temp(tab_reg[0],tab_reg[1]).expect("jumo set temperatur error");
        // log::info!("READ 0x16BB: TEMPERATUR: {}",jumo.temp());
    }
    Ok(())
}
// Analog device
// fn read_tur(analog: &Analog) -> Result<()> {
//     use embedded_hal::adc::OneShot;
//     use linux_embedded_hal::I2cdev;
//     use nb::block;
//     use ads1x1x::{channel, Ads1x1x, SlaveAddr};
    
//     match I2cdev::new(analog.addr().as_str()) {
//         Ok(dev) => {
//             let address = SlaveAddr::default();
//             let mut adc = Ads1x1x::new_ads1013(dev, address);
//             let value   = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap() as f32;
//             analog.set_value(value)?;
//             let _dev = adc.destroy_ads1013();
//         },
//         Err(e) => {
//             log::error!("Open I2c device failed {}", e); 
//         }
//     }

//     Ok(())
// }

#[paw::main]
fn main(args: Args) -> Result<()> {
    wmnet::logger::debug();
    // wmnet::metrics::init()?;
    let wms = wms::default();
    let nitri    = wmnet::nitri::setup(&wms)?;
    let jumo     = wmnet::jumo::setup(&wms)?;
    nitri.reset()?;
    jumo.reset()?;
    let analog   = wmnet::analog::setup(&wms)?;
    let thingspeak = wms.thingspeak()?;

    let mut channels = wms.channels()?;
    channels.reset()?;
    let path = channels.path();
    let mut tox  = Channel::create(path,"tox","Tox","%");
    let mut dos  = Channel::create(path,"dos","Dos","ml/min");
    let mut ph   = Channel::create(path,"ph","pH","");
    let mut ec   = Channel::create(path,"ec","EC","mS/cm");
    let mut orp  = Channel::create(path,"orp","ORP","mV");
    let mut temp = Channel::create(path,"temp","Temp","°C");
    // let mut tur  = Channel::create(path,"tur","Tur","NTU");
    
   
    let ticks     = tick(channels.measurement_interval());
    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    log::info!("Run interval {} [msec] average time {} [sec]",channels.measurement_interval().as_millis(),channels.average_interval().as_secs());
    let mut start_measurement = std::time::Instant::now();
    // let mut rng = thread_rng();
    // let tur_sumulate = Normal::new(0.0,100.0).unwrap();
    tox.clear()?;
    dos.clear()?;
    ph.clear()?;
    ec.clear()?;
    orp.clear()?;
    temp.clear()?;
    loop {
        select! {
            recv(ticks) -> _ => {
                if args.debug() {
                    nitri.simulate()?;
                    jumo.simulate()?;
                    analog.simulate()?;
                } else {
                    if let Err(e) = nitritox_rs232(&nitri) {
                        log::error!("nitritox collect rs232 - {}",e);
                    }
                    if let Err(e) = jumo_modbus(&jumo) {
                        log::error!("jumo collect modbus- {}",e);
                    }
                    // if let Err(e) = analog.simulate() {
                        // log::error!("Analog: collect data - {}",e);
                    // }
                    // if let Err(e) = read_tur(&analog) {
                        // log::error!("Analog: collect data - {}",e);
                    // }
                    // if let Err(e) = kit(&mut dulling_data) {
                        // log::error!("KIT: collect data - {}",e);
                    // }
                }
                if let Err(e) = tox.push_data(nitri.toxic().as_str()){
                    log::error!("push data to toxic channel tox failed- {}", e); 
                }
                if let Err(e) = dos.push_data(nitri.dosing().as_str()){
                    log::error!("push data to channel dos failed- {}", e); 
                }
                if let Err(e) = ph.push_data(jumo.ph().as_str()){
                    log::error!("push data to  channel ph failed- {}", e); 
                }
                if let Err(e) = ec.push_data(jumo.ec().as_str()){
                    log::error!("push data to channel cond failed- {}", e); 
                }
                if let Err(e) = orp.push_data(jumo.orp().as_str()){
                    log::error!("push data to channel orp failed- {}", e); 
                }
                if let Err(e) = temp.push_data(jumo.temp().as_str()){
                    log::error!("push data to channel temp failed- {}", e); 
                }
                // if let Err(e) = tur.push_data(analog.value().as_str()){
                    // log::error!("push data to channel tur failed- {}", e); 
                // }
                if start_measurement.elapsed().as_secs() > channels.average_interval().as_secs()  {
                    log::info!("CALCULATE DATA!");
                    if let Err(e)=channels.calculate() {
                        log::error!("calculate channel value failed- {}", e); 
                    }
                    // if let Err(e) = tox.calculate(){
                    //     log::error!("tox calculate channel value {}", e);
                    // }
                    // if let Err(e) = dos.calculate(){
                    //     log::error!("dos calculate channel value {}", e);
                    // }
                    // if let Err(e) = ph.calculate(){
                    //     log::error!("ph calculate channel value {}", e);
                    // }
                    // if let Err(e) = cond.calculate(){
                    //     log::error!("cond calculate channel value {}", e);
                    // }
                    // if let Err(e) = orp.calculate(){
                    //     log::error!("orp calculate channel value {}", e);
                    // }
                    // if let Err(e) = temp.calculate(){
                    //     log::error!("temp calculate channel value {}", e);
                    // }
                    // if let Err(e) = tur.calculate(){
                    //     log::error!("tur calculate channel value {}", e);
                    // }
                    if let Err(e) = channels.history() {
                        log::error!("channels: write history - {}", e);
                    }
                    start_measurement = std::time::Instant::now();
                    let mut data = TSData::new(); 
                    println!("TOX={}DOS={}",tox.last_value().unwrap(),dos.last_value().unwrap());
                    data.field1 = tox.last_value();
                    data.field2 = dos.last_value();
                    data.field3 = ph.last_value();
                    data.field4 = ec.last_value();
                    data.field5 = orp.last_value();
                    data.field6 = None;
                    data.field7 = temp.last_value();
                    data.field8 = None;
                    data.status = channels.status();
                    if let Err(e) = thingspeak.publish(data){
                        log::error!("thingspeak publish data- {}", e);
                    }
                    // let mut pipe = redis::pipe();
                    // pipe.cmd("SADD").arg("my_set").arg(num).ignore();
                    // log::info!("Collect data to csv");
                    // if let Err(e) = wtr.write_record(&[&format!("{}",Utc::now().format("%Y.%m.%d-%H:%M:%S")),&tox.value(), &dos.value(), &ph.value(),&orp.value(),&cond.value(),&temp.value(),&tur.value()]) {
                        // log::error!("Collect csv data - {}",e);
                    // }
                    // wtr.flush()?;
                    // if Utc::now().day() != now.day() {
                        // path = channels.path().join(format!( "{}.csv", Utc::now().format("%Y%m%d%H%M")));
                        // wtr = csv::Writer::from_path(path).unwrap();
                        // now = Utc::now();
                        // if let Err(e) = wtr.write_record(&[ "timestamp", "tox", "dos", "ph", "orp", "cond","temp", "tur"]) {
                            // log::error!("Write header data - {}", e);
                        // }
                    // }
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
