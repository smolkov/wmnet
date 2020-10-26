
use async_std::{
    prelude::*,
    task,
};
use wmnet::Result;
// use std::path::Path;
// use wmnet::wms::Workspace;
#[allow(dead_code)]
pub async fn collect() -> Result<()> {
    let wms = wmnet::wms::default();
    loop {
        let mut child = std::process::Command::new(wms.bindir().join("wmnet-collect")).spawn()?;
        let ecode = child.wait()?;
        log::warn!("data collect stop {}",ecode);
    }
    // Ok(())
}
#[allow(dead_code)]
pub async fn telebot() -> Result<()> {
    let wms = wmnet::wms::default();
    loop {
        let mut child = std::process::Command::new(wms.bindir().join("wmnet-bot")).spawn()?;
        let ecode = child.wait()?;
        log::warn!("data telebot stop {}",ecode);
    }
    // Ok(())
}

#[allow(dead_code)]
async fn inky() -> Result<()> {
    let wms = wmnet::wms::default();
    loop {
        let mut child = std::process::Command::new(wms.bindir().join("wmnet-inky")).spawn()?;
        let ecode = child.wait()?;
        log::warn!("data collect stop {}",ecode);
    }
    // Ok(())
}
#[allow(dead_code)]
pub async fn app_loop() -> Result<()> { // 1
    let collect_task = task::spawn(async {
        let result= collect().await;
        match result {
            Ok(_) => println!("Collect OK"),
            Err(e) => println!("Error reading file: {:?}", e)
        } 
    });
    let bot_task = task::spawn(async {
        match telebot().await {
            Ok(_) => println!("Telegot OK"),
            Err(e) => println!("Error reading file: {:?}", e) 
        }
    }); 
    collect_task.join(bot_task).await;
    Ok(())
}