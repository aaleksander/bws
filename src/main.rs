//use chrono::{DateTime, Datelike, Local};

mod dt;
mod history;
mod db;
//use crate::msgs::message::MessagePayload;


fn main() 
{
    let gazp = history::series::Series::new("GAZP");

    println!("{}", gazp.count());
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

