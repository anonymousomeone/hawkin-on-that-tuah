use std::net::TcpStream;

pub struct Client {
    pub connection: TcpStream
}

impl Client {
    pub fn new(address: &str) -> Result<Client, std::io::Error> {
        let connection =  TcpStream::connect(address)?;

        let client = Client { 
            connection,
        };

        Ok(client)
    }
}