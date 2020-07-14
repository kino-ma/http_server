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
