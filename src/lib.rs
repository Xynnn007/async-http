use std::time::Duration;

use async_std::fs;
use async_std::io::{Read, Write, WriteExt};
use futures::AsyncReadExt;

pub async fn handle_connection(mut stream: impl Read + Write + Unpin) {
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
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/error.html")
    };
    let contents = fs::read_to_string(filename).await.unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}