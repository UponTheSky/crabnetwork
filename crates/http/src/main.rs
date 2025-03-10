use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::thread;
use std::{
    net::{self, TcpListener},
    time::Duration,
};

fn main() {
    let addr = "127.0.0.1:0"; // ask OS to provide the port number
    let listener = TcpListener::bind(addr).unwrap(); // create socket, bind to the address, and listen

    // accept loop
    for stream in listener.incoming() {
        if let Ok(mut accepted) = stream {
            // make a multi-thread for the accepted stream(socket)

            thread::spawn(move || {
                accepted
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .unwrap_or_else(|_| panic!("failed to set read timeout on the accept socket {:?}",
                        accepted.as_fd()));

                accepted
                    .set_write_timeout(Some(Duration::from_secs(10)))
                    .unwrap_or_else(|_| panic!("failed to set write timeout on the accept socket {:?}",
                        accepted.as_fd()));

                let mut buf = vec![];

                match accepted.read(&mut buf) {
                    Ok(len) => {
                        if len == 0 {
                            // EOF
                            accepted.shutdown(net::Shutdown::Both).unwrap_or_else(|_| panic!("failed to shutdown the socket {:?}",
                                accepted.as_fd()));
                        } else {
                            // echo server
                            println!(
                                "{}",
                                String::from_utf8(buf.clone())
                                    .expect("failed to convert byte to string")
                            );
                            accepted.write(&buf).unwrap_or_else(|_| panic!("failed to send the message from the socket {:?}",
                                accepted.as_fd()));
                        }
                    }
                    Err(error) => {
                        println!("{}", error);
                    }
                }
            });
        }
    }
}
