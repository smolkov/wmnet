use serde::{Deserialize, Serialize};
use crate::Workspace;
// use glob::glob;
// use std::path::Path;
// Define a model. Simple as deriving a few traits.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prop {
    pub id: String,
    pub path: String,
    pub value: String,
}

pub fn list(ws:&Workspace) -> Vec<Prop> {
    let mut props: Vec<Prop> = Vec::new();
    // for entry in glob(format!("{}/**/*",rootpath.display()).as_str()).unwrap(){     
    for entry in glob::glob(format!("{}/**/*",ws.rootdir().display()).as_str()).unwrap(){     
        let p = entry.unwrap();
        if !p.is_dir() && p.extension().is_none() {
            let value = std::fs::read_to_string(&p).unwrap();
            // let name = wqms::util::name_from_path(&p);
            let path = ws.truncate(&p);
            println!("Check path:{}",path.display());
            if crate::util::hiddens(&path)==0 {
                let path = format!("{}",path.display());
                let id = path.replace("/", "-");
                props.push(Prop{id,path,value})
            }
       }
    }
    props
}

pub fn get(ws:&Workspace,id:&str) -> Prop {
    let idnew = id.replace("-", "/");

    let path = ws.rootdir().join(idnew);
    let value = std::fs::read_to_string(&path).unwrap_or(String::from("none")); 
    let path = ws.truncate(&path);
    Prop{
        id: id.to_owned(),
        path: format!("{}",path.display()),
        value: value,
    }
}