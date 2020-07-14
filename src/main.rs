extern crate http_server;

use std::env;

use http_server::*;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        log_exit("not enough args");
    }

    if let Err(error) = service(std::io::stdin(), std::io::stdout(), &args[1]) {
        log_exit(&error.to_string());
    }
}
