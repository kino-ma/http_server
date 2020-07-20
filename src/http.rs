use std::fmt;
use std::io::{self, Result, Error, ErrorKind};
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
pub struct Request<'a> {
    info: Info<'a>,
    headers: RequestHeaders<'a>,
    body: &'a str,
    length: usize,
}

#[derive(Debug, PartialEq)]
pub struct Info<'a> {
    method: &'a str,
    path: &'a str,
    protocol_minor_version: i32,
}

#[derive(Debug, PartialEq)]
pub struct RequestHeaders<'a> {
    headers: Vec<RequestHeader<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct RequestHeader<'a> {
    key: &'a str,
    value: &'a str,
}


impl Request<'_> {
    pub fn new<'a>(src: &'a str) -> Result<Request<'a>> {
        let (first, headers, body) = Self::split_content(src)?;

        let info = Info::new(first)?;

        let headers = Self::parse_headers(headers)?;

        Ok( Request {
            info,
            headers,
            body: body,
            length: body.len() as usize,
        })
    }

    fn split_content(src: &str) -> Result<(&str, Vec<&str>, &str)> {
        let splited: Vec<&str> = src.splitn(2, "\r\n\r\n").collect();

        println!("{:?}", splited[0]);

        let data = to_result(splited.get(0), &format!("failed to parse data: '{}'", src))?;
        let body = to_result(splited.get(1), &format!("failed to parse body: '{}'", src))?;

        let lines: Vec<&str> = data.split("\r\n").collect();
        let first = to_result(lines.get(0), "failed to parse request line")?;

        let mut iter = lines[1..].iter();
        let mut headers: Vec<&str> = Vec::new();

        while let Some(line) = iter.next() {
            if line == &"" {
                break;
            }

            headers.push(line);
        }

        return Ok((first, headers, body));
    }

    pub fn parse_headers(lines: Vec<&str>) -> Result<RequestHeaders> {
        let v: Vec<Vec<&str>> = lines
            .iter()
            .map( |line| line.split(": ").collect() )
            .collect();

        let mut headers = RequestHeaders::new();

        for l in &v {
            let name = to_result(l.get(0), "failed to parse header")?;
            let value = to_result(l.get(1), "failed to parse header")?;

            headers.insert(name, value);
        }

        Ok(headers)
    }

    pub fn resource(&self) -> &str {
        self.info.path
    }
}

impl Info<'_> {
    pub fn new(line: &str) -> Result<Info> {
        let mut words = line.split_whitespace();

        let method = to_result(words.next(), "failed to parse method")?;
        let path = to_result(words.next(), "failed to parse path")?;
        let protocol_minor_version = Self::parse_version(to_result(words.next(), "failed to parse version")?)?;

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
        let pv = to_result(pv, "failed to get version")?
            .to_digit(10);
        let pv = to_result(pv, "failed to parse version number");

        pv.and_then(|x| Ok(x as i32))
    }
}

impl<'a> RequestHeaders<'a> {
    pub fn new() -> RequestHeaders<'a> {
        let headers = Vec::new();
        RequestHeaders { headers }
    }

    pub fn insert(&mut self, key: &'a str, value: &'a str) {
        let header = RequestHeader { key, value };
        self.headers.push(header);
    }
}



/* Response */

use std::fs::{self, File};

#[derive(Debug, PartialEq)]
pub struct Response {
    version: &'static str,
    status: Status,
    headers: ResponseHeaders,
    body: String,
}

#[derive(Debug, PartialEq)]
pub struct Status {
    code: i32,
    text: String,
}

#[derive(Debug, PartialEq)]
pub struct ResponseHeaders {
    headers: Vec<ResponseHeader>,
}

#[derive(Debug, PartialEq)]
pub struct ResponseHeader {
    key: &'static str,
    value: String,
}

impl Response {
    pub fn new(resource: &str, docroot: &str) -> Result<Response> {
        let filepath = Self::make_path(docroot, resource);

        let body = fs::read_to_string(&filepath)?;

        let headers = Self::make_headers(&body);

        let status = Status {
            code: 200,
            text: "OK".to_string(),
        };

        Ok( Response {
            version: "HTTP/1.0",
            status,
            headers,
            body,
        })
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<String> {
        use std::io::Write;

        let text = self.show();
        stream.write_all(text.as_bytes());
        Ok(text)
    }

    fn make_headers(body: &String) -> ResponseHeaders {
        let mut headers: ResponseHeaders = ResponseHeaders::new();

        headers.insert("Content-Length", body.len().to_string());
        headers.insert("Content-Type", "text/html".to_string());
        headers.insert("Server", "LittleHTTP/1.0".to_string());
        headers.insert("Connection", "Close".to_string());

        headers
    }

    fn make_path(docroot: &str, filename: &str) -> String {
        let mut path = docroot.to_string();
        path.push_str(filename);
        path
    }

    pub fn show(&self) -> String {
        format!("{}", self)
    }
}

impl ResponseHeaders {
    pub fn new() -> ResponseHeaders {
        let headers = Vec::new();
        ResponseHeaders { headers }
    }

    pub fn insert(&mut self, key: &'static str, value: String) {
        let header = ResponseHeader { key, value };
        self.headers.push(header);
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{} {}\r\n{}\r\n\r\n{}",
                self.version,
                self.status,
                self.headers,
                self.body))
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.code, self.text)
    }
}

impl fmt::Display for ResponseHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = self.headers.iter().map(|header| format!("{}", header)).collect::<Vec<_>>().join("\r\n");
        write!(f, "{}", t)
    }
}

impl fmt::Display for ResponseHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
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
        let content = "GET /hoge.html HTTP/1.0\r\nAccept: */*\r\nConnection: Close\r\nUser-Agent: Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)\r\n\r\nhogehoge\r\n\r\nfugafuga\r\n";
        let request = Request::new(content).expect("failed to parse request(test)");

        let info = Info {
            method: "GET",
            path: "/hoge.html",
            protocol_minor_version: 0,
        };

        let mut headers = RequestHeaders::new();
        headers.insert("Accept", "*/*" );
        headers.insert("Connection", "Close" );
        headers.insert("User-Agent", "Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)" );

        let body = "hogehoge\r\n\r\nfugafuga\r\n";

        let expect = Request { info, headers, body, length: body.len() as usize };

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

        let mut expect = RequestHeaders::new();
        expect.insert("Accept", "*/*" );
        expect.insert("Connection", "Close" );
        expect.insert("User-Agent", "Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)" );

        assert_eq!(headers, expect);
    }


    #[test]
    pub fn should_create_file_response() {
        let info = Info {
            method: "GET",
            path: "/hoge.html",
            protocol_minor_version: 0,
        };

        let mut headers = RequestHeaders::new();
        headers.insert("Accept", "*/*" );
        headers.insert("Connection", "Close" );
        headers.insert("User-Agent", "Mozilla/4.0 (Compatible; MSIE 6.0; Windows NT 5.1;)" );

        let body = "hogehoge\r\n\r\nfugafuga\r\n";

        let request = Request { info, headers, body, length: body.len() as usize };

        let response = Response::new(request, "./pages").expect("failed to create response(test)");


        let status = Status { code: 200, text: "OK".to_string() };

        let body = "<h1>hogehoge</h1>\n".to_string();

        let mut headers = ResponseHeaders::new();
        headers.insert("Content-Length", body.len().to_string());
        headers.insert("Content-Type", "text/plain".to_string());
        headers.insert("Server", "LittleHTTP/1.0".to_string());
        headers.insert("Connection", "Close".to_string());

        let expect = Response { status, headers, body };

        assert_eq!(response, expect);
    }
}
