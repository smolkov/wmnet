

pub async fn start_process() -> Result<()> {

 let mut child = Command::new(name)
 .args(args)
 .stdin(Stdio::null())
 .stdout(Stdio::null())
 .spawn()?;
thread::Builder::new()
 .name("subprocess".into())
 .spawn(move || child.wait())
 .unwrap();
}