use std::io::Result;

pub struct Request<'a> {
    protocol_minor_version: i32,
    method: &'a str,
    path: &'a str,
    headers: Vec<Header<'a>>,
    body: &'a str,
    length: u64,
}

pub struct Header<'a> {
    name: &'a str,
    value: &'a str,
}

impl Request<'_> {
    pub fn new<'a>(src: &'a str) -> Result<Request<'a>> {
        Ok(Request {
            protocol_minor_version: 0,
            method: src,
            path: src,
            headers: Vec::new(),
            body: src,
            length: 1,
        })
    }
}
