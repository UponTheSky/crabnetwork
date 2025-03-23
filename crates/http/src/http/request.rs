use std::collections::HashMap;

use super::{HttpError, HttpMethod, Protocol, Status};

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub protocol: Protocol,
    pub headers: HashMap<String, String>,
    pub body: String,
}

const HEADER_LIST: [&'static str; 5] = [
    "Host",
    "User-Agent",
    "Accept",
    "Content-Type",
    "Content-Length",
];

impl Request {
    pub fn parse(message_byte: Vec<u8>) -> Result<Self, HttpError> {
        let mut message = String::new();

        match String::from_utf8(message_byte) {
            Ok(msg) => {
                message = msg;
            }
            Err(error) => {
                return Err(HttpError::new(Status::BadRequest400(format!(
                    "{}: {}",
                    "invalid http request form", error
                ))));
            }
        }

        let mut lines: Vec<&str> = message.lines().collect();

        let error = HttpError {
            status: Status::BadRequest400("invalid http request format".to_string()),
        };

        // For strictness, we need to parse the given http request text according to
        // a list of rules. But here we're trying to simplify that process
        // and focusing on learning the materials

        let request_line: Vec<&str> = lines[0].split(" ").collect();

        if request_line.len() < 2 {
            return Err(error);
        }

        let method = match request_line[0] {
            "GET" => Ok(HttpMethod::GET),
            _ => Err(error.clone()),
        }?;

        let path = request_line[1].to_string();
        let protocol = match request_line.get(2) {
            Some(&"HTTP/1.1") => Ok(Protocol::HTTP11),
            _ => Err(error.clone()),
        }?;

        let mut headers: HashMap<String, String> = HashMap::new();

        let headers_iter = (&lines[1..]).iter().take_while(|line| **line != "\n");

        headers_iter.for_each(|line| {
            let splitted: Vec<&str> = line.split(" ").collect();

            if splitted.len() == 2 && splitted[0].ends_with(":") {
                let key = splitted[0].trim_end_matches(":");

                if HEADER_LIST.contains(&key) {
                    headers
                        .entry(key.to_string())
                        .or_insert(splitted[1].to_string());
                }
            }
        });

        let mut body_iter = (&mut lines[1..])
            .iter_mut()
            .skip_while(|line| **line != "\n");

        body_iter.next(); // skip the empty line

        let mut body = String::new();

        while let Some(line) = body_iter.next() {
            body.push_str(*line);
        }

        Ok(Self {
            method,
            path,
            protocol,
            headers,
            body,
        })
    }
}
