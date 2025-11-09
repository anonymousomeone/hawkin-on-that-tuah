use std::{thread, time::Duration};

use crate::{clients::client::Client, modules::{errors::{connection::ConnectionError, disconnected::DisconnectedError, error::HawkTuahError}, keyboard::{Key, KeyState, Keyboard}, networking::{Client as NetClient, TcpConnection}}};

pub struct Driver {
    pub client: NetClient,
    pub connection: TcpConnection,
}

impl Client for Driver {
    fn setup() -> Result<Driver, Box<dyn HawkTuahError>> {
        println!("Enter server ip: ");

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("Connecting to gunner server...");

        let client = match NetClient::new(&input.trim()) {
            Ok(c) => c,
            Err(e) => {
                return Err(Box::new(ConnectionError {
                    details: format!("Couldnt connect to server (did you start the server?) {}", e)
                }));
            }
        };

        let connection = TcpConnection::new(client.connection.try_clone().unwrap());
        
        let driver = Driver {
            client,
            connection,
        };

        Ok(driver)
    }

    fn run(&mut self) -> Result<(), Box<dyn HawkTuahError>> {
        loop {
            let messages = match self.connection.read() {
                Ok(msgs) => msgs,
                Err(_) => return Err(Box::new(DisconnectedError {})),
            };

            for message in messages {
                let key: Key = message.into();

                match key.key_state {
                    KeyState::Down => Keyboard::press_key(key.key_code),
                    KeyState::Up => Keyboard::release_key(key.key_code),
                }
            }

            thread::sleep(Duration::from_millis(50));
        }
    }
}