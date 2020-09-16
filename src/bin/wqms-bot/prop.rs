// use futures::{StreamExt};
use telegram_bot::*;
// use std::time::Duration;
// use std::sync::Mutex;
// use std::path::{PathBuf};

const HELP: &'static str = r#"
*🔧 Property*

*/set* `path_to_value` `value`
*/get* `path_to_value`
*/wifi* `ssid` `key`
*/tswkey* `write_key`
*/tsrkey* `read_key`
"#;


pub async fn help(api:Api,message:Message) -> Result<(),Error> {
    let mut md = message.chat.text(format!("{}",HELP));
    api.send(md.parse_mode(ParseMode::Markdown)).await?;
    Ok(())

}


pub async fn tswkey(api: Api, message: Message) -> Result<(),Error> {
    let th = wqms::ws::default().thingspeak().unwrap();
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        if cmd.len() >1 {
            if let Err(e) = th.set_wkey(cmd[1]) {
                api.send(message.chat.text(format!("write thingspeak write key {} - {}",cmd[1],e))).await?; 
            }
        }else {
            api.send(message.chat.text(format!("set thingspeak write api {}",data))).await?; 
            help(api,message).await?;
        }
    }
    Ok(())
}

pub async fn tsrkey(api: Api, message: Message) -> Result<(),Error> {
    let th = wqms::ws::default().thingspeak().unwrap();
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        if cmd.len() > 1 {
            if let Err(e) = th.set_rkey(cmd[1]) {
                api.send(message.chat.text(format!("write thingspeak read key {} - {}",cmd[1],e))).await?; 
            }
        }else {
            api.send(message.chat.text(format!("set thingspeak write api {}",data))).await?; 
            help(api,message).await?;
        }
    }
    Ok(())
}

pub async fn setwifi(api: Api, message: Message) -> Result<(),Error> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        if cmd.len() > 2 {

        } else {
            api.send(message.chat.text(format!("change wifi credential wrond data {}",data))).await?; 
            help(api,message).await?;
        }
    }
    Ok(())
}


pub async fn set(api: Api, message: Message)  -> Result<(), Error> {
    let ws = wqms::ws::default();
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        if cmd.len() > 2 {
            let path = ws.rootdir().join(cmd[1]);
            if let Err(e) = std::fs::write(path, format!("{}", cmd[2]).trim().as_bytes()) {
                api.send(message.chat.text(format!("set prop {} write value {} - {}",cmd[1],cmd[2],e))).await?;
            }else {
                api.send(message.chat.text(format!("prop {} valeu changed {} ",cmd[1],cmd[2]))).await?;
            }
        }else {
            api.send(message.chat.text(format!("set properte wrong format {}
                /set directory/prop  value
            ",data))).await?;

        }
    }
    Ok(())
}

pub async fn get(api: Api, message: Message)  -> Result<(), Error> {
    let ws = wqms::ws::default();
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        if cmd.len() > 1 {
            let path = ws.rootdir().join(cmd[1]);
            match std::fs::read_to_string(path) {
                 Ok(v)  => {api.send(message.chat.text(v)).await?;},
                 Err(e) => { println!("Err{}",e);api.send(message.chat.text(format!("get prop {}  - {}",cmd[1],e))).await?;},
            }
        }else {
            api.send(message.chat.text(format!("set properte wrong format {}
                /get path/prop
            ",data))).await?;

        }
    }
    Ok(())
}

// pub async fn handle(api: Api, message: Message) -> Result<(), Error> {
//     if let MessageKind::Text { ref data, .. } = message.kind {
//         let cmd: Vec<&str> = data.split(' ').collect();
//         let addr = if cmd.len() > 1 {
//             cmd[1]
//         }else {
//             "wrong_prop"
//         };
//         match cmd[0] {
//             "set"       => set(api,message).await?,
//             "get"       => get(api,message).await?,
//             "wifi"      => setwifi(api,message).await?,
//             "tswkey"    => tswkey(api,message).await?,
//             "tsrkey"    => tsrkey(api,message).await?,
//             _ => {
//                 help(api,message).await?;
//             },
//         }
//     }
//     Ok(())
// }
