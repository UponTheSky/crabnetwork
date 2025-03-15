use std::fmt::Display;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::{self, TcpListener};
use std::os::fd::AsFd;
use std::thread;

use crate::http::request::Request;
use crate::http::{request, response, HttpMethod, ProtocolVersion, Status};

pub struct Config {
    host: String,
    port: u32,
}

impl Config {
    pub fn new(host: String, port: u32) -> Self {
        Self { host, port }
    }
}

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&self) {
        let listener =
            TcpListener::bind(format!("{}:{}", self.config.host, self.config.port)).unwrap(); // create socket, bind to the address, and listen

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
}

fn handle_tcp_stream(mut accepted: TcpStream) {
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

                let response = response::Response {
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
