use ssh2::Session;
use crate::Result;
use std::io::prelude::*;
use std::net::{TcpStream};

pub struct User {

}

//ssh -R 20004:127.0.0.1:22  root@78.47.241.33
pub async fn tunnel() -> Result<std::process::ExitStatus> {
    let wms = crate::wms::default();
    println!("port{} server{}", wms.remote_port()?, wms.remote_server());
    let mut child = std::process::Command::new("ssh")
        .arg("-R")
        .arg(format!("{}:127.0.0.1:22", wms.remote_port()?.trim()))
        .arg(wms.remote_server())
        .arg("-N")
        .arg("-f")
        .spawn()?;
    let ecode = child.wait()?;
    // log::warn!("ssh stop {}", ecode);
    Ok(ecode)
}

pub async fn test() -> Result<()> {
    // Almost all APIs require a `Session` to be available
    // let sess = Session::new().unwrap();
    // let mut agent = sess.agent().unwrap();

    // Connect the agent and request a list of identities
    // agent.connect().unwrap();
    // agent.list_identities().unwrap();

    // for identity in agent.identities().unwrap() {
        // println!("{}", identity.comment());
        // let _pubkey = identity.blob();
    // }
    Ok(())
}


pub async fn update() -> Result<()> {
      // Connect to the local SSH server
      let tcp = TcpStream::connect("192.168.0.10:22")?;
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
      Ok(())
}