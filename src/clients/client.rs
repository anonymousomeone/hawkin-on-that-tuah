use crate::modules::errors::error::HawkTuahError;

pub trait Client {
    fn setup() -> Self where Self: Sized;
    fn run(&mut self) -> Result<(), Box<dyn HawkTuahError>>;
}