pub trait CommonStrategy {
    fn execute(&self);
    fn step(&self) -> bool;
}
