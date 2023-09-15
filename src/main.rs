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
//pub mod bin;

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
//     //smp_render::run();
//     //smp::run();
// }

//#[path = "src/bin/smp.rs"]
//pub mod smp;
use futures_util::sink::SinkExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebsocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebsocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    // TODO: For a hint, see the description of the task below.
    ws_stream.send(Message::text("Hi Hello 안녕".into())).await?;
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        let msg = msg.as_text()?;
                        println!("From client {addr:?} {msg:?}");
                        bcast_tx.send(msg.into())?;
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    let tx = bcast_tx.clone();
    tokio::spawn(async move {
        smp::run_loop(tx);
    });

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}