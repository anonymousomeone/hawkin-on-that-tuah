mod modules;
mod clients;

use clients::client::Client;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CrewType {
    Gunner,
    Driver,
    MQ1Reaper,
}

fn main() {
    println!("Hello, world!");
    println!("Enter a crew type (gunner or driver): ");
    
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let crew_type: CrewType = match input.trim() {
        "gunner" => CrewType::Gunner,
        "driver" => CrewType::Driver,
        "mq1" => CrewType::MQ1Reaper,
        _ => panic!("bad type, try again")
    };

    let mut client: Box<dyn Client> = match crew_type {
        CrewType::Gunner => {
            Box::new(clients::gunner::Gunner::setup())
        },
        CrewType::Driver => {
            Box::new(clients::driver::Driver::setup())
        },
        CrewType::MQ1Reaper => {
            Box::new(clients::mq1_reaper::MQ1Reaper::setup())
        },
    };

    println!("Connected!");

    client.run();
}