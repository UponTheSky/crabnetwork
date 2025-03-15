use std::fmt::Display;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::os::fd::AsFd;
use std::str::Bytes;
use std::thread;
use std::{
    net::{self, TcpListener},
    time::Duration,
};

fn handle_tcp_stream(mut accepted: TcpStream) {
    accepted
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap_or_else(|_| {
            panic!(
                "failed to set read timeout on the accept socket {:?}",
                accepted.as_fd()
            )
        });

    accepted
        .set_write_timeout(Some(Duration::from_secs(10)))
        .unwrap_or_else(|_| {
            panic!(
                "failed to set write timeout on the accept socket {:?}",
                accepted.as_fd()
            )
        });

    let mut buf = vec![0; 1024];

    match accepted.read(&mut buf) {
        Ok(len) => {
            if len == 0 {
                // EOF
                accepted.shutdown(net::Shutdown::Both).unwrap_or_else(|_| {
                    panic!("failed to shutdown the socket {:?}", accepted.as_fd())
                });
            } else {
                let recv_message =
                    String::from_utf8(buf.clone()).expect("failed to convert byte to string");
                println!("{}", &recv_message);

                let request = parse_http_message(recv_message).unwrap();

                let response = Response {
                    protocol_version: request.protocol_version,
                    status_code: Status::OK200(String::from("OK")),
                };

                dbg!(&response);

                accepted.write(&response.encode()).unwrap_or_else(|_| {
                    panic!(
                        "failed to send the message from the socket {:?}",
                        accepted.as_fd()
                    )
                });
            }
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}

#[derive(Debug)]
enum HttpMethod {
    GET,
}

#[derive(Debug)]
enum ProtocolVersion {
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
enum Status {
    OK200(String),
}

struct Request {
    method: HttpMethod,
    path: String,
    protocol_version: ProtocolVersion,
}

#[derive(Debug)]
struct Response {
    protocol_version: ProtocolVersion,
    status_code: Status,
}

impl Response {
    fn encode(self) -> Vec<u8> {
        let (status_code, message) = match self.status_code {
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

fn parse_http_message(message: String) -> Result<Request, String> {
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

    let request_type = match request_line[0] {
        "GET" => Some(HttpMethod::GET),
        _ => None,
    }
    .unwrap();

    let endpoint = request_line[1];
    let version = match request_line[2] {
        "HTTP/1.1" => Some(ProtocolVersion::HTTP11),
        _ => None,
    }
    .unwrap();

    Ok(Request {
        method: request_type,
        path: endpoint.into(),
        protocol_version: version,
    })
}

fn main() {
    let addr = "localhost:8080";
    let listener = TcpListener::bind(addr).unwrap(); // create socket, bind to the address, and listen

    // accept loop
    for stream in listener.incoming() {
        match stream {
            Ok(accepted) => {
                // make a multi-thread for the accepted stream(socket)
                thread::spawn(move || {
                    handle_tcp_stream(accepted);
                });
            }
            Err(error) => {
                println!("error while listening tcp requests: {error}");
            }
        };
    }
}
