pub mod http;

use std::io::prelude::*;

pub fn log_exit(text: &str) {
    eprintln!("error: {}", text);

    std::process::exit(1);
}

pub fn service<I: Read, O: Write>(mut input: I, mut output: O, docdir: &str) -> std::io::Result<()> {
    let mut buf = String::new();

    input.read_to_string(&mut buf)?;

    let mut req = http::Request::new(&buf)?;

    output.write_all(&buf.into_bytes())
}
