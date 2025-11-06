use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use super::io::Key;
use super::io::KeyState;

pub const SERVER_ADDR: &str = "0.0.0.0:1984";

// i love rawdogging tcp
pub struct Connection {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream.try_clone().unwrap());

        Connection {
            stream,
            reader,
            writer
        }
    }

    pub fn read(&mut self) -> Result<Vec<Message>, Error> {
        let mut bytes = [0; 128];
        let bytes_read = self.stream.read(&mut bytes)?;

        let mut messages = Vec::with_capacity(bytes.len());
        for i in 0..bytes_read {
            let message = Message::from_byte(bytes[i]);
            messages.push(message);
        }

        return Ok(messages);
    }

    pub fn write(&mut self, messages: Vec<Message>) -> Result<(), Error> {
        let mut bytes = Vec::with_capacity(messages.len());

        for message in messages {
            let byte = message.as_byte();
            bytes.push(byte);
        }

        self.stream.write(&bytes)?;
        self.stream.flush().unwrap();

        Ok(())
    }
}

pub struct Server {
    pub listener: TcpListener,
    pub connection: Option<TcpStream>
}

impl Server {
    pub fn new() -> Result<Server, std::io::Error> {
        let listener = TcpListener::bind(SERVER_ADDR)?;

        let server = Server {
            listener,
            connection: None,
        };

        Ok(server)
    }

    pub fn await_connection(&self) -> Result<Connection, std::io::Error> {
        let (stream, _addr) = self.listener.accept()?;

        Ok(Connection::new(stream))
    }
}

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

#[derive(Debug)]
pub enum Message {
    WDown,
    ADown,
    SDown,
    DDown,
    QDown,
    EDown,
    WUp,
    AUp,
    SUp,
    DUp,
    QUp,
    EUp,
}

impl Message {
    pub fn as_byte(self) -> u8 {
        let code = match self {
            Message::WDown => 0,
            Message::ADown => 1,
            Message::SDown => 2,
            Message::DDown => 3,
            Message::QDown => 4,
            Message::EDown => 5,
            Message::WUp => 6,
            Message::AUp => 7,
            Message::SUp => 8,
            Message::DUp => 9,
            Message::QUp => 10,
            Message::EUp => 11,
        };

        return code
    }

    pub fn from_byte(code: u8) -> Message {
        let message = match code {
            0 => Message::WDown,
            1 => Message::ADown,
            2 => Message::SDown,
            3 => Message::DDown,
            4 => Message::QDown,
            5 => Message::EDown,
            6 => Message::WUp,
            7 => Message::AUp,
            8 => Message::SUp,
            9 => Message::DUp,
            10 => Message::QUp,
            11 => Message::EUp,
            _ => panic!("unexpected message code"),
        };

        message
    }
}

impl From<Key> for Message {
    fn from(key: Key) -> Self {
        match key.key_state {
            KeyState::Up => {
                match key.key_code {
                    87 => Message::WUp,
                    65 => Message::AUp,
                    83 => Message::SUp,
                    68 => Message::DUp,
                    81 => Message::QUp,
                    69 => Message::EUp,
                    _ => panic!("trolled"),
                }
                
            },
            KeyState::Down => {
                match key.key_code {
                    87 => Message::WDown,
                    65 => Message::ADown,
                    83 => Message::SDown,
                    68 => Message::DDown,
                    81 => Message::QDown,
                    69 => Message::EDown,
                    _ => panic!("trolled"),
                }
            }
        }
    }
}