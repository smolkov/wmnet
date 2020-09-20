mod prop;
mod dir;
mod cmd;

use futures::{StreamExt};
use telegram_bot::*;
// use std::time::Duration;
use std::sync::Mutex;
// use std::path::{PathBuf};
// use async_trait::async_trait;

// #[async_trait]
// pub trait TextHandler {
    // async fn handle(&mut self, api:Api, message: Message) -> Result<(), Error>;
// }

pub struct BotHandler {

}


pub struct State {
    // command:String,
    // help: String,
    // path: PathBuf,
}


lazy_static::lazy_static!{

}

impl State {
    pub fn new() -> State {
        State{
            // command:"".to_owned(),
            // help:"".to_owned(),
            // path:PathBuf::from("."),
        }
    }
}

use lazy_static::lazy_static;
lazy_static!{
    static ref STATE: Mutex<State> =   Mutex::new(State::new());
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ws = wqms::ws::default();
    let telegram = ws.telegram().unwrap(); 
    println!("TOKEN:{}",telegram.token().trim());
    let api = Api::new(telegram.token().trim());
    let result = api.send(GetMe).await?;
    // let mut state = State::new();
    println!("{:?}", result); 
    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let Err(e) = cmd::handle(api.clone(), message).await {
                log::error!("telegram bot command - {}",e);
            }
        }
    }
    Ok(())
}