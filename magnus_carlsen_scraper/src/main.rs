#![feature(wake_trait)]

use fantoccini::{Client, Element, Locator};
use fs::write;
use tokio::runtime::Runtime;

use std::process::Command;

use std::str;

use serde_json::json;

use std::fs;


mod executor;

mod waker;

use crate::executor::{Executor,Spawner,new_executor_and_spawner};

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

async fn generate_fen_string(mut board: Vec<Element>) -> String {
    let mut count = 0;

    let board_len = board.len();

    let mut fen_string = String::new();

    for (index, b) in board.iter_mut().enumerate() {
        let squares = b
            .find_all(Locator::Css("div[class^=square]"))
            .await
            .expect("failed to fetch find squares");

        for mut square in squares {
            match square.find(Locator::Css("img")).await {
                Ok(mut img) => {
                    if count > 0 {
                        fen_string.push_str(&count.to_string())
                    }
                    count = 0;

                    let data_piece = img
                        .attr("data-piece")
                        .await
                        .expect("no attribute found")
                        .unwrap();

                    if data_piece.contains('b') {
                        let piece_str = data_piece.replace('b', "").to_lowercase();
                        fen_string.push_str(&piece_str);
                    } else if data_piece.contains('w') {
                        fen_string.push_str(&data_piece.replace('w', ""));
                    }
                }
                Err(_) => {
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
    }

    fen_string
}

#[tokio::main]
async fn main() {
    spawn!("geckodriver");

    // let mut c = Client::new("http://localhost:4444")
    //     .await
    //     .expect("failed to connect");
    let mut fen_strings: Vec<String> = Vec::new();

    let (executor, spawner): (Executor<i32>,Spawner<i32>) = new_executor_and_spawner();


    spawner.spawn(async {
        32
    });

    spawner.spawn(async {
        32
    });

    spawner.spawn(async {
        32
    });

    spawner.spawn(async {
        32
    });

    
    drop(spawner);
    

    executor.run();

    // for _ in 0..3 {
    //     c.new_window(false).await.unwrap();
    // }

    // c.goto("https://chessgames.com/perl/chessgame?gid=1394457")
    //     .await
    //     .expect("failed to navigate");

    // let play_button = c
    //     .find(Locator::Id("nextB"))
    //     .await
    //     .expect("could not find play button");

    // for _ in 0..86 {
    //     let p = play_button.clone();

    //     let mut rt = Runtime::new().unwrap();

    //     rt.block_on(async {
    //         let board = c
    //         .find(Locator::Css(".board-b72b1"))
    //         .await
    //         .expect("failed to find chess board")
    //         .find_all(Locator::Css("div[class^=row"))
    //         .await
    //         .expect("failed to find rows");
    //         fen_strings.push(generate_fen_string(board).await);
    //     });
        
    //     p.click().await.expect("failed to click");
    // }

    // write(
    //     "./moves.json",
    //     json!({
    //         "title": "Magnus Carlsen vs T",
    //         "moves": fen_strings
    //     })
    //     .to_string()
    //     .as_bytes(),
    // )
    // .unwrap();

    // println!("Fen Strings {:?}", fen_strings);

    // c.close_window().await.unwrap();

    // for window_name in c.windows().await.unwrap() {
    //     println!("name: {}", window_name.0);

    //     c.switch_to_window(window_name)
    //         .await
    //         .expect("failed to switch");

    //     c.goto("https://chessgames.com/perl/chessgame?gid=1394457")
    //         .await
    //         .expect("failed to navigate");

    //     let play_button = c
    //         .find(Locator::Id("nextB"))
    //         .await
    //         .expect("could not find play button");

    //     for _ in 0..86 {
    //         let board = c
    //             .find(Locator::Css(".board-b72b1"))
    //             .await
    //             .expect("failed to find chess board")
    //             .find_all(Locator::Css("div[class^=row"))
    //             .await
    //             .expect("failed to find rows");

    //         generate_fen_string(board).await;

    //         let p = play_button.clone();

    //         p.click().await.expect("failed to click");
    //     }

    //     c.close_window().await.unwrap();

    //     c.new_window(false).await.unwrap();
    // }

    // c.switch_to_window(window)

    // c.close().await.expect("failed to close");

    exec!("sudo", "fuser", "-k", "4444/tcp");

    println!("Scraper, Ended!");
}
