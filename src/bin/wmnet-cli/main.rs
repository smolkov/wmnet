// use std::{env, time::Duration};
// use tide::{sessions::SessionMiddleware, Redirect};

// pub mod records;
// mod templates;
// use tide_handlebars::prelude::*;
// use async_std::sync::Arc;
// use async_std::task;
// use std::io::Read;
// use ssh2::Session;
use std::path::PathBuf;
use structopt::clap::Shell;
use structopt::StructOpt;
use wmnet::Result;
// use std::net::TcpStream;

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// Update remote wms
    Update {
        /// Server addresse
        #[structopt(short, long)]
        addr:String,
        /// 🔧 The port
        #[structopt(short, long)]
        port: u16,
    },
    /// Generates shell completion files.
    /// This can be ignored during normal operations.
    Completions {
        /// The target shell. Can be `bash`, `fish`, `powershell`, `elvish` and `zsh`.
        shell: Shell,
        /// The output directory to which the file should be written.
        output_directory: PathBuf,
    },
}
/// ✇ wmnet-cli
#[derive(Debug, StructOpt)]
pub struct Opt {
    /// 🔧 directory
    #[structopt(short, long)]
    path: Option<PathBuf>,

    #[structopt(subcommand)]
    pub cmd: SubCommand,
}
impl Opt {
    /// get path
    #[inline]
    pub fn get_path(&self) -> PathBuf {
        self.path.clone().unwrap_or(PathBuf::from("./"))
    }
}


#[async_std::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    wmnet::logger::debug();
    if let SubCommand::Completions {
        shell,
        output_directory,
    } = &opt.cmd
    {
        let mut clap = Opt::clap();
        clap.gen_completions("wmnet-cli", *shell, output_directory);
        return Ok(());
    }
    // let ssh_addr = format!("{}:{}", opt.get_remote(), opt.get_port());
    // log::info!("Remote PI {}", ssh_addr);
    match opt.cmd {
        SubCommand::Update {
            addr,
            port,
        } => {
            wmnet::rpi::update(&addr,port).await?;
        },
        _ => ()
    }
    // wmnet::rpi::setup().await?;
    Ok(())
}
