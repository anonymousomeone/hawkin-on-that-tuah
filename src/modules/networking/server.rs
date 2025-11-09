use std::net::{TcpListener, TcpStream, UdpSocket};

use crate::modules::networking::{SERVER_ADDR, TcpConnection};

pub struct Server {
    pub listener: TcpListener,
    pub tcp_connection: Option<TcpStream>,
    pub udp_connection: Option<UdpSocket>,
}

impl Server {
    pub fn new() -> Result<Server, std::io::Error> {
        let listener = TcpListener::bind(SERVER_ADDR)?;

        let server = Server {
            listener,
            tcp_connection: None,
            udp_connection: None,
        };

        Ok(server)
    }

    pub fn await_tcp_connection(&self) -> Result<TcpConnection, std::io::Error> {
        let (stream, _addr) = self.listener.accept()?;

        Ok(TcpConnection::new(stream))
    }

    pub fn await_udp_connection(&self) -> Result<(), std::io::Error> {
        let socket = UdpSocket::bind(SERVER_ADDR)?;



        Ok(())
    }
}