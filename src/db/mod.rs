use crate::{dt, history::candle::Candle};
use chrono::{NaiveDate, Datelike};
use postgres::{Client, NoTls, types::FromSql};

pub fn load_candles(sec_code: &str, _period: &str, connection_string: &str) -> Vec<Candle> {
    let mut res: Vec<Candle> = vec![];

    let mut client: Client = match Client::connect(connection_string, NoTls) {
        Ok(res) => res,
        Err(_) => panic!("can't open db"),
    };

    let table_name = format!("history_{sec_code}_w").to_lowercase();
    let sql = format!(r#"select * from "LazyInvestor".{table_name} order by dt"#);

    for row in client.query(&sql, &[]).unwrap() {
        let dt_str: String = row.get("dt");
        let dt = NaiveDate::parse_from_str(&dt_str, "%Y.%m.%d %H:%M:%S").unwrap();
        // match dt_res {
        //     Ok(dt)=> {println!("{}", dt);}
        //     Err(msg)=> {println!("{}", msg);}
        // }

        let o: f64 = row.get("open");
        let h: f64 = row.get("high");
        let l: f64 = row.get("low");
        let c: f64 = row.get("close");
        let v: i32 = row.get("volume");

        res.push(Candle::new(dt::ymd(dt.year(), dt.month(), dt.day()), o, h, l, c, v as u32));
    }

    return res;
}

pub fn get_active_security(connection_string: &str) -> Vec<String> {
    let mut client: Client = Client::connect(connection_string, NoTls).unwrap();

    let mut res: Vec<String> = vec![];

    for row in client
        .query(
            r#"select sec_code from "LazyInvestor".security where is_active=true"#,
            &[],
        )
        .unwrap()
    {
        let name: String = row.get("sec_code");
        res.push(name);
    }

    res
}
