use telegram_bot::*;
use crate::prop;
use crate::dir;

const HELP: &'static str = r#"
*WQMS* - Soutch Africa

Water quality monitoring station *u.s.w*
*TODO:*

*Commands:*

/start    - start
/help     - help 
/channel  - channel
/system   - system information 
/status   - status
/reboot   - reboot
/leave    - leave
/ls       - ls directory
/csv      - show&download csv data
"#;

pub async fn help(api: Api, message: Message)  -> Result<(), Error> {
    println!("HELP CMD");
    let mut md = message.chat.text(format!("{}",HELP));
    api.send(md.parse_mode(ParseMode::Markdown)).await?;
    prop::help(api,message).await?;
    Ok(())
}




async fn start(api: Api, message: Message) -> Result<(), Error> {
const MARKDOWN: &'static str = r#"
*bold \*text*
_italic \*text_
__underline__
~strikethrough~
*bold _italic bold ~italic bold strikethrough~ __underline italic bold___ bold*
[inline URL](http://www.example.com/)
[inline mention of a user](tg://user?id=123456789)
`inline fixed-width code`
```
pre-formatted fixed-width code block
```
```python
pre-formatted fixed-width code block written in the Python programming language
```
"#;

const HTML: &'static str = r#"
<b>bold</b>, <strong>bold</strong>
<i>italic</i>, <em>italic</em>
<u>underline</u>, <ins>underline</ins>
<s>strikethrough</s>, <strike>strikethrough</strike>, <del>strikethrough</del>
<b>bold <i>italic bold <s>italic bold strikethrough</s> <u>underline italic bold</u></i> bold</b>
<a href="http://www.example.com/">inline URL</a>
<a href="tg://user?id=123456789">inline mention of a user</a>
<code>inline fixed-width code</code>
<pre>pre-formatted fixed-width code block</pre>
<pre><code class="language-python">pre-formatted fixed-width code block written in the Python programming language</code></pre>
"#;
    println!("cmd start");
    api.send(message.chat.text(format!( "START commad"))).await?;
    
    let mut html = message.chat.text(format!("{}",HTML));
    api.send(html.parse_mode(ParseMode::Html)).await?;
    let mut md = message.chat.text(format!("{}",MARKDOWN));
    api.send(md.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}


pub async fn leave(api: Api, message: Message)-> Result<(), Error> {
    println!("leve channels");
    api.send(message.chat.leave()).await?;
    Ok(())
}
    
pub async fn channel(api: Api, message: Message)  -> Result<(), Error> {
    let channels = wqms::ws::root().channels().unwrap();
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

pub async fn system(api: Api, message:Message) -> Result<(), Error> {
    // let ws = wqms::ws::root();
    let mut md = String::new();
    if wqms::network::online() {
        md.push_str("*status:* `ONLINE`\n");
    }else {
        md.push_str("*status:* `OFFLINE`\n");
    }
    let mut md = message.chat.text(md);
    api.send(md.parse_mode(ParseMode::Markdown)).await?;
    Ok(())
}

pub async fn status(api: Api, message: Message)  -> Result<(), Error> {
        println!("status");
        let ws = wqms::ws::root();
        let channels = ws.channels().unwrap();
        let md = channels.markdown().unwrap_or("*Empty*".to_owned());
        let mut md = message.chat.text(md);
        api.send(md.parse_mode(ParseMode::Markdown)).await?;
        system(api,message).await?;
        Ok(())
}

pub async fn reboot(api: Api, message: Message) -> Result<(), Error> {
    println!("info channels");
    api.send(message.chat.text(format!( "reboot system"))).await?;
    Ok(())
}

pub async fn handle(api: Api, message: Message) -> Result<(), Error> {
    // let chat = api.send(message.chat.get_chat()).await?;
    match message.kind {
        MessageKind::Text { ref data, .. } => {
            println!("COMMAND:{}",data);
            let cmd: Vec<&str> = data.split(' ').collect();
            match cmd[0] {
                "/start"     => start(api,message).await?,
                "/help"      => help(api, message).await?,
                "/system"    => system(api, message).await?,
                "/channel"   => channel(api, message).await?,
                "/status"    => status(api, message).await?,
                "/reboot"    => reboot(api,message).await?,
                "/leave"     => leave(api,message).await?,
                "/ls"        => dir::ls(api,message).await?,
                "/csv"       => dir::csv(api,message).await?,
                "/get"       => prop::get(api,message).await?,
                "/set"       => prop::set(api,message).await?,
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