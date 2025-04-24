use std::fmt::Display;

pub(crate) mod handler;
pub(crate) mod request;
pub(crate) mod response;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Debug)]
pub enum Protocol {
    HTTP11,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::HTTP11 => f.write_str("Http/1.1"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    OK200(String),
    NoContent204,
    PartialContent206,
    MultipleChoices300,
    MovedPermanently301,
    Found302,
    SeeOther303,
    NotModified304,
    TemporaryRedirect307,
    PermanentRedirect308,
    BadRequest400(String),
    UnAuthorized401(String),
    Forbidden403(String),
    NotFound404(String),
    ProxyAuthenticationRequired407(String),
    PreconditionFailed412,
    RequestedRangeNotSatisfiable416,
    TCPError,
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
