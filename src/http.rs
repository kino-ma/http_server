use std::io::{self, Result, Error, ErrorKind};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Request<'a> {
    info: Info<'a>,
    headers: HashMap<&'a str, &'a str>,
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
        let (first, headers, body) = Self::split_content(src)?;

        let info = Info::new(first)?;

        Ok( Request {
            info,
            headers: HashMap::new(),
            body: body,
            length: body.len() as u64,
        })
    }

    fn split_content(src: &str) -> Result<(&str, Vec<&str>, &str)> {
        let splited: Vec<&str> = src.split("\r\n\r\n").collect();

        let data = to_result(splited.get(0), "failed to parse")?;
        let body = to_result(splited.get(1), "failed to parse")?;

        let lines: Vec<&str> = data.split("\r\n").collect();
        let first = to_result(lines.get(0), "failed to parse")?;

        let mut iter = lines[1..].iter();
        let mut headers: Vec<&str> = Vec::new();

        loop {
            let line = to_result(iter.next(), "failed to parse")?;

            if line == &"" {
                break;
            }

            headers.push(line);
        }

        return Ok((first, headers, body));
    }

    pub fn parse_headers(lines: Vec<&str>) -> Result<HashMap<&str, &str>> {
        let v: Vec<Vec<&str>> = lines
            .iter()
            .map( |line| line.split(": ").collect() )
            .collect();

        let mut headers: HashMap<&str, &str> = HashMap::new();
        for l in &v {
            let name = to_result(l.get(0), "failed to parse header")?;
            let value = to_result(l.get(1), "failed to parse header")?;

            headers.insert(name, value);
        }

        Ok(headers)
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

    #[test]
    pub fn should_parse_request() {
        let content = "GET /path/to/resource HTTP/1.0\r\nAccept: */*\r\nConnection: Close\r\nUser-Agent: Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)\r\n\r\nhogehoge\r\nfugafuga\r\n";
        let request = Request::new(content);

        let info = Info {
            method: "GET",
            path: "/path/to/resource",
            protocol_minor_version: 0,
        };

        let mut headers = Vec::new();
        headers.push(Header { name: "Accept", value: "*/*" });
        headers.push(Header { name: "Connection", value: "Close" });
        headers.push(Header { name: "User-Agent", value: "Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)" });

        let body = "hogehgoe\r\nfugafuga";

        let expect = Request { info, headers, body };

        assert_eq!(request, expect);
    }

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

    #[test]
    pub fn should_parse_headers() {
        let content = "Accept: */*\r\nConnection: Close\r\nUser-Agent: Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)\r\n";

        let lines = content.lines().collect();
        let headers = Request::parse_headers(lines).expect("failed to parse");

        let mut expect = HashMap::new();
        expect.insert("Accept", "*/*" );
        expect.insert("Connection", "Close" );
        expect.insert("User-Agent", "Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)" );

        assert_eq!(headers, expect);
    }
}
