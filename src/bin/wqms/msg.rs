use rocket::{get, post};
use rocket_contrib::msgpack::MsgPack;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub contents: String,
}

#[get("/<id>", format = "msgpack")]
pub fn get(id: u64) -> MsgPack<Message> {
    MsgPack(Message {
        id: id,
        contents: "Hello, world!".to_owned(),
    })
}

#[post("/", data = "<data>", format = "msgpack")]
pub fn create(data: MsgPack<Message>) -> String {
    data.contents.to_string()
}
