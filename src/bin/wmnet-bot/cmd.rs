use telegram_bot::*;
use super::prop;
use super::dir;

const HELP: &'static str = r#"
*WQMS* - Soutch Africa

Water quality monitoring station *u.s.w*
*TODO:*

*Commands 📢 :*

/help - help 
/status - status
/getvals - get all channels value
/ls - ls directory
/csv - show csv files
/d - download
/download - download csv file 
/dl - download last csv file

*🔧 Property*
/set - `path_to_value` `value`
/get - `path_to_value`
/wifi - `ssid` `key`
/tswkey - `write_key`
/tsrkey - `read_key`
"#;

pub async fn help(api: Api, message: Message)  -> Result<(), Error> {
    println!("HELP CMD");
    let mut md = message.chat.text(format!("{}",HELP));
    api.send(md.parse_mode(ParseMode::Markdown)).await?;
    prop::help(api,message).await?;
    Ok(())
}


const START: &'static str = r#"
WQMS - Soutch Africa #1

Water quality monitoring station

Show list command type :
/help     - help
"#;

async fn start(api: Api, message: Message) -> Result<(), Error> {
    println!("cmd start");
    api.send(message.chat.text(format!("{}",START))).await?;
    status(api,message).await?;
    Ok(())
}


// pub async fn leave(api: Api, message: Message)-> Result<(), Error> {
//     println!("leve channels");
//     api.send(message.chat.leave()).await?;
//     Ok(())
// }
    
pub async fn channel(api: Api, message: Message)  -> Result<(), Error> {
    let channels = wmnet::wms::default().channels().unwrap();
    println!("channel");
    if let MessageKind::Text { ref data, .. } = message.kind {
        let cmd: Vec<&str> = data.split(' ').collect();
        if cmd.len() > 1 { 
            if let Some(ch) = channels.find(cmd[1]) {
                api.send(message.chat.text(format!("{}",ch.value()))).await?; 
            }
        }else {
            let mut data = String::new();
            let list = channels.list().unwrap();
            for (index,ch) in list.iter().enumerate() {
                if index >0 {
                    data.push(',');
                }
                data.push_str(format!("{}",ch.value()).as_str());
            }
            api.send(message.chat.text(data)).await?;  
        }

    }
    Ok(())
}

async fn system(api: Api, message:Message) -> Result<(), Error> {
    let mut md = String::new();
    md.push_str(format!("*status:* {}\n",wmnet::network::status()).as_str());
    let mut md = message.chat.text(md);
    api.send(md.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}

pub async fn status(api: Api, message: Message)  -> Result<(), Error> {
        println!("status");
        let wms = wmnet::wms::default();
        let channels = wms.channels().unwrap();
        let md = channels.markdown().unwrap_or("*Empty*".to_owned());
        let mut md = message.chat.text(md);
        api.send(md.parse_mode(ParseMode::Markdown)).await?;
        system(api,message).await?;
        Ok(())
}

// pub async fn reboot(api: Api, message: Message) -> Result<(), Error> {
//     println!("info channels");
//     api.send(message.chat.text(format!( "reboot system"))).await?;
//     Ok(())
// }

pub async fn handle(api: Api, message: Message) -> Result<(), Error> {
    // let chat = api.send(message.chat.get_chat()).await?;
    match message.kind {
        MessageKind::Text { ref data, .. } => {

            let cmd: Vec<&str> = data.split(|c| c == ' ' || c == '@').collect();
            println!("COMMAND:{}",cmd[0]);
            match cmd[0]{
                "/start"     => start(api,message).await?,
                "/help"      => help(api, message).await?,
                "/status"    => status(api, message).await?,
                // "/system"    => system(api, message).await?,
                "/getvals"   => channel(api, message).await?,
                // "/reboot"    => reboot(api,message).await?,
                // "/leave"     => leave(api,message).await?,
                "/ls"        => dir::ls(api,message).await?,
                "/csv"       => dir::csv(api,message).await?,
                "/download"  => dir::download(api,message).await?,
                "/d"         => dir::download(api,message).await?,
                "/dl"        => dir::dlast(api,message).await?,
                "/tg"        => dir::dlast(api,message).await?,
                "/get"       => prop::get(api,message).await?,
                "/set"       => prop::set(api,message).await?,
                "/wifi"      => prop::setwifi(api,message).await?,
                "/tswkey"    => prop::tswkey(api,message).await?,
                "/tsrkey"    => prop::tsrkey(api,message).await?,
                // "/forward"   => test_forward(api, message).await?,
                // "/autorise" => autorise(api,message)?,
                // "/channel"  => channel(api,message)?,
                _ =>         help(api, message).await?,
            }
        }
        _ => (),
    }
    Ok(())
}