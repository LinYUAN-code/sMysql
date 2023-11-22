use std::{str::from_utf8, sync::Arc};

use chumsky::prelude::*;

use sql_parser::parser::gen_parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

// 实现Sync trait
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("simple mysqld for learn!");
    println!("--linYuan 2023.11.20--");
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("mysql running on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("connected {:?}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut command_buffer = String::new();

            loop {
                // 以分号作为命令分隔符
                match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => break,
                    Ok(n) => command_buffer.push_str(from_utf8(&buf[0..n]).unwrap()),
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        break;
                    }
                };
                while let Some(index) = command_buffer.find(';') {
                    let command = command_buffer.as_str()[0..=index].to_string();
                    command_buffer = command_buffer.as_str()[(index + 1)..].to_string();

                    println!("{:?}", gen_parser().parse(command));
                    if let Err(e) = socket.write_all("detect command".as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            }
            println!("leave {:?}", socket.peer_addr().unwrap());
        });
    }
}
