use crate::{crew::crew::Crew, modules::{errors::error::HawkTuahError, util::Rect}};

pub struct MQ1Reaper {
    pub capture_rect: Rect,
}

impl Crew for MQ1Reaper {
    fn setup() -> Result<MQ1Reaper, Box<dyn HawkTuahError>> {
        let mq1 = MQ1Reaper {
            capture_rect: Rect {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
            },
        };

        Ok(mq1)
    }

    fn run(&mut self) -> Result<(), Box<dyn HawkTuahError>> {
        Ok(())
    }
}

