use std::fs;
use std::path::{PathBuf};
use crate::{Workspace,Result};
use rand_distr::{Normal, Distribution};
use rand::thread_rng;


const ADDR: &str = "addr";
const PORT: &str = "port";
const UART: &str = "uart";
const DATA: &str = "data";
const TOX:  &str = "tox";
const DOS:  &str = "dos";
const STATUS:  &str = "status";

// pub struct NitriFields{
//     toxic: f32,
//     dosing: f32,
// }
// fn simulate() -> NitriFields {
    

//     NitriFields{
//         toxic:tox.sample(&mut rng) as f32,
//         dosing:dos.sample(&mut rng) as f32,
//     }
// }

pub struct Nitri {
    path: PathBuf,
}

impl Nitri {
    pub fn addr(&self) -> String {
        fs::read_to_string(self.path.join(ADDR)).unwrap_or("192.168.0.2".to_owned())
    }
    pub fn port(&self) -> i32 {
        if let Ok(number) = fs::read_to_string(self.path.join(PORT)) {
            if let Ok(n) = number.parse::<i32>() {
                return n;
            }
        }
        443 
    }
    pub fn uart(&self) -> String{
        fs::read_to_string(self.path.join(UART)).unwrap_or("/dev/ttyUSB0".to_owned())
    }
    pub fn set_addr(&self,addr:&str) -> Result<()> {
        fs::write(self.path.join(ADDR), addr.trim().as_bytes())?;
        Ok(())
    }
    pub fn set_port(&self,port:&str) -> Result<()> {
        fs::write(self.path.join(PORT), port.trim().as_bytes())?; 
        Ok(())
    }
    pub fn set_uart(&self,uart:&str) -> Result<()> {
        fs::write(self.path.join(UART), uart.trim().as_bytes())?;
        Ok(())
    }
    pub fn toxic(&self) -> String {
        fs::read_to_string(self.path.join(TOX)).unwrap_or("none".to_owned())
    }
    pub fn dosing(&self) -> String {
        fs::read_to_string(self.path.join(DOS)).unwrap_or("none".to_owned())
    }
    pub fn data(&self) -> String {
        fs::read_to_string(self.path.join(DATA)).unwrap_or("none".to_owned())
    }
    pub fn status(&self) -> String {
        fs::read_to_string(self.path.join(STATUS)).unwrap_or("E".to_owned())
    }
    pub fn decode(&self,buf: &[u8]) -> Result<()> {
        let data =  std::str::from_utf8(buf)?;
        fs::write(self.path.join(DATA),data)?;
        let v: Vec<&str> = data.split(';').collect();
        if v.len() > 6 {
            fs::write(self.path.join(TOX), v[3].as_bytes())?;
            fs::write(self.path.join(DOS), v[4].as_bytes())?;
            fs::write(self.path.join(STATUS),"M")?;

        }else{
            fs::write(self.path.join(TOX),"none")?;
            fs::write(self.path.join(DOS),"none")?; 
            fs::write(self.path.join(STATUS),"E")?;
        }
        Ok(())
    }
    pub fn simulate(&self) -> Result<()> {
        fs::write(self.path.join(DATA),"simulate")?;
        fs::write(self.path.join(STATUS),"S")?;
        let mut rng = thread_rng();

        let tox = Normal::new(0.0,100.0).unwrap();
        let dos = Normal::new(0.0,250.0).unwrap();
        let tox = tox.sample(&mut rng);
        let dos = dos.sample(&mut rng);
        fs::write(self.path.join(TOX), format!("{}", tox).trim().as_bytes())?;
        fs::write(self.path.join(DOS), format!("{}", dos).trim().as_bytes())?;
        Ok(())
    }
}

pub fn setup(ws: &Workspace) -> Result<Nitri> {
    let path = ws.rootdir().join("nitri");
    let nitri = Nitri {
        path: path.to_path_buf(),
    };
    if !nitri.path.is_dir() {
        log::info!("Create nitri {}",nitri.path.as_path().display());
        fs::create_dir_all(&nitri.path)?;
    }
    Ok(nitri)
}


pub fn open() -> Result<Nitri> {
    setup(&crate::ws::root())
}