use std::net::{self, SocketAddr, TcpListener, ToSocketAddrs};

fn main() {
    let addr = "127.0.0.1:0"; // ask OS to provide the port number
    let listener = TcpListener::bind(addr).unwrap(); // create socket, bind to the address, and listen

    /// accept loop
    for stream in listener.incoming() {
        if let Ok(accepted) = stream {
            // make a multi-thread for the accepted stream(socket)
        }
    }
}
