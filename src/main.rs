use std::{fs, time::Duration};
use std::error::Error;
use async_std::io::WriteExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::task::spawn;
use futures::{AsyncReadExt, StreamExt};
use log::{LevelFilter, info};

const ADDRESS : &str = "0.0.0.0:7878";

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else if buffer.starts_with(sleep) {
        async_std::task::sleep(Duration::from_secs(3)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>>{
    env_logger::builder().filter_level(LevelFilter::Info).init();
    
    let listener = TcpListener::bind(ADDRESS).await?;
    info!("Start listening {}...", ADDRESS);
    listener
        .incoming()
        .for_each_concurrent(100, |stream|async move {
            let stream = stream.unwrap();
            info!("Got new connection from {}", stream.peer_addr().unwrap().to_string());
            spawn(handle_connection(stream));
    })
    .await;
    
    Ok(())
}