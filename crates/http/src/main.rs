use std::io::{Read, Write};
use std::net::TcpStream;
use std::os::fd::AsFd;
use std::thread;
use std::{
    net::{self, TcpListener},
    time::Duration,
};

use regex::Regex;

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

                dbg!(request.endpoint);
                dbg!(request.request_type);

                accepted.write(&buf).unwrap_or_else(|_| {
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
enum RequestType {
    GET,
}

struct Request {
    request_type: RequestType,
    endpoint: String,
}

fn parse_http_message(message: String) -> Result<Request, String> {
    let lines: Vec<&str> = message.lines().collect();
    let error = Err("invalid http request format".into());

    if lines.len() < 3 {
        return error;
    }

    let type_check_regex = Regex::new(r"(GET) (\/|(\/(?=[a-z])[a-z0-9]+)+\/?(\?(?=[a-z])[a-z0-9]+=\w+(&(?=[a-z])[a-z0-9]+=\w+)*)?) HTTP\/1\.1").unwrap();

    // might required later
    // let host_regex = Regex::new(r"Host: ((?=[a-z])[a-z0-9]+\.((?=[a-z])[a-z0-9]+\.)*[a-z]+)");
    // let language_regex = Regex::new(r"Accept-Language: ([a-z]{2})");

    match type_check_regex.captures(lines[0]) {
        Some(captured) => {
            let (_, matched): (&str, [&str; 2]) = captured.extract();

            if matched.len() != 2 {
                return error;
            }

            let request_type = match matched[0] {
                "GET" => Some(RequestType::GET),
                _ => None,
            };

            if request_type.is_none() {
                return error;
            }

            Ok(Request {
                request_type: request_type.unwrap(),
                endpoint: String::from(matched[1]),
            })
        }
        None => error,
    }
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
