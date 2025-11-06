use std::{thread, time::Duration};

use crate::{clients::client::Client, modules::{io::{Key, KeyState, Keyboard}, networking::{Client as NetClient, Connection}}};

pub struct Driver {
    pub client: NetClient,
    pub connection: Connection,
}

impl Client for Driver {
    fn setup() -> Driver {
        println!("Enter server ip: ");

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("Connecting to gunner server...");

        let client = NetClient::new(&input.trim());
        let connection = Connection::new(client.connection.try_clone().unwrap());
        
        Driver {
            client,
            connection,
        }
    }

    fn run(&mut self) {
        loop {
            let messages = self.connection.read();

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