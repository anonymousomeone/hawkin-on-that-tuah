mod modules;
mod crew;

use crew::crew::Crew;

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
        _ => {
            println!("bad type, try again");
            return main();
        }
    };

    let mut crew: Box<dyn Crew> = match crew_type {
        CrewType::Gunner => {
            match crew::gunner::Gunner::setup() {
                Ok(gunner) => Box::new(gunner),
                Err(e) => {
                    println!("Error occurred during setup: {}", e);
                    return main();
                }
            }
        },
        CrewType::Driver => {
            match crew::driver::Driver::setup() {
                Ok(d) => Box::new(d),
                Err(e) => {
                    println!("Error occurred during setup: {}", e);
                    return main();
                }
            }
        },
        CrewType::MQ1Reaper => {
            match crew::mq1_reaper::MQ1Reaper::setup() {
                Ok(m) => Box::new(m),
                Err(e) => {
                    println!("Error occurred during setup: {}", e);
                    return main();
                }
            }
        },
    };

    println!("Connected!");

    match crew.run() {
        Ok(_) => {},
        Err(e) => {
            println!("Error occurred: {}", e);
            return main();
        }
    };
}