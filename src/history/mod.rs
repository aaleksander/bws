pub mod candle;
pub mod series;

use self::series::Series;
use crate::db;

pub struct History {
    pub series: Vec<Series>,
}

impl History {
    ///сбрасывает состояние истории на первоначальный вид
    pub fn reset(&self) {}
    pub fn is_end(&self) -> bool {
        true
    }
    pub fn step(&self) -> bool {
        println!("history step");
        false
    }
}

///Загружаем из бд всю историю по активным тикерам
pub fn new(connection_string: &str) -> History {
    //загружаем из базы всю историю по активным тикерам
    let mut res = History { series: vec![] };

    let secs = db::get_active_security(connection_string);

    for sec in secs {
        res.series.push(Series::new(&sec, connection_string));
    }

    res.reset();

    return res;
}
