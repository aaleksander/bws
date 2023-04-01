use crate::{history::candle::Candle, dt};


pub fn load_candles(_sec_code: &str, _period: &str) -> Vec<Candle>
{
    let mut res: Vec<Candle> = vec![];

    res.push(Candle::new(
        dt::ymd(2023, 10, 3),
        3.0, 4.0, 5.0, 6.0, 7
    ));

    return res;
}