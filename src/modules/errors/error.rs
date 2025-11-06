use std::error::Error;
use std::fmt::Display;
use std::fmt::Debug;

pub trait HawkTuahError: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)>;
}