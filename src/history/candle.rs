use chrono::{DateTime, Local};

#[allow(dead_code)]
pub struct Candle {
    dt: DateTime<Local>,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: u64,
}

impl Candle {
    pub fn new(date_time: DateTime<Local>, o: f64, h: f64, l: f64, c: f64, v: u64) -> Candle {
        return Candle {
            dt: date_time,
            open: o,
            high: h,
            low: l,
            close: c,
            volume: v,
        };
    }
}
