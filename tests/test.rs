use std::task::Poll;

use async_http::handle_connection;
use async_std::{io::{Read, Write, WriteExt}, fs};

#[derive(Default)]
struct MockTcpStream {
    buf: Vec<u8>,
}

impl Read for MockTcpStream {
    fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &mut [u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
        let res = self.buf.len();
        buf[..res].clone_from_slice(&self.buf);
        self.buf.clear();
        Poll::Ready(Ok(res))
    }
}

impl Write for MockTcpStream {
    fn poll_write(
            mut self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
        let res = buf.len();
        self.buf.append(&mut buf.to_vec());
        Poll::Ready(Ok(res))
    }

    fn poll_flush(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[async_std::test]
async fn stream_get() {
    let mut stream = MockTcpStream::default();
    let request = b"GET / HTTP/1.1\r\n";
    stream.write(request).await.unwrap();
    handle_connection(&mut stream).await;
    let status_code = "HTTP/1.1 200 OK\r\n\r\n";
    let file_content = fs::read_to_string("html/hello.html").await.unwrap();
    let response = format!("{}{}", status_code, file_content);
    assert!(stream.buf.starts_with(response.as_bytes()));
}

#[async_std::test]
async fn stream_sleep() {
    let mut stream = MockTcpStream::default();
    let request = b"GET /sleep HTTP/1.1\r\n";
    stream.write(request).await.unwrap();
    handle_connection(&mut stream).await;
    let status_code = "HTTP/1.1 200 OK\r\n\r\n";
    let file_content = fs::read_to_string("html/hello.html").await.unwrap();
    let response = format!("{}{}", status_code, file_content);
    assert!(stream.buf.starts_with(response.as_bytes()));
}

#[async_std::test]
async fn stream_error() {
    let mut stream = MockTcpStream::default();
    let request = b"GET /mickey_mouse_house HTTP/1.1\r\n";
    stream.write(request).await.unwrap();
    handle_connection(&mut stream).await;
    let status_code = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let file_content = fs::read_to_string("html/error.html").await.unwrap();
    let response = format!("{}{}", status_code, file_content);
    assert!(stream.buf.starts_with(response.as_bytes()));
}