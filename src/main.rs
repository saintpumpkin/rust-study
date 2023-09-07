#![allow(dead_code)]
// include!("week1.rs");
// include!("week2.rs");
//include!("week3.rs");
pub mod week3;
pub mod week4;
pub mod week5;
pub mod week6;
pub mod week7;
pub mod week8;
pub mod smp_input;
pub mod smp_render;
pub mod smp;

use tokio::time;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpSocket};
use tokio::sync::mpsc::{self, Receiver};
use anyhow::Result;
use futures::future;
use reqwest;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use futures::future::join_all;
use std::time::Instant;
use tokio::sync::{oneshot};
use tokio::task::spawn;
use async_trait::async_trait;
use std::io::{ErrorKind};
use tokio::io::{DuplexStream};


// fn main() {
//     println!("===========================================================");
//     println!("========================== week2 ==========================");
//     println!("===========================================================");
//     //week2();
//     println!();
//     println!();
//     println!();
//     println!("===========================================================");
//     println!("========================== week3 ==========================");
//     println!("===========================================================");
//     week3::week3();
//     println!();
//     println!();
//     println!();
//     println!("===========================================================");
//     println!("========================== week4 ==========================");
//     println!("===========================================================");
//     week4::week4();
//     println!();
//     println!();
//     println!();
//     println!("===========================================================");
//     println!("========================== week5 ==========================");
//     println!("===========================================================");
//     //week5::week5();
//     println!();
//     println!();
//     println!();
//     println!("===========================================================");
//     println!("========================== week6 ==========================");
//     println!("===========================================================");
//     //week6::week6();
//     println!();
//     println!();
//     println!();
//     println!("===========================================================");
//     println!("========================== week7 ==========================");
//     println!("===========================================================");
//     week7::week7();
//     println!();
//     println!();
//     println!();
//     println!("===========================================================");
//     println!("========================== week8 ==========================");
//     println!("===========================================================");
//     week8::week8();
//     println!();
//     println!();
//     println!();
//     println!("===========================================================");
//     println!("=========================== smp ===========================");
//     println!("===========================================================");
//     //smp_input::run();
//     smp_render::run();
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (client, server) = tokio::io::duplex(5);
    let handle = tokio::spawn(week8::slow_copy("hi\nthere\n".to_owned(), client));

    let mut lines = week8::LinesReader::new(server);
    let mut interval = tokio::time::interval(Duration::from_millis(60));
    loop {
        tokio::select! {
            _ = interval.tick() => println!("tick!"),
            line = lines.next() => if let Some(l) = line? {
                print!("{}", l)
            } else {
                break
            },
        }
    }
    handle.await.unwrap()?;
    Ok(())
}