
use crate::message::Message;
use rocket_contrib::msgpack::MsgPack;
use rocket::{get,post};


#[get("/<id>", format = "msgpack")]
pub fn get(id: u64) -> MsgPack<Message> {
    MsgPack(Message { id: id, contents: "Hello, world!".to_owned(), })
}

#[post("/", data = "<data>", format = "msgpack")]
pub fn create(data: MsgPack<Message>) -> String {
    data.contents.to_string()
}