pub mod http;

use std::io::prelude::*;
use std::net::TcpStream;

pub fn log_exit(text: &str) {
    eprintln!("error: {}", text);

    std::process::exit(1);
}

pub fn service(mut stream: TcpStream, docdir: &str) -> std::io::Result<()> {
    let mut buf = String::new();

    stream.read_to_string(&mut buf)?;

    let mut request = http::Request::new(&buf)?;

    let mut response = http::Response::new(request)?;

    response.send(&mut stream);
}
