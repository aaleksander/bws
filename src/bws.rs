use crate::{
    history::{self, History},
    strategy::CommonStrategy,
};

pub struct Bws {
    pub hist: History,
    pub amount: f64,
}

impl Bws {
    pub fn new(_amount: f64, connection_string: &str) -> Bws {
        return Bws {
            amount: _amount,
            hist: history::new(connection_string),
        };
    }
}

impl CommonStrategy for Bws {
    fn execute(&self) {
        println!("main");
    }

    fn step(&self) -> bool {
        return self.hist.step();
    }
}
