use std::net::{SocketAddr, UdpSocket};
use std::str::from_utf8;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

const GROUP_IP: &str = "239.0.0.1:3000";

// Representa un nodo, tiene un identificador tipo Long
struct Node {
    id: i32
}

fn main() {
/*
 * Protocolo de comunicación:
 * Externo: se maneja con el tcp, a lo sumo ACK y respuesta.
 * Interno: 3 o 4 chars y un espacio 
 * JOIN
 * MSG
 * ACK
 * 
 */


    /*
     * 1. escuchar en dirección multicast
     * 2. poner un servidor para afuera de grupo en la ip privada del coordinador
     * 3. cuando llega un mensaje distribuirlo por la dirección multicast
     * 4. esperar respuesta de todos los nodos
     * 5. cuando recibe respuesta de todos, mandar confirmación al nodo externo.
     * 
     * Debe mantener una tabla con todos los nodos >> un loop escuchando anuncios.
     */

    let (tx, rx): (Sender<&str>, Receiver<&str>) = mpsc::channel();

    let internal_listener_tx = tx.clone();

    let handle = thread::spawn(move || {
        // escucha a la red interna de multicast
        println!("Listening on internal group multicast address");
        let received_message = listen(GROUP_IP);
        internal_listener_tx.send(received_message);
    });

    let internal_received_message = rx.recv();

    handle_message(internal_received_message.unwrap());


    // espero que termine el hijo.
    handle.join();

}

fn listen(addr: &str) -> &str {
    let mut socket = UdpSocket::bind(addr).expect("Error binding to internal group address");
    let mut buf:[u8; 512] = [0; 512];
    let (amt, src) = socket.recv_from(&mut buf).expect("Error obtaining data from socket");
    let message = from_utf8(&mut buf).unwrap();
    return message;
}

fn handle_message(message: &str) {
    match message {
        "JOIN" => add_node(),
    }
}

fn add_node() {todo!("implementar add_node")}

fn handle_connection(mut stream: UdpSocket, addr: SocketAddr) {
    println!("Connected to {}", addr.to_string());
    let mut buf = [0; 512];
    stream.read(&mut buf).expect("Error reading stream!");
    let message = from_utf8(&mut buf).unwrap();
}

fn send_message() {
    let mut socket = UdpSocket::connect("127.0.0.1:3000").expect("Could not connect");
        
        socket.write("test".as_bytes()).expect("Could not write to socket");
}
