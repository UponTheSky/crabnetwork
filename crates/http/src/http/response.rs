use std::collections::HashMap;

use super::{Protocol, Status};

#[derive(Debug)]
pub struct Response {
    pub protocol: Protocol,
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

const HEADER_LIST: [&'static str; 6] = [
    "server",
    "date",
    "cache-control",
    "content-type",
    "content-length",
    "etag",
    "last-modified",
    "transfer-encoding",
];

impl Response {
    pub fn new(
        protocol: Protocol,
        status: Status,
        headers: HashMap<String, String>,
        body: Option<Vec<u8>>,
    ) -> Self {
        Self {
            protocol,
            status,
            headers,
            body,
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let (status_code, message) = match self.status {
            Status::OK200(message) => (200, message),
            _ => (400, "bad request".into()),
        };

        let headers = self
            .headers
            .into_iter()
            .fold(String::new(), |acc, (key, value)| {
                acc.push_str(format!("{}: {}\n").as_str());
            });

        let mut encode_str = format!("{} {} {}\n", self.protocol, status_code, message, headers);

        if let Some(body) = self.body {
            encode_str.push_str(body);
        }

        encode_str.into_bytes()
    }
}
