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
        match req {
            Ok(req_ok) => Response::new(
                req_ok.protocol,
                Status::OK200("Ok".into()),
                HashMap::new(),
                None,
            ),
            Err(req_error) => {
                // todo!("put protocol versions and other necessary information into HttpErrror");
                Response::new(
                    crate::http::Protocol::HTTP11,
                    req_error.status,
                    HashMap::new(),
                    None,
                )
            }
        }
    }
}
