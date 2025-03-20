use std::fmt::Display;

pub(crate) mod handler;
pub(crate) mod request;
pub(crate) mod response;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
}

#[derive(Debug)]
pub enum ProtocolVersion {
    HTTP11,
}

impl Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolVersion::HTTP11 => f.write_str("Http/1.1"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    OK200(String),
    BadRequest400(String),
}

#[derive(Debug, Clone)]
pub struct HttpError {
    status: Status,
}

impl HttpError {
    pub fn new(status: Status) -> Self {
        Self { status }
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (main_message, details) = match &self.status {
            Status::BadRequest400(details) => ("bad request", details),
            _ => ("internal server error", &"unidentified problem".to_string()),
        };
        let _ = f.write_str(&format!("{}; details: {}", main_message, details));

        Ok(())
    }
}
