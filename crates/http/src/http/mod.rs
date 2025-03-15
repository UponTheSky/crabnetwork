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

#[derive(Debug)]
pub enum Status {
    OK200(String),
}
