extern crate tempfile;

use std::io;
use std::str;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{thread, time};

struct Bitcoind {
    datadir: tempfile::TempDir,
    config_path: String
}

impl Drop for Bitcoind {
    fn drop(&mut self) {
        self.cli(&["stop"]).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn bitcoind() -> Result<Bitcoind, io::Error> {
    let datadir = tempfile::TempDir::new()?;
    let config = datadir.path().join("bitcoin.conf");
    let config_path = config.clone().to_str().unwrap().to_owned();
    let mut config_file = File::create(config)?;
    writeln!(config_file, "rpcuser=regtest")?;
    writeln!(config_file, "rpcpassword=regtest")?;

    println!("starting bitcoind with -conf={}", config_path);
    let _bitcoind = Command::new("bitcoind")
        .arg("-regtest")
        .arg("-daemon")
        .arg(format!("-conf={}", config_path))
        .arg(format!("-datadir={}", datadir.path().to_str().unwrap()))
        .stdout(Stdio::piped())
        .spawn()?;

    thread::sleep(time::Duration::from_secs(2));
    Ok(Bitcoind{datadir, config_path})
}

impl Bitcoind {
    fn cli(&self, args: &[&str]) -> Result<String, io::Error> {
        println!("starting bitcoin-cli {}", args.iter()
            .map(|s| {let mut str = s.to_string(); str.push_str(" "); str})
            .collect::<Vec<_>>().concat());
        let bitcoind = Command::new("bitcoin-cli")
            .arg("-regtest")
            .arg(format!("-conf={}", self.config_path))
            .arg(format!("-datadir={}", self.datadir.path().to_str().unwrap()))
            .arg("-rpcuser=regtest")
            .arg("-rpcpassword=regtest")
            .args(args)
            .stdout(Stdio::piped())
            .spawn()?;

        let output = bitcoind.wait_with_output()?;

        Ok(str::from_utf8(output.stdout.as_slice()).unwrap().to_string())
    }
}

#[test]
fn download() {
    let bitcoind = bitcoind().unwrap();
    bitcoind.cli(&["generate", "500"]).unwrap();
    thread::sleep(time::Duration::from_secs(2));
    //println!("{}", bitcoind.cli(&["getblockchaininfo"]).unwrap());
}