use serial::prelude::*;
// use std::fs;
use std::io::prelude::*;
// use wqms::Workspace;
use wqms::thingspeak::*;
// use std::io::prelude::*;
// use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;
// use wqms::channel::Data;
// use wqms::channel::*;
// use wqms::store::Store;
// use std::io::prelude::*;
use wqms::{ws, Result};
use wqms::jumo::Jumo;
use wqms::nitri::Nitri;
use wqms::analog::Analog;
use crossbeam::channel::{bounded, select, tick, Receiver};
// use chrono::{DateTime, Datelike, Utc};
// use serde::{Deserialize, Serialize};

use rand_distr::{Normal, Distribution};
use rand::thread_rng;



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
    log::info!("NITRITOX: collect data");
    let mut buf: Vec<u8> = vec![0; 1000];
    let mut port = serial::open(nitri.uart().as_str())?;
    port.configure(&SETTINGS).unwrap();
    port.set_timeout(Duration::from_secs(4))?;
    port.write(CMD.as_bytes())?;
    std::thread::sleep(std::time::Duration::from_millis(2000));
    port.read(&mut buf[..])?;
   
    Ok(())
}


fn jumo_modbus(jumo:&Jumo) -> std::io::Result<()> {
    use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};

    let modbus = Modbus::new_tcp(jumo.addr().as_str(),jumo.port()).expect("Could not create TCP context");
    if let Err(e) = modbus.connect() {
        log::error!("Connection failed: {}", e);
    }
    let mut tab_reg = vec![0u16; 32];
    if let Err(e) = modbus.read_registers(wqms::jumo::ORP_REG,3, &mut tab_reg) {
        log::error!("read register 0x164D {}", e); 
    } else {
        jumo.set_orp(tab_reg[0],tab_reg[1]).expect("jomo set orp error");
        log::info!("READ 0x164D: ORP: {}",jumo.orp());
    }
    if let Err(e) = modbus.read_registers(wqms::jumo::PH_REG,3, &mut tab_reg) {
        log::error!("read register 0x164F {}", e); 
    }else {
        jumo.set_ph(tab_reg[0],tab_reg[1]).expect("jomo set orp error");
        log::info!("READ 0x164F: PH: {}",jumo.ph());
    }
    if let Err(e) = modbus.read_registers(wqms::jumo::COND_REG,3, &mut tab_reg) {
        log::error!("read register 0x1651 {}", e); 
    }else {
        jumo.set_cond(tab_reg[0],tab_reg[1]).expect("jomo set orp error");
        log::info!("READ 0x1651: LEITFAHIGKEIT: {}",jumo.cond());
    }
    if let Err(e) = modbus.read_registers(wqms::jumo::TEMP_REG,3, &mut tab_reg) {
        log::error!("read register 0x16BB {}", e); 
    } else {
        jumo.set_temp(tab_reg[0],tab_reg[1]).expect("jumo set temperatur error");
        log::info!("READ 0x16BB: TEMPERATUR: {}",jumo.temp());
    }
    Ok(())
}
/// Analog device
fn read_tur(analog: &Analog) -> Result<()> {
    use embedded_hal::adc::OneShot;
    use linux_embedded_hal::I2cdev;
    use nb::block;
    use ads1x1x::{channel, Ads1x1x, SlaveAddr};
    
    match I2cdev::new(analog.addr().as_str()) {
        Ok(dev) => {
            let address = SlaveAddr::default();
            let mut adc = Ads1x1x::new_ads1013(dev, address);
            let value   = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap() as f32;
            analog.set_value(value)?;
            let _dev = adc.destroy_ads1013();
        },
        Err(e) => {
            log::error!("Open I2c device failed {}", e); 
        }
    }

    Ok(())
}

#[paw::main]
fn main(args: Args) -> Result<()> {

    wqms::logger::debug();
    let ws = ws::root();
    let channels = ws.channels()?;
    let nitri    = wqms::nitri::setup(&ws)?;
    let jumo     = wqms::jumo::setup(&ws)?;
    let analog   = wqms::analog::setup(&ws)?;
    let thingspeak = ws.thingspeak()?;
    if channels.lastid() == 0 {
        channels.add("tox", "%")?;
        channels.add("dos", "mgl")?;
        channels.add("ph", "mgl")?;
        channels.add("cond", "mgl")?;
        channels.add("orp", "mgl")?;
        channels.add("temp", "mgl")?;
        channels.add("tur", "mgl")?;
    }
    let mut tox = channels.find("1").unwrap();
    let mut dos = channels.find("2").unwrap();
    let mut ph = channels.find("3").unwrap();
    let mut cond = channels.find("4").unwrap();
    let mut orp = channels.find("5").unwrap();
    let mut temp = channels.find("6").unwrap();
    let mut tur = channels.find("7").unwrap();

    let ticks     = tick(channels.measurement_interval());
    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    log::info!("Run interval {} [msec] average time {} [sec]",channels.measurement_interval().as_millis(),channels.average_interval().as_secs());
    let mut start_measurement = std::time::Instant::now();
    let mut rng = thread_rng();

    let tur_sumulate = Normal::new(0.0,100.0).unwrap();
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
                    if let Err(e) = read_tur(&analog) {
                        log::error!("Analog: collect data - {}",e);
                    }
                    // if let Err(e) = kit(&mut dulling_data) {
                        // log::error!("KIT: collect data - {}",e);
                    // }
                }
                tox.push_data(nitri.toxic().as_str()).expect("push data to toxic channel failed");
                dos.push_data(nitri.dosing().as_str()).expect("push data to dosing channel failed");
                ph.push_data(jumo.ph().as_str()).expect("push data to ph channel failed");
                cond.push_data(jumo.cond().as_str()).expect("push data to cond channel failed");
                orp.push_data(jumo.cond().as_str()).expect("push data to orp channel failed");
                temp.push_data(jumo.temp().as_str()).expect("push data to temp channel failed");
                tur.push_value(tur_sumulate.sample(&mut rng)).expect("push simulatiun value to tur channel failed");
                if start_measurement.elapsed().as_secs() > channels.average_interval().as_secs()  {
                    log::info!("CALCULATE DATA!");
                    if let Err(e) = tox.calculate(){
                        log::error!("tox calculate channel value {}", e);
                    }
                    if let Err(e) = dos.calculate(){
                        log::error!("dos calculate channel value {}", e);
                    }
                    if let Err(e) = ph.calculate(){
                        log::error!("ph calculate channel value {}", e);
                    }
                    if let Err(e) = cond.calculate(){
                        log::error!("cond calculate channel value {}", e);
                    }
                    if let Err(e) = orp.calculate(){
                        log::error!("orp calculate channel value {}", e);
                    }
                    if let Err(e) = temp.calculate(){
                        log::error!("temp calculate channel value {}", e);
                    }
                    if let Err(e) = tur.calculate(){
                        log::error!("tur calculate channel value {}", e);
                    }
                    if let Err(e) = channels.history() {
                        log::error!("channels: write history - {}", e);
                    }
                    start_measurement = std::time::Instant::now();
                    let mut data = TSData::new(); 
                    data.field1 = tox.last_value();
                    data.field2 = dos.last_value();
                    data.field3 = ph.last_value();
                    data.field4 = cond.last_value();
                    data.field5 = orp.last_value();
                    data.field6 = temp.last_value();
                    data.field7 = tur.last_value();
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
