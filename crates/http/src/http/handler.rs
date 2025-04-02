use std::collections::HashMap;

use crate::http::response::{CacheOptions, Cookies};

use super::request::{self, Request};
use super::response::Response;
use super::{HttpError, Status};

pub struct HttpHandler {}

impl HttpHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_request(&self, request_byte: Vec<u8>) -> Response {
        let req = Request::parse(request_byte);
        // todo: add logic for various requests
        dbg!(&req);

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
        match req {
            Ok(req_ok) => Response::new(
                req_ok.protocol,
                Status::OK200("Ok".into()),
                HashMap::new(),
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
                None,
            ),
            Err(req_error) => {
                // todo!("put protocol versions and other necessary information into HttpErrror");
                Response::new(
                    crate::http::Protocol::HTTP11,
                    req_error.status,
                    HashMap::new(),
                    None,
                    None,
                    None,
                )
            }
        }
    }
}
