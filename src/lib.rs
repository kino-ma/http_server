pub mod http;

use std::io::{Read};
use std::net::TcpStream;

pub fn log_exit(text: &str) {
    eprintln!("error: {}", text);

    std::process::exit(1);
}

pub fn service(mut stream: TcpStream, docdir: &str) -> std::io::Result<()> {
    let mut buf = [0; 2048];
    let mut content = String::new();

    loop {
        let nbytes = stream.read(&mut buf)?;

        let buf_str: &str = match std::str::from_utf8(&buf[..]) {
            Ok(s) => s,
            Err(error) => {
                let kind = std::io::ErrorKind::Other;
                return Err(std::io::Error::new(kind, error));
            }

        };

        content.push_str(&buf_str[0..nbytes]);

        if nbytes < 2048 {
            break;
        }
    }

    let request = http::Request::new(&content)?;

    let response = match http::Response::new(request.resource(), docdir) {
        Ok(r) => r,
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                http::Response::new("/404.html", docdir)?
            } else {
                http::Response::new("/500.html", docdir)?
            }
        }
    };

    response.send(&mut stream)?;

    Ok(())
}
