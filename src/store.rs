// use crate::workdir;
// use crate::Result;
// use sled::Config;
// use std::collections::HashMap;
use redis::{self, Commands};

pub struct Store {
    con : redis::Connection,
} 

impl Store {
    pub fn new(addr: &str) -> redis::RedisResult<Store> {
        let client = redis::Client::open(addr)?;
        let con = client.get_connection()?;
        Ok(Store{
            con: con,
        })
    }
    pub fn thinkspeak_readkey(&mut self) -> redis::RedisResult<String> {
        let read_key : String = self.con.get("thinkspeak.api.readkey")?;
        Ok(read_key)
    }
    pub fn tramsmit_thingspeak(&self)-> redis::RedisResult<()>{

        Ok(())
    }
    pub fn transmit(&self) {
        // if let Err(e) = tramsmit_thingspeak(self) {
            // log::error!("Transmit data to thingspeak error {:?}",e);
        // }
    }
    pub fn put_tox(&mut self, value:f32) -> redis::RedisResult<()> {
        self.con.set("channel.tox.value", value)?;
        Ok(())
    }
    pub fn put_dos(&mut self, value:f32)  -> redis::RedisResult<()>  {
        self.con.set("channel.dos.value", value)?;
        Ok(())
    }
    pub fn put_ec(&mut self, value : f32)  -> redis::RedisResult<()> {
        self.con.set("channel.ec.value", value)?;
        Ok(())
    }
    pub fn put_orp(&mut self, value : f32)  -> redis::RedisResult<()> {
        self.con.set("channel.orp.value", value)?;
        Ok(())
    }
    pub fn put_cond(&mut self, value : f32)  -> redis::RedisResult<()> {
        self.con.set("channel.cond.value", value)?;
        Ok(())
    }
    pub fn put_dulling(&mut self, value : f32)  -> redis::RedisResult<()> {
        self.con.set("channel.cond.value", value)?;
        Ok(())
    }
}