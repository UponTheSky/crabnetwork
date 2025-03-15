use super::{HttpMethod, ProtocolVersion};

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub protocol_version: ProtocolVersion,
}

impl Request {
    pub fn parse(message_byte: Vec<u8>) -> Result<Self, String> {
        let message = String::from_utf8(message_byte).unwrap();
        let lines: Vec<&str> = message.lines().collect();
        let error = Err("invalid http request format".into());

        if lines.len() < 3 {
            return error;
        }

        // For strictness, we need to parse the given http request text according to
        // a list of rules. But here we're trying to simplify that process
        // and focusing on learning the materials
        let request_line: Vec<&str> = lines[0].split(" ").collect();

        if request_line.len() != 3 {
            return error;
        }

        let method = match request_line[0] {
            "GET" => Some(HttpMethod::GET),
            _ => None,
        }
        .unwrap();

        let path = request_line[1];
        let protocol_version = match request_line[2] {
            "HTTP/1.1" => Some(ProtocolVersion::HTTP11),
            _ => None,
        }
        .unwrap();

        Ok(Self {
            method,
            path: path.into(),
            protocol_version,
        })
    }
}
