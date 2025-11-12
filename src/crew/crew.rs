use crate::modules::errors::error::HawkTuahError;

pub trait Crew {
    fn setup() -> Result<Self, Box<dyn HawkTuahError>> where Self: Sized;
    fn run(&mut self) -> Result<(), Box<dyn HawkTuahError>>;
}