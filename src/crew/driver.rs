use std::{thread, time::Duration};

use crate::{CrewType, crew::crew::Crew, modules::{errors::{connection::ConnectionError, disconnected::DisconnectedError, error::HawkTuahError}, keyboard::{Key, KeyState, Keyboard}, networking::Client}};

pub struct Driver {
    pub client: Client,
}

impl Crew for Driver {
    fn setup() -> Result<Driver, Box<dyn HawkTuahError>> {
        println!("Enter server ip: ");

        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("Connecting to gunner server...");

        let client = match Client::new(&input.trim(), CrewType::Driver) {
            Ok(c) => c,
            Err(e) => {
                return Err(Box::new(ConnectionError {
                    details: format!("Couldnt connect to server (did you start the server?) {}", e)
                }));
            }
        };

        let driver = Driver {
            client,
        };

        Ok(driver)
    }

    fn run(&mut self) -> Result<(), Box<dyn HawkTuahError>> {
        loop {
            let messages = match self.client.tcp_connection.read() {
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