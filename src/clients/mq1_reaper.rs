use crate::clients::client::Client;

pub struct MQ1Reaper {
    pub capture_rect: Rect,
}

impl Client for MQ1Reaper {
    fn setup() -> MQ1Reaper {
        MQ1Reaper {
            capture_rect: Rect {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
            },
        }
    }

    fn run(&mut self) {

    }
}

struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}