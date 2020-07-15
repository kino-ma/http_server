extern crate http_server;

use std::env;
use std::net::TcpListener;

use http_server::*;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        log_exit("not enough args");
    }

    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind port");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                log_exit(&error.to_string());
                continue;
            }
        };

        if let Err(error) = service(stream, &args[1]) {
            log_exit(&error.to_string());
        }
    }
}
