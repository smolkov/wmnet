use super::Chan;
use crate::Class;
use crate::Workspace;

pub struct Tox {
    path: PathBuf,
}
pub struct Dosing {
    path: PathBuf,
}

impl Class for Tox {
    const META: &'static str = "TOX";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Property for Tox {}
impl Chan for Tox {
    const NUMBER: u16 = 1;
}
impl Class for Dosing {
    const META: &'static str = "DOS";
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Chan for Dosing {
    const NUMBER: u16 = 2;
}

impl Property for Dosing {}

pub fn tox(path: PathBuf) -> Tox {}
