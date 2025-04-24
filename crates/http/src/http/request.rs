use std::{collections::HashMap, io::prelude::*, io::BufReader, net::TcpStream};

use regex::Regex;

use super::{HttpError, HttpMethod, Protocol, Status};

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub protocol: Protocol,
    pub headers: HashMap<String, String>,
    pub body: String,
}

const HEADER_LIST: [&'static str; 18] = [
    "accept",
    "accept-encoding",
    "accept-language",
    "authorization",
    "cookie",
    "content-length",
    "content-type",
    "host",
    "if-match",
    "if-modified-since",
    "if-none-match",
    "if-range",
    "if-unmodified-since",
    "location",
    "pragma",
    "proxy-authorization",
    "ranges",
    "user-agent",
];

impl Request {
    pub fn parse(request_stream: &TcpStream) -> Result<Self, HttpError> {
        let request_buf = BufReader::new(request_stream);

        let error = HttpError {
            status: Status::BadRequest400("invalid http request format".to_string()),
        };

        let mut lines = vec![];

        for line in request_buf.lines() {
            match line {
                Ok(l) => {
                    if !l.is_empty() {
                        lines.push(l)
                    } else {
                        break;
                    }
                }
                Err(err) => {
                    dbg!(err);

                    return Err(error);
                }
            }
        }

        // handle EOF
        if lines.is_empty() {
            return Err(HttpError::new(Status::TCPError));
        }

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
            // TODO: check if it is http/2 or above
            _ => Err(error.clone()),
        }?;

        let mut headers: HashMap<String, String> = HashMap::new();

        let headers_iter = (&lines[1..]).iter().take_while(|line| **line != "\n");

        headers_iter.for_each(|line| {
            let splitted: Vec<&str> = line.split(" ").collect();

            if splitted.len() == 2 && splitted[0].ends_with(":") {
                let key = splitted[0].trim_end_matches(":").to_lowercase();

                if HEADER_LIST.contains(&key.as_str()) {
                    headers
                        .entry(key.to_string())
                        .or_insert(splitted[1].to_string());
                }
            }
        });

        let mut body_iter = (&mut lines[1..])
            .iter_mut()
            .skip_while(|line| **line != "\r\n");

        body_iter.next(); // skip the empty line

        let mut body = String::new();

        while let Some(line) = body_iter.next() {
            body.push_str(line);
        }

        let req = Self {
            method,
            path,
            protocol,
            headers,
            body,
        };

        req.validate()?;

        Ok(req)
    }

    pub fn validate(&self) -> Result<(), HttpError> {
        // case 1: asterisk form
        if self.path == "*" && self.method != HttpMethod::OPTIONS {
            return Err(HttpError::new(Status::BadRequest400(
                "request target '*' can only be used with OPTIONS method".to_string(),
            )));
        }

        // case 2: authority form
        let authority_syntax = Regex::new(r"(?<host>[a-z0-9\.]+):(?<port>\d+)").unwrap();

        if authority_syntax.is_match(&self.path) && self.method != HttpMethod::CONNECT {
            return Err(HttpError::new(Status::BadRequest400(
                "request target in the form <host>:<port> can only be used with CONNECT method"
                    .to_string(),
            )));
        }

        // case 3: for non-empty body
        if !self.body.is_empty()
            && !([HttpMethod::PATCH, HttpMethod::POST, HttpMethod::PUT].contains(&self.method))
        {
            return Err(HttpError::new(Status::BadRequest400(
                "only PATCH, POST, and PUT methods can send non-empty body".to_string(),
            )));
        }

        Ok(())
    }
}
