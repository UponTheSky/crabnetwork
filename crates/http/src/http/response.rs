use super::{ProtocolVersion, Status};

#[derive(Debug)]
pub struct Response {
    pub protocol_version: ProtocolVersion,
    pub status: Status,
}

impl Response {
    pub fn new(protocol_version: ProtocolVersion, status: Status) -> Self {
        Self {
            protocol_version,
            status,
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let (status_code, message) = match self.status {
            Status::OK200(message) => (200, message),
            _ => (400, "bad request".into()),
        };

        let mock_body = "This is a mock response to your GET request!";

        let encode_str = format!(
            "{} {} {} \n\n {}",
            self.protocol_version, status_code, message, mock_body
        );

        encode_str.into_bytes()
    }
}
