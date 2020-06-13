use crate::Result;
use crate::Workspace;
pub use crate::{Class, Property};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct InkyChan {
    name: String,
    unit: String,
    value: String,
}

#[derive(Debug)]
pub struct InkyState {
    chs: [InkyChan; 4],
    hostname: String,
    online: String,
}

// type Receiver = mpsc::Receiver<InkyState>;
// type Sender = mpsc::Sender<InkyState>;

pub struct Inky {
    path: PathBuf,
}

impl Inky {
    pub fn next(&self) -> Result<()> {
        Ok(())
    }
}
impl Class for Inky {
    const META: &'static str = "inky";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Property for Inky {}

pub fn open(ws: &Workspace) -> Inky {
    let path = ws.rootdir().join(Inky::META);
    Inky { path }
}

pub fn setup(ws: &Workspace) -> Result<Inky> {
    let path = ws.rootdir().join(Inky::META);
    if !path.is_dir() {
        fs::create_dir_all(&path)?;
    }
    // let (sender, receiver) = channel();
    // let ws_new = ws.workspace();
    // thread::spawn(move || rendrer(ws_new, receiver));
    // let (sender, resiver): (Sender, Receiver) = mpsc::channel();
    Ok(Inky { path })
}
