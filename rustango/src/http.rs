use httparse::{Header, Request, Status};
use std::{io::prelude::*, net::TcpStream};

pub struct Response {
    pub data: String,
    pub status_code: u16,
    pub version: &'static str,
}

pub enum RequestErrors {
    ParsingError,
}

pub fn parse_request<'a>(
    buf: &'a mut [u8],
    headers: &'a mut [Header<'a>],
    stream: &'a mut TcpStream,
) -> Result<Request<'a, 'a>, RequestErrors> {
    stream.read(buf).unwrap();
    let mut req = Request::new(headers);
    match req.parse(buf) {
        Ok(Status::Complete(_)) => Ok(req),
        _ => Err(RequestErrors::ParsingError),
    }
}

impl Response {
    pub fn to_string(&self) -> String {
        let data = &self.data;
        let status_code_str = self.status_code_to_string();
        let version_str = self.version_to_string();
        let length = data.len();
        let res =
            format!("{version_str} {status_code_str}\r\nContent-Length: {length}\r\n\r\n{data}");
        res
    }

    fn version_to_string(&self) -> String {
        match self.version {
            "1.1" => "HTTP/1.1".to_string(),
            _ => "HTTP/1.0".to_string(), // fallback
        }
    }

    fn status_code_to_string(&self) -> String {
        match self.status_code {
            200 => "200 OK".to_string(),
            400 => "400 Bad request".to_string(),
            404 => "404 NOT FOUND".to_string(),
            405 => "405 Method Not Allowed".to_string(),
            i => format!("{i} UNKOWN STATUS CODE"),
        }
    }
}
