use std::{thread, time::Duration};

use modules::io::*;
use modules::networking::*;

mod modules;

struct Multiboxer {
    crew_type: CrewType,
    connection: Connection,
    server: Option<Server>,
    client: Option<Client>,
    hook_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CrewType {
    Gunner,
    Driver
}

fn main() {
    println!("Hello, world!");
    println!("Enter a crew type (gunner or driver): ");
    
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let crew_type: CrewType = match input.trim() {
        "gunner" => CrewType::Gunner,
        "driver" => CrewType::Driver,
        _ => panic!("bad type, try again")
    };

    let multiboxer = match crew_type {
        CrewType::Gunner => {
            let server = Server::new();
            
            println!("Awaiting driver client connection...");
            let connection = server.await_connection();
            Multiboxer {
                crew_type,
                connection,
                server: Some(server),
                client: None,
                hook_enabled: true,
            }
        },
        CrewType::Driver => {
            println!("Enter server ip: ");

            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            println!("Connecting to gunner server...");
    
            let client = Client::new(&input.trim());
            let connection = Connection::new(client.connection.try_clone().unwrap());
            
            Multiboxer {
                crew_type,
                connection,
                server: None,
                client: Some(client),
                hook_enabled: true,
            }
        },
    };
    println!("Connected!");
    
    match crew_type {
        CrewType::Gunner => do_gunner(multiboxer),
        CrewType::Driver => do_driver(multiboxer),
    }
}

fn do_gunner(mut multiboxer: Multiboxer) {
    let mut keyboard = Keyboard::new();
    keyboard.install_hook();

    loop {
        keyboard.parse_callbacks();
        let keys = keyboard.state_changes.clone();
        keyboard.state_changes.clear();
        let mut messages = Vec::with_capacity(keys.len());

        for key in keys {
            // lctrl key
            if key.key_code == 0xA2 {
                if key.key_state == KeyState::Up { continue; }

                multiboxer.hook_enabled = !multiboxer.hook_enabled;
                modules::io::Keyboard::set_hooking(multiboxer.hook_enabled);

                println!("keyboard hook enabled: {}", multiboxer.hook_enabled);
                continue;
            }

            messages.push(key.into());
        }

        multiboxer.connection.write(messages);

        message_loop_keepalive();
        thread::sleep(Duration::from_millis(5));
    }

}

fn do_driver(mut multiboxer: Multiboxer) {
    loop {
        let messages = multiboxer.connection.read();

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