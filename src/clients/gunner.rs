use std::{thread, time::Duration};

use crate::{clients::client::Client, modules::{self, errors::error::HawkTuahError, io::{KeyState, Keyboard, message_loop_keepalive}, networking::{Connection, Server}}};

pub struct Gunner {
    pub keyboard: Keyboard,
    pub connection: Connection,
    pub server: Server,
    pub hook_enabled: bool,
}

impl Client for Gunner {
    fn setup() -> Gunner {
        let server = Server::new();
        let mut keyboard = Keyboard::new();

        keyboard.install_hook();

            
        println!("Awaiting driver client connection...");
        let connection = server.await_connection();

        Gunner {
            keyboard,
            connection,
            server,
            hook_enabled: false,
        }
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
                    modules::io::Keyboard::set_hooking(self.hook_enabled);

                    println!("keyboard hook enabled: {}", self.hook_enabled);
                    continue;
                }

                messages.push(key.into());
            }

            self.connection.write(messages);

            message_loop_keepalive();
            thread::sleep(Duration::from_millis(5));
        }
    }
}