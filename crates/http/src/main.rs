use std::io::{Read, Write};
use std::net::TcpStream;
use std::os::fd::AsFd;
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

    todo!("use regex to parse the get request");
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
