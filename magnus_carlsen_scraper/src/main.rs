use fantoccini::{Client, Locator};

use std::process::Command;

use std::str;

macro_rules! spawn {

    ($f: expr) => {
        std::thread::spawn(move || {
            Command::new($f).output().expect("failed to spawn");
        });
    };

    ($f: expr, $($x: expr),*) => {
        std::thread::spawn(move || {
            Command::new($f)
            $(.arg($x))*.output().expect("failed to spawn");
        });
    };
}

macro_rules! exec {
    ($f: expr) => {
        match Command::new($f).output() {
         Ok(x) => println!("{}", str::from_utf8(&x.stdout).expect("failed to parse")),
         Err(e) => panic!(e.raw_os_error()),
        }
    };

    ($f: expr, $($x: expr),*) => {
        match Command::new($f)
            $(.arg($x))*.output() {
                Ok(x) => println!("{}", str::from_utf8(&x.stdout).expect("failed to parse")),
                Err(e) => panic!(e.raw_os_error()),
            }
    };
}

#[tokio::main]
async fn main() {
    spawn!("geckodriver");

    let mut c = Client::new("http://localhost:4444")
        .await
        .expect("failed to connect");

    c.goto("https://chessgames.com/perl/chessgame?gid=1394457").await.expect("failed to navigate");

    

    c.close().await.expect("failed to close");

    exec!("sudo", "fuser", "-k", "4444/tcp");

    println!("Hello, world!");
}
