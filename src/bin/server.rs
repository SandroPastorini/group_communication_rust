use std::{net::{TcpListener, TcpStream, SocketAddr}, io::Read, str};


/**
 * spawn server listening in new thread.
 * 
 */



fn handle_connection(mut stream: TcpStream, addr: SocketAddr) {
    println!("Connected to {}", addr.to_string());
    let mut buf = [0; 512];
    stream.read(&mut buf).expect("Error reading buffer!");
    let message = str::from_utf8(&mut buf).unwrap();
    println!("Result: {}", message);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    match listener.accept() {
        Ok((socket, addr)) => { handle_connection(socket, addr);}
        Err(_) => {println!("Error connecting to client!");}
    }
}