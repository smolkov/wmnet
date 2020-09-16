// use std::fs;
// use std::io;
// use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

/// ✇ init
#[derive(Debug, StructOpt)]
pub struct Init {
    /// 🔧 replace
    #[structopt(name = "replace", long = "replace")]
    replace: bool,
    /// 🔧 git
    #[structopt(name = "git", long = "git")]
    git: bool,
}
/// Init
impl Init {
    /// is replace on setup
    #[inline]
    pub fn replace(&self) -> bool {
        self.replace
    }
    /// is git setup
    #[inline]
    pub fn git(&self) -> bool {
        self.git
    }
}
/// ✇ service
#[derive(Debug, StructOpt)]
pub struct Service {
}
/// ✇ clean
#[derive(Debug, StructOpt)]
pub struct Clean {
    // /// 🔧 replace
// #[structopt(name = "replace", long = "replace")]
// replace: bool,
// /// 🔧 git
// #[structopt(name = "git", long = "git")]
// git: bool,
}
/// ✇ clean
#[derive(Debug, StructOpt)]
pub struct Watch {
    /// 🔧 log
    #[structopt(name = "l", long = "log")]
    log: bool,
}
/// ✇ server cli
#[derive(Debug, StructOpt)]
pub struct Server {
    /// ⏱  interval in seconds
    #[structopt(name = "port", long = "port", default_value = "8080")]
    port: usize,
}

/// ✇ integration signal
#[derive(Debug, StructOpt)]
pub struct Integral {
    /// ⏱  interval in seconds
    #[structopt(name = "count", long = "count", default_value = "20")]
    count: usize,
}

/// ✇ set prop signal
#[derive(Debug, StructOpt)]
pub struct SetProp {
    //⏱ interval in seconds
    #[structopt(short = "d", long = "name")]
    name: PathBuf,
    ///🔌 hardware connection address
    #[structopt(
        short = "a",
        long = "address",
        default_value = "tcp:host=192.168.66.59,port=6666"
    )]
    value: String,
}

/// ✇ get property value
#[derive(Debug, StructOpt)]
pub struct GetProp {
    /// 🔧 name
    #[structopt(short = "d", long = "name")]
    name: PathBuf,
}
/// ✇ list show workspace
#[derive(Debug, StructOpt)]
pub struct List {}
/// List
// impl List {
// }

///📢 Commands
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "service", about = "📢service start")]
    Service(Service),
    #[structopt(name = "init", about = "📢init workspace")]
    Init(Init),
    #[structopt(name = "clean", about = "📢clean workspace")]
    Clean(Clean),
    #[structopt(name = "watch", about = "📢watch workspace")]
    Watch(Watch),
    #[structopt(name = "list", about = "📢show workspace")]
    List(List),
    #[structopt(name = "set", about = "📢set property value")]
    Set(SetProp),
    #[structopt(name = "get", about = "📢get property value")]
    Get(GetProp),
}

///automata command argument
#[derive(Debug, StructOpt)]
#[structopt(name = "wqms", about = "🧰wqms console interface usage.")]
pub struct Args {
    ///📢 subcommands
    #[structopt(subcommand, about = "📢automata commands list")]
    cmd: Cmd,
}

/// 🔧 Activate debug mode
impl Args {
    /// Access the directory name.
    // #[inline]
    // pub fn directory(&self) -> &Path {
    // &self.path
    // }
    /// Commands
    #[inline]
    pub fn commands(&self) -> &Cmd {
        &self.cmd
    }
}
