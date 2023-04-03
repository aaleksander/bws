//use chrono::{DateTime, Datelike, Local};
#[macro_use]
extern crate ini;


use bws::Bws;
use history::History;

use crate::strategy::CommonStrategy;

mod bws;
mod db;
mod dt;
mod history;
mod strategy;

fn main() {
    println!("get settings");
    let map = ini!("settings.ini");
    let connection_string = map["db"]["postgres"].clone().unwrap();

    //let connection_string = String::from("postgresql://postgres:123123@localhost:5432/postgres");
//    let i = Ini::load_from_file("conf.ini").unwrap();

    println!("create strategy");
    let bws: Bws = Bws::new(100_000.0, &connection_string);

    println!("main loop");
    while bws.step() {
        bws.execute();
    }

    println!("result: {}", bws.amount);

    //println!("{}", gazp.count());
    //gazp.skip(5);

    //    let gmkn = history::series::Series::new_one(String::from("GMKN"), &db);

    // let dt: DateTime<Local> = dt::now();
    // println!(
    //     "{:0>2}.{:0>2}.{} {:?}",
    //     dt.day().to_string(),
    //     dt.month(),
    //     dt.year(),
    //     dt.weekday().num_days_from_monday() + 1
    // );

    // let history = History::new();
    // history.load_to(dt::ymd(2020, 1, 1));

    // let bws = BwsStrategy::new(history, 8);

    // let res = bws.test_to(dt::now());

    // println!("{:?}", res);
}
