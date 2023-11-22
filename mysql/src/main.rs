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
    // 可以考虑实现一个StringBuilder
    let mut user_input_buffer = String::new();
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
        user_input_buffer.push_str(buffer.as_str());

        let need_send_command = match user_input_buffer.find(';') {
            Some(index) => {
                let command = user_input_buffer.as_str()[0..=index].to_string();
                user_input_buffer = user_input_buffer.as_str()[(index + 1)..].to_string();
                Some(command)
            }
            None => None,
        };
        if let None = need_send_command {
            continue;
        }

        // 发送消息到服务器
        stream
            .write_all(need_send_command.unwrap().as_bytes())
            .await?;

        // 读取服务器的响应
        // 这里暂时假定服务器只发送<= 1024回来
        let mut response = vec![0; 1024];
        let n = stream.read(&mut response).await?;
        println!("Received: {}", String::from_utf8_lossy(&response[..n]));
    }

    Ok(())
}
