/// JUMO Aqua

use std::fs;
use std::path::{PathBuf};
use crate::{Workspace,Result};
use rand_distr::{Normal, Distribution};
use rand::thread_rng;

const ADDR: &str = "addr";
const PORT: &str = "port";
const STATUS: &str = "status";
const DATA: &str = "status";

const ORP: &str = "orp";
const PH: &str = "ph";
const COND: &str = "cond"; //LEITFAHIGKEIT
const TEMP: &str = "temperatur";
pub const ORP_REG: u16 = 0x164D;
pub const PH_REG: u16 = 0x164F;
pub const COND_REG: u16 = 0x1651;
pub const TEMP_REG: u16 = 0x1651;
pub struct Jumo {
    path: PathBuf,
}

fn modbus_value(b1 : u16 , b2: u16) -> f32  {
    let b1 = b1.to_be_bytes();
    let b2 = b2.to_be_bytes();
    let value = f32::from_be_bytes([b2[0],b2[1], b1[0], b1[1]]);
    value
}

impl Jumo {
    pub fn addr(&self) -> String {
        fs::read_to_string(self.path.join(ADDR)).unwrap_or("192.168.0.2".to_owned())
    }
    pub fn port(&self) -> i32 {
        if let Ok(number) = fs::read_to_string(self.path.join(PORT)) {
            if let Ok(n) = number.parse::<i32>() {
                return n;
            }
        }
        502 
    }
    pub fn set_address(&self,addr:&str) -> Result<()> {
        fs::write(self.path.join(ADDR), addr.trim().as_bytes())?;
        Ok(())
    }
    pub fn set_port(&self,port:&str) -> Result<()> {
        fs::write(self.path.join(PORT), port.trim().as_bytes())?; 
        Ok(())
    }
    pub fn orp(&self) -> String {
        fs::read_to_string(self.path.join(ORP)).unwrap_or("none".to_owned())
    }
    pub fn ph(&self) -> String {
        fs::read_to_string(self.path.join(PH)).unwrap_or("none".to_owned())
    }
    /// Leitfähigkeit
    pub fn cond(&self) -> String {
        fs::read_to_string(self.path.join(COND)).unwrap_or("none".to_owned())
    }
    /// Temperatur 
    pub fn temp(&self) -> String {
        fs::read_to_string(self.path.join(TEMP)).unwrap_or("none".to_owned())
    }
    pub fn data(&self) -> String {
        fs::read_to_string(self.path.join(DATA)).unwrap_or("none".to_owned())
    }
    pub fn status(&self) -> String {
        fs::read_to_string(self.path.join(STATUS)).unwrap_or("E".to_owned())
    }
    pub fn simulate(&self) -> Result<()> {
        fs::write(self.path.join(DATA),"simulate")?;
        fs::write(self.path.join(STATUS),"S")?;
        let mut rng = thread_rng();
        let orp = Normal::new(0.0,1000.0).unwrap();
        let ph = Normal::new(0.0,14.0).unwrap();
        let cond = Normal::new(0.0,2.0).unwrap();
        let temp = Normal::new(0.0,100.0).unwrap();
        let orp = orp.sample(&mut rng);
        let ph = ph.sample(&mut rng);
        let cond = cond.sample(&mut rng);
        let temp = temp.sample(&mut rng);
        fs::write(self.path.join(ORP), format!("{}", orp).trim().as_bytes())?;
        fs::write(self.path.join(PH), format!("{}", ph).trim().as_bytes())?;
        fs::write(self.path.join(COND), format!("{}", cond).trim().as_bytes())?;
        fs::write(self.path.join(TEMP), format!("{}", temp).trim().as_bytes())?;
        Ok(())
    }
    pub fn set_orp(&self,b1:u16,b2:u16) -> Result<()> {
        fs::write(self.path.join(ORP), format!("{}", modbus_value(b1,b2)).trim().as_bytes())?;
        Ok(())
    }
    pub fn set_ph(&self,b1:u16,b2:u16) -> Result<()> {
        fs::write(self.path.join(PH), format!("{}", modbus_value(b1,b2)).trim().as_bytes())?;
        Ok(())
    }
    pub fn set_cond(&self,b1:u16,b2:u16) -> Result<()> {
        fs::write(self.path.join(COND), format!("{}", modbus_value(b1,b2)).trim().as_bytes())?;
        Ok(())
    }
    pub fn set_temp(&self,b1:u16,b2:u16) -> Result<()> {
        fs::write(self.path.join(TEMP), format!("{}", modbus_value(b1,b2)).trim().as_bytes())?;
        Ok(())
    }
    pub fn set_status(&self,status:&str) -> Result<()> {
        fs::write(self.path.join(STATUS),status.trim())?;
        Ok(())
    }
    pub fn decode(&self,orp:(u16,u16),ph:(u16,u16),cond:(u16,u16),temp:(u16,u16)) -> Result<()> {
        self.set_orp(orp.0,orp.1)?;
        self.set_ph(ph.0,ph.1)?;
        self.set_cond(cond.0,ph.1)?;
        self.set_temp(temp.0,ph.1)?;
        Ok(())
    }
}

pub fn setup(ws: &Workspace) -> Result<Jumo> {
    let path = ws.rootdir().join("jumo");
    let jumo = Jumo {
        path: path.to_path_buf(),
    };
    if !jumo.path.is_dir() {
        log::info!("Create jumo device {}",jumo.path.as_path().display());
        fs::create_dir_all(&jumo.path)?;
    }
    Ok(jumo)
}


pub fn open() -> Result<Jumo> {
    setup(&crate::ws::root())
}