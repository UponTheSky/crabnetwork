mod threadpool;

use std::io::{BufWriter, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::os::fd::AsFd;
use std::sync::Arc;
use std::thread;

use crate::http::handler::HttpHandler;
use threadpool::ThreadPool;

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

        let pool = ThreadPool::new(4);

        // accept loop
        for stream in listener.incoming() {
            let http_handler = Arc::clone(&self.http_handler);

            match stream {
                Ok(accepted) => {
                    // make a multi-thread for the accepted stream(socket)
                    pool.execute(move || {
                        Self::handle_tcp_stream(accepted, http_handler);
                    });
                }
                Err(error) => {
                    println!("error while listening tcp requests: {error}");
                }
            };
        }
    }

    fn handle_tcp_stream(accepted: TcpStream, http_handler: Arc<HttpHandler>) {
        // check if the stream is EOF first
        match http_handler.handle_request(&accepted) {
            Ok(response) => {
                let mut res_buf = BufWriter::new(&accepted);
                res_buf.write_all(&response.encode()).unwrap_or_else(|_| {
                    panic!(
                        "failed to send the message from the socket {:?}",
                        accepted.as_fd()
                    )
                });
            }
            Err(tcp_error) => {
                eprintln!("Incoming tcp request is EOF: {}", tcp_error);
            }
        }
    }
}
