//#![allow(unused)]

use std::io::prelude::*;
use std::io;
use std::str;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("Got a client! {:?}", stream);

    stream.write(b"khi\n");
    stream.flush();

    let mut bytes: [u8; 4] = [0; 4];
    let mut last = 0;

    for byte in stream.bytes() {
        if let Ok(byte) = byte {

            bytes[last] = byte;
            last += 1;

            // print the rune if we're ready
            let newb = bytes.clone();
            let rune = str::from_utf8(&newb);
            match rune {
                Ok(rune) => {
                    print!("{}", rune);

                    // clear array
                    for i in 0..bytes.len() {
                        bytes[i] = 0;
                    }
                    last = 0;
                }
                Err(_) => {
                    if last == 3 {
                        return Err(io::Error::new(io::ErrorKind::Other, "malformed utf8"));
                    }
                }
            }
        }
    }
    println!("uh byee");

    Ok(())
}

fn main() -> io::Result<()> {

    let client_port = 8715;
    let _client4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), client_port);
    let _client6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), client_port);

    let server_port = 8716;
    let _server4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), server_port);
    let _server6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), server_port);

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
