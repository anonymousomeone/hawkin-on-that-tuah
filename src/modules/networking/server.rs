use std::{net::{SocketAddr, TcpListener, TcpStream, UdpSocket}, sync::Mutex};

use crate::{CrewType, modules::networking::{Client, SERVER_ADDR, TcpConnection}};

pub struct Server {
    pub listener: TcpListener,
    pub clients: Mutex<Vec<Client>>,
    pub broadcast_socket: UdpSocket,
}

impl Server {
    pub fn new() -> Result<Server, std::io::Error> {
        let listener = TcpListener::bind(SERVER_ADDR)?;

        let socket = UdpSocket::bind("255.255.255.255").unwrap();
        socket.set_broadcast(true).expect("set_broadcast call failed");

        let server = Server {
            listener,
            clients: Mutex::new(Vec::new()),
            broadcast_socket: socket,
        };

        Ok(server)
    }

    pub fn broadcast_discovery(&self) {
        let discovery_message = b"HAWK_TUAH_DISCOVERY";
        self.broadcast_socket.send(discovery_message).unwrap();
    }

    pub fn handle_connections(&mut self) -> Result<(), std::io::Error> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(tcp_stream) => {
                    let udp_connection = UdpSocket::bind(tcp_stream.peer_addr().unwrap())?;
                    let client = Client {
                        tcp_connection: TcpConnection::new(tcp_stream),
                        udp_connection,
                        client_type: CrewType::Gunner,
                    };
                    self.clients.lock().unwrap().push(client);
                    println!("New client connected!");
                }
                Err(e) => {
                    println!("Error accepting connection: {}", e);
                }
            }
        }
        Ok(())
    }
}