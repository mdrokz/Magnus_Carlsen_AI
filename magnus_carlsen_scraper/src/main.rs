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

    c.goto("https://chessgames.com/perl/chessgame?gid=1394457")
        .await
        .expect("failed to navigate");

    let play_button = c
        .find(Locator::Id("nextB"))
        .await
        .expect("could not find play button");

    let mut board = c
        .find(Locator::Css(".board-b72b1"))
        .await
        .expect("failed to find chess board")
        .find_all(Locator::Css("div[class^=row"))
        .await
        .expect("failed to find rows");

    let mut count = 0;

    play_button.click().await.expect("failed to click");

    let board_len = board.len();

    let mut fen_string = String::new();

    for (index, b) in board.iter_mut().enumerate() {
        let squares = b
            .find_all(Locator::Css("div[class^=square]"))
            .await
            .expect("failed to fetch find squares");

        println!("square len {}", squares.len());

        for mut square in squares {
            match square.find(Locator::Css("img")).await {
                Ok(mut img) => {
                    if count > 0 {
                        fen_string.push_str(&count.to_string())
                    }
                    count = 0;

                    fen_string.push_str(
                        &img.attr("data-piece")
                            .await
                            .unwrap()
                            .unwrap()
                            // .replace("b", "")
                            // .replace("w", ""),
                    )
                }
                Err(e) => {
                    count = count + 1;
                }
            }
        }
        if count > 0 {
            fen_string.push_str(&count.to_string())
        }
        count = 0;
        if index != board_len {
            fen_string.push_str("/");
        }
        // for mut im in img {
        // println!("html {}",im.html(false).await.expect("msg"));
        // }
    }

    println!("FEN {}", fen_string);

    // for _ in 0..86 {
    //     let p = play_button.clone();

    //     p.click().await.expect("failed to click");
    // }

    std::thread::sleep(std::time::Duration::from_millis(10000));

    c.close().await.expect("failed to close");

    exec!("sudo", "fuser", "-k", "4444/tcp");

    println!("Hello, world!");
}
