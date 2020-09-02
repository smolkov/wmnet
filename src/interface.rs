// use crate::ws::workdir;
use crate::Result;
// use std::fmt::Display;
// use crate::workdir;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub trait ToDir {
    fn to_dir(self) -> PathBuf;
}


pub trait Class {
    const META: &'static str = "class";
    const UUID: &'static str = "nan";
    const LABEL: &'static str = "label";
    const DESC: &'static str = "desk";
    /// path
    fn path(&self) -> &Path;
    fn label(&self) -> String {
        fs::read_to_string(self.path().join("label")).unwrap_or(Self::LABEL.to_owned())
    }
    fn set_label(&self, label: &str) -> Result<()> {
        fs::write(self.path().join("label"), label.trim())?;
        Ok(())
    }
    fn descriprion(&self) -> String {
        fs::read_to_string(self.path().join("description")).unwrap_or(Self::DESC.to_owned())
    }
    fn set_description(&self, desc: &str) -> Result<()> {
        fs::write(self.path().join("description"), desc)?;
        Ok(())
    }
    fn get_name(&self) -> String {
        self.path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("noname")
            .to_owned()
    }
    fn meta(&self) -> String {
        let path = self.path().join("meta");
        if !path.is_file() {
            if let Err(e) = fs::write(&path, Self::META) {
                log::error!("class[{}] create meta - {}", path.display(), e);
            }
        }
        fs::read_to_string(&path).unwrap_or(Self::META.to_owned())
    }
    fn uuid(&self) -> String {
        let path = self.path().join("uuid");
        fs::read_to_string(&path).unwrap_or(Self::UUID.to_owned())
    }
    fn setup(&self) -> Result<()> {
        if !self.path().is_dir() {
            log::debug!("setup new class directory {}", self.path().display());
            fs::create_dir_all(self.path())?;
            self.set_label(Self::LABEL)?;
            self.set_description(Self::DESC)?;
            self.meta();
            self.uuid();
        }
        Ok(())
    }
}
pub trait Property: Class {
    // type Value: String; //  From<String> + Into<String> + Display;
    fn get(&self, name: &str) -> Option<String> {
        if let Ok(value) = fs::read_to_string(self.path().join(name)) {
            Some(value)
        } else {
            None
        }
    }
    fn set(&self, name: &str, value: &str) -> Result<()> {
        fs::write(self.path().join(name), value.as_bytes())?;
        Ok(())
    }
}

pub trait Statistic: Class {
    const INTERVAL: u64 = 1000;
    const AVERAGE: u64 = 60;
    const OUTLIERS: u16 = 0;
    const MAXCV: f32 = 2.5;
    /// Average interval
    /// default 60 seconds
    fn average(&self) -> std::time::Duration {
        if let Ok(sec) = fs::read_to_string(self.path().join("interval")) {
            if let Ok(sec) = sec.parse::<u64>() {
                return std::time::Duration::from_secs(sec);
            }
        }
        std::time::Duration::from_secs(Self::AVERAGE)
    }
    /// Measurement interval
    /// default 1000 milliseconds
    fn interval(&self) -> std::time::Duration {
        if let Ok(millis) = fs::read_to_string(self.path().join("average")) {
            if let Ok(millis) = millis.parse::<u64>() {
                return std::time::Duration::from_millis(millis);
            }
        }
        std::time::Duration::from_millis(Self::INTERVAL)
    }
    /// Outliers
    /// default 0
    fn outliers(&self) -> u16 {
        if let Ok(outliers) = fs::read_to_string(&self.path().join("outliers")) {
            if let Ok(outliers) = outliers.parse::<u16>() {
                return outliers;
            }
        }
        Self::OUTLIERS
    }
    /// Max CV
    /// default :2.5
    fn maxcv(&self) -> f32 {
        if let Ok(maxcv) = fs::read_to_string(self.path().join("maxcv")) {
            if let Ok(maxcv) = maxcv.parse::<f32>() {
                return maxcv;
            }
        }
        Self::MAXCV
    }
}
