use std::io::{self, Result, Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub struct Request<'a> {
    info: Info<'a>,
    headers: Vec<Header<'a>>,
    body: &'a str,
    length: u64,
}

#[derive(Debug, PartialEq)]
pub struct Info<'a> {
    method: &'a str,
    path: &'a str,
    protocol_minor_version: i32,
}

#[derive(Debug, PartialEq)]
pub struct Header<'a> {
    name: &'a str,
    value: &'a str,
}

impl Request<'_> {
    pub fn new<'a>(src: &'a str) -> Result<Request<'a>> {
        let mut lines = src.lines();

        let first = to_result(lines.next(), "not enough info")?;

        let info = Info::new(first)?;

        Ok( Request {
            info,
            headers: Vec::new(),
            body: src,
            length: 1,
        })
    }
}

impl Info<'_> {
    pub fn new<'a>(line: &'a str) -> Result<Info<'a>> {
        let mut words = line.split_whitespace();

        let method = to_result(words.next(), "failed to parse")?;
        let path = to_result(words.next(), "failed to parse")?;
        let protocol_minor_version = Self::parse_version(to_result(words.next(), "failed to parse")?)?;

        Ok( Info {
            method,
            path,
            protocol_minor_version,
        })
    }

    fn parse_version(src: &str) -> Result<i32> {
        let pv = src
            .chars()
            .last();
        let pv = to_result(pv, "failed to parse")?
            .to_digit(10);
        let pv = to_result(pv, "failed to parse");

        pv.and_then(|x| Ok(x as i32))
    }
}

fn to_result<T>(value: Option<T>, msg: &str) -> Result<T> {
    value.ok_or(Error::new(ErrorKind::Other, msg))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /*
    pub fn should_parse_request() {
        let content = "GET /path/to/resource HTTP/1.0\r\nAccept: *_/_*\r\nAccept-Language: ja\r\nAccept-Encoding: gzip, deflate\r\nUser-Agent: Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)\r\nConnection: Close\r\n";
        let request = Request::new(content);

        let info = Info {
            method: "GET",
            path: "/path/to/resource",
            protocol_minor_version: 0,
        };

        let headers = Vec::new()
        headers: Vec<Header<'a>>,
        body: &'a str,
        length: u64,
    }
    */

    #[test]
    pub fn should_parse_info() {
        let line = "GET /path/to/resource HTTP/1.0";
        let info = Info::new(line).expect("failed to parse");
        let expect = Info {
            method: "GET",
            path: "/path/to/resource",
            protocol_minor_version: 0,
        };

        assert_eq!(info, expect);
    }

    pub fn should_parse_headers() {
        let content = "Accept: */*\r\nConnection: Close\r\nUser-Agent: Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)\r\n";
        let headers = content.lines().map(|line| Header::new(line).expect("failed to parse"));

        let mut expect = Vec::new();
        expect.push(Header { name: "Accept", value: "*/*" });
        expect.push(Header { name: "Connection", value: "Close" });
        expect.push(Header { name: "Accept", value: "*/*" });
        expect.push(Header { name: "User-Agent", value: "Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;" });;
    }
}
