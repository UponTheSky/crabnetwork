use std::collections::HashMap;
use std::io;
use std::net::TcpStream;

use crate::http::response::{CacheOptions, Cookies};

use super::request::Request;
use super::response::Response;
use super::Status;

pub struct HttpHandler {}

impl HttpHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_request(&self, request_stream: &TcpStream) -> std::io::Result<Response> {
        // todo: add logic for various requests
        let request = Request::parse(request_stream);
        dbg!(&request);

        // todo: add more info for responses
        let mut cookie_values = HashMap::new();
        cookie_values.insert(
            "best-movie".to_string(),
            "how to train your dragon".to_string(),
        );
        cookie_values.insert(
            "the-second-best-movie".to_string(),
            "hangover trilogy".to_string(),
        );

        let mut common_headers = HashMap::new();
        common_headers.insert("Content-Type".to_string(), "text/html".to_string());

        match request {
            Ok(req_ok) => Ok(Response::new(
                req_ok.protocol,
                Status::OK200("Ok".into()),
                common_headers,
                Some(Cookies::new(
                    cookie_values,
                    None,
                    true,
                    true,
                    None,
                    Some("/".to_string()),
                    None,
                )),
                Some(CacheOptions::new_default()),
                Some(
                    "\r\n
<html>
    <head>
        <title>
            heyya
        </title>
    </head>
    <body>
        hey this is my own http server written in Rust!
    </body>
</html>
\r\n
                "
                    .as_bytes()
                    .to_owned(),
                ),
            )),
            Err(req_error) => {
                match req_error.status {
                    Status::TCPError => Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "tcp stream is EOF",
                    )),
                    _ => Ok(Response::new(
                        crate::http::Protocol::HTTP11,
                        req_error.status,
                        HashMap::new(),
                        None,
                        None,
                        None,
                    )),
                }

                // todo!("put protocol versions and other necessary information into HttpErrror");
            }
        }
    }
}
