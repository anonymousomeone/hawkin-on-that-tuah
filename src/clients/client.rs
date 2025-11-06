pub trait Client {
    fn setup() -> Self where Self: Sized;
    fn run(&mut self);
}