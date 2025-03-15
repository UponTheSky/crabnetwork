use std::fmt::Display;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::{self, TcpListener};
use std::os::fd::AsFd;
use std::thread;

mod http;
mod tcp_server;

fn main() {
    let server_config = tcp_server::Config::new("localhost".into(), 8080);
    let server = tcp_server::Server::new(server_config);

    server.run();
}
