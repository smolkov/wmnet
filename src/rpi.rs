use crate::Result;
use std::fs;
use std::path::{Path,PathBuf};
use ssh2::Session;
use std::io::prelude::*;
use std::net::{TcpStream};


async fn unzip(path:&Path) -> Result<()> {
    let _output = std::process::Command::new("unzip").arg(path).output()?;
    Ok(())
}

//https://downloads.raspberrypi.org/raspios_lite_armhf_latest
//https://downloads.raspberrypi.org/raspios_lite_armhf_latest
pub async fn get_image(_path:&str) -> Result<PathBuf> {
    let mut _res = surf::get("http://downloads.raspberrypi.org/raspios_lite_armhf/images/raspios_lite_armhf-2020-08-24/2020-08-20-raspios-buster-armhf-lite.zip").await.unwrap();
    let path = PathBuf::from("raspios.zip");
    // fs::write(&path,res.body_bytes().await.unwrap())?;
    Ok(path)
}


pub async fn setup() -> Result<()> {
    let imagezip = get_image("").await?;
    unzip(&imagezip).await?;
    Ok(())
}

pub async fn update(addr:&str,port:u16) -> Result<()> {
    log::info!("Update {}:{}",addr,port);
    let addr = format!("{}:{}",addr,port);
    let tcp = TcpStream::connect(&addr)?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    // Try to authenticate with the first identity in the agent.
    sess.userauth_agent("pi")?;

    // Make sure we succeeded
    let mut channel = sess.channel_session()?;
    channel.exec("ls")?;
    let mut s = String::new();
    channel.read_to_string(&mut s)?;
    println!("{}", s);
    channel.wait_close()?;
    println!("{}", channel.exit_status()?);
    sess.channel_session()?.exec("mkdir .update")?;
    sess.channel_session()?.exec("mkdir -p wms/bin")?;
 
    // channel.wait_close()?;
    // println!("{}", channel.exit_status()?);
    let relase = PathBuf::from("target/arm-unknown-linux-gnueabihf/release");
    let update = PathBuf::from(".update");
    
    let mut remote_file  = sess.scp_send(&update.join("wmnet"), 0o777, 10, None)?;
    println!("test1");
    remote_file.write(&fs::read(relase.join("wmnet"))?)?;

    let mut remote_file  = sess.scp_send(&update.join("wmnet-inky"),0o777, 10, None)?;
    remote_file.write(&fs::read(relase.join("wmnet-inky"))?)?;

    let mut remote_file  = sess.scp_send(&update.join("wmnet-bot"),  0o777, 10, None)?;
    remote_file.write(&fs::read(relase.join("wmnet-bot"))?)?;

    let mut remote_file  = sess.scp_send(&update.join("wmnet-collect"),  0o777, 10, None)?;
    remote_file.write(&fs::read(relase.join("wmnet-collect"))?)?;

    sess.channel_session()?.exec("rm wms/bin/*")?;

    sess.channel_session()?.exec("mv .update/* wms/bin/")?;
    sess.channel_session()?.exec("rm -rf .update")?;

    println!("{}", channel.exit_status()?);
    Ok(())
}
