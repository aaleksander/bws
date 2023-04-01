use chrono::{DateTime, Local};
use std::fmt;

use crate::db::database::DataBase;

//История по одному тикеру
pub struct Candle {
    dt: DateTime<Local>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub struct Series{
    pub security: String,
    pub candles: Vec<Candle>,
}

impl Series{
    pub fn new_one(secCode: String, db: &DataBase) -> Series
    {
        return Series{
            security: secCode,
            candles: Vec::new(),
        };
    }
}


