use sql_parser::parser::gen_parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = gen_parser();
    println!("simple mysqld for learn!");
    println!("--linYuan 2023.11.20--");
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("mysql running on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("connected {:?}", socket);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
