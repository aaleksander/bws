//use chrono::{DateTime, Local};

use crate::db;

use super::candle::Candle;

//История по одному тикеру


pub struct Series{
    pub security: String,
    pub candles: Vec<Candle>,

    index: usize, //индекс, который указывает на исторические данные
}

impl Series{
    pub fn new(sec_code: &str) -> Series
    {
        return Series{
            security: sec_code.to_string(),
            candles: db::load_candles(sec_code, "w"),
            index: 0
        };
    }

    pub fn count(&self)->usize{
        return self.candles.len();
    }

    #[allow(dead_code)]
    pub fn skip(mut self, _index: usize)
    {
        self.index += _index;
        if self.index >= self.count() {
            panic!("out of range in candles");
        }
    }
}


