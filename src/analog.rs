use std::fs;
use std::path::{PathBuf};
use crate::{Workspace,Result};
use rand_distr::{Normal, Distribution};
use rand::thread_rng;


const ADDR:   &str = "addr";
const DATA:   &str = "data";
const VALUE:  &str = "value";
const VOLT:   &str = "volt";
const AMPER:  &str = "amper";
const STATUS: &str = "status";

///  Analog
pub struct Analog {
    path: PathBuf,
}

impl Analog {
    /// Addres
    pub fn addr(&self) -> String {
        fs::read_to_string(self.path.join(ADDR)).unwrap_or("192.168.0.2".to_owned())
    }
    /// Change addr
    pub fn set_addr(&self,addr:&str) -> Result<()> {
        fs::write(self.path.join(ADDR), addr.trim().as_bytes())?;
        Ok(())
    }
    pub fn value(&self) -> String {
        fs::read_to_string(self.path.join(VALUE)).unwrap_or("none".to_owned())
    }
    pub fn volt(&self) -> String {
        fs::read_to_string(self.path.join(VOLT)).unwrap_or("none".to_owned())
    }
    pub fn amper(&self) -> String {
        fs::read_to_string(self.path.join(AMPER)).unwrap_or("none".to_owned())
    }
    pub fn data(&self) -> String {
        fs::read_to_string(self.path.join(DATA)).unwrap_or("none".to_owned())
    }
    pub fn status(&self) -> String {
        fs::read_to_string(self.path.join(STATUS)).unwrap_or("E".to_owned())
    }
    pub fn set_value(&self,value:f32) -> Result<()> {
        fs::write(self.path.join(VALUE), format!("{}",value).trim().as_bytes())?;
        Ok(())
    }
    pub fn simulate(&self) -> Result<()> {
        fs::write(self.path.join(DATA),"simulate")?;
        fs::write(self.path.join(STATUS),"S")?;
        let mut rng = thread_rng();
        let value = Normal::new(0.0,5.0).unwrap();
        let value = value.sample(&mut rng);
        fs::write(self.path.join(VALUE), format!("{}", value).trim().as_bytes())?;
        Ok(())
    }
}

pub fn setup(ws: &Workspace) -> Result<Analog> {
    let path   = ws.rootdir().join("nitri");
    let analog = Analog{
        path: path.to_path_buf(),
    };
    if !analog.path.is_dir() {
        log::info!("Create nitri {}",analog.path.as_path().display());
        fs::create_dir_all(&analog.path)?;
    }
    Ok(analog)
}


pub fn open() -> Result<Analog> {
    setup(&crate::ws::root())
}