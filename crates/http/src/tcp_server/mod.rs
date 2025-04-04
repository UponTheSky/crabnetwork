use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::{self, TcpListener};
use std::os::fd::AsFd;
use std::sync::Arc;
use std::thread;

use crate::http::handler::HttpHandler;

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
    http_handler: Arc<HttpHandler>,
}

impl Server {
    pub fn new(config: Config, http_handler: HttpHandler) -> Self {
        Self {
            config,
            http_handler: Arc::new(http_handler),
        }
    }

    pub fn run(&self) {
        let listener =
            TcpListener::bind(format!("{}:{}", self.config.host, self.config.port)).unwrap(); // create socket, bind to the address, and listen

        // accept loop
        for stream in listener.incoming() {
            let http_handler = Arc::clone(&self.http_handler);

            match stream {
                Ok(accepted) => {
                    // make a multi-thread for the accepted stream(socket)
                    thread::spawn(move || {
                        Self::handle_tcp_stream(accepted, http_handler);
                    });
                }
                Err(error) => {
                    println!("error while listening tcp requests: {error}");
                }
            };
        }
    }

    fn handle_tcp_stream(mut accepted: TcpStream, http_handler: Arc<HttpHandler>) {
        let mut buf = vec![0; 1024];

        match accepted.read(&mut buf) {
            Ok(len) => {
                if len == 0 {
                    // EOF
                    accepted.shutdown(net::Shutdown::Both).unwrap_or_else(|_| {
                        panic!("failed to shutdown the socket {:?}", accepted.as_fd())
                    });
                } else {
                    let response = http_handler.handle_request(buf);

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
}
