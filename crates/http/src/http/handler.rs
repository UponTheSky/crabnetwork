use super::request::Request;
use super::response::Response;
use super::Status;

pub struct HttpHandler {}

impl HttpHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_request(&self, request_byte: Vec<u8>) -> Result<Response, String> {
        match Request::parse(request_byte) {
            Ok(req) => {
                // todo: add logic for various requests
                dbg!(&req);

                Ok(Response::new(
                    req.protocol_version,
                    Status::OK200("Ok".into()),
                ))
            }
            Err(error) => Err(error),
        }
    }
}
