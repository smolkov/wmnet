use std::path::Path;
use telegram_bot::*;
use wqms::iface::Class;



fn list(path:&Path) -> wqms::Result<String> {
    let mut data = String::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap();
        if path.is_dir() {
            data.push_str(format!("{}/\n",name.to_str().unwrap()).as_str());
        } else {
            data.push_str(format!("{}\n",name.to_str().unwrap()).as_str());
        }
    } 
    Ok(data)
}


pub async fn ls(api: Api, message: Message)  -> Result<(), Error> {
    println!("ls");
    let ws = wqms::ws::root();
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        let data = match cmd.len() {
            1 => list(ws.rootdir()),
            2 => list(ws.rootdir().join(cmd[1]).as_path()),
            _ => list(ws.rootdir()),
        };
        api.send(message.chat.text(data.as_str())).await?;
    }
    Ok(())
}


pub async fn csv(api: Api, message:Message) -> Result<(),Error> {
    let channels = wqms::ws::root().channels().unwrap();
    println!("csv");
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();

        match cmd.len() {
           2 => {
                
            }, 
            _ => {
                api.send(message.chat.text(list(channels.path()).as_str())).await?; 
            },
        };
    }
    Ok(())
}

