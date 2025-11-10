use std::{thread, time::Duration};

use crate::{clients::client::Client, modules::{self, errors::{connection::ConnectionError, disconnected::DisconnectedError, error::HawkTuahError}, keyboard::{KeyState, Keyboard, message_loop_keepalive}, networking::{TcpConnection, Server}}};

pub struct Gunner {
    pub keyboard: Keyboard,
    pub server: Server,
    pub hook_enabled: bool,
}

impl Client for Gunner {
    fn setup() -> Result<Gunner, Box<dyn HawkTuahError>> {
        let mut server = match Server::new() {
            Ok(s) => s,
            Err(e) => {
                return Err(Box::new(ConnectionError {
                    details: format!("Couldnt start server {}", e)
                }));
            }
        };

        let mut keyboard = Keyboard::new();

        match keyboard.install_hook() {
            Ok(k) => k,
            Err(e) => {
                return Err(Box::new(ConnectionError {
                    details: format!("Couldnt install keyboard hook {}", e)
                }));
            }
        };

            
        println!("Awaiting driver client connection...");
        let _remote_addr = match server.await_tcp_connection() {
            Ok(c) => c,
            Err(e) => {
                return Err(Box::new(ConnectionError {
                    details: format!("Driver connection issue {}", e)
                }));
            }
        };

        let gunner = Gunner {
            keyboard,
            server,
            hook_enabled: false,
        };

        Ok(gunner)
    }

    fn run(&mut self) -> Result<(), Box<dyn HawkTuahError>> {
        loop {
            self.keyboard.parse_callbacks();
            let keys = self.keyboard.state_changes.clone();
            self.keyboard.state_changes.clear();
            let mut messages = Vec::with_capacity(keys.len());

            for key in keys {
                // lctrl key
                if key.key_code == 0xA2 {
                    if key.key_state == KeyState::Up { continue; }

                    self.hook_enabled = !self.hook_enabled;
                    modules::keyboard::Keyboard::set_hooking(self.hook_enabled);

                    println!("keyboard hook enabled: {}", self.hook_enabled);
                    continue;
                }

                messages.push(key.into());
            }

            match self.server.tcp_connection.as_mut().unwrap().write(messages) {
                Ok(_) => {},
                Err(_) => return Err(Box::new(DisconnectedError {})),
            };

            message_loop_keepalive();
            thread::sleep(Duration::from_millis(5));
        }
    }
}