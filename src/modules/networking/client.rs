use std::net::{TcpStream, UdpSocket};

use crate::{CrewType, modules::networking::TcpConnection};

pub struct Client {
    pub tcp_connection: TcpConnection,
    pub udp_connection: UdpSocket,
    pub client_type: CrewType,
}

impl Client {
    pub fn new(address: &str, client_type: CrewType) -> Result<Client, std::io::Error> {
        let tcp_connection =  TcpConnection::new(TcpStream::connect(address)?);
        let udp_connection = UdpSocket::bind(address)?;

        let client = Client { 
            tcp_connection,
            udp_connection,
            client_type,
        };

        Ok(client)
    }
}