use crate::Result;
use async_std::fs;
use std::path::{Path,PathBuf};



async fn unzip(path:&Path) -> Result<()> {
    let _output = std::process::Command::new("unzip").arg(path).output()?;
    Ok(())
}

//https://downloads.raspberrypi.org/raspios_lite_armhf_latest
//https://downloads.raspberrypi.org/raspios_lite_armhf_latest
pub async fn get_image(_path:&str) -> Result<PathBuf> {
    let mut res = surf::get("http://downloads.raspberrypi.org/raspios_lite_armhf/images/raspios_lite_armhf-2020-08-24/2020-08-20-raspios-buster-armhf-lite.zip").await.unwrap();
    let path = PathBuf::from("raspios.zip");
    fs::write(&path,res.body_bytes().await.unwrap()).await?;
    Ok(path)
}


pub async fn setup() -> Result<()> {
    let imagezip = get_image("").await?;
    unzip(&imagezip).await?;
    Ok(())
}

pub async fn update() -> Result<()> {
    Ok(())
}