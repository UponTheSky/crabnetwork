use super::{HttpMethod, ProtocolVersion};

pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub protocol_version: ProtocolVersion,
}
