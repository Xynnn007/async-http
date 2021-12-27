use std::error::Error;
use async_http::handle_connection;
use async_std::net::TcpListener;
use async_std::task::spawn;
use futures::StreamExt;
use log::{LevelFilter, info};

const ADDRESS : &str = "0.0.0.0:7878";

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