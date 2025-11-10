use std::net::{SocketAddr, TcpListener, UdpSocket};

use crate::modules::networking::{SERVER_ADDR, TcpConnection};

pub struct Server {
    pub listener: TcpListener,
    pub tcp_connection: Option<TcpConnection>,
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

    pub fn await_tcp_connection(&mut self) -> Result<SocketAddr, std::io::Error> {
        let (stream, addr) = self.listener.accept()?;
        self.tcp_connection = Some(TcpConnection::new(stream));

        Ok(addr)
    }

    pub fn await_udp_connection(&self, remote_addr: &str) -> Result<(), std::io::Error> {
        let socket = UdpSocket::bind(SERVER_ADDR)?;
        socket.connect(remote_addr)?;

        Ok(())
    }
}