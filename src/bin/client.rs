use std::net::TcpStream;
use std::io::prelude::Write;

fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:3000").expect("Could not connect");
    
    socket.write("test".as_bytes()).expect("Could not write to socket");
}