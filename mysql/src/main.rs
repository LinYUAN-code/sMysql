use std::env;
use std::io::{self, BufRead, Write};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("simple mysql for learn!");
    println!("--linYuan 2023.11.20--");
    let addr = "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr).await?;
    println!("Connected to {}", addr);

    // 获取标准输入流
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let line_ending = match env::consts::OS {
        "windows" => "\r\n",
        _ => "\n",
    };

    let ending = "q".to_string() + line_ending;

    loop {
        // 读取用户输入
        let mut buffer = String::new();
        print!(">");
        io::stdout().flush()?;
        reader.read_line(&mut buffer)?;

        // 移除换行符

        if buffer == ending {
            break;
        }

        // 发送消息到服务器
        stream.write_all(buffer.as_bytes()).await?;

        // 读取服务器的响应
        let mut response = vec![0; 1024];
        let n = stream.read(&mut response).await?;
        println!("Received: {}", String::from_utf8_lossy(&response[..n]));
    }

    Ok(())
}
