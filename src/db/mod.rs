use crate::{dt, history::candle::Candle};
use postgres::{Client, NoTls};

pub fn load_candles(_sec_code: &str, _period: &str, connection_string: &str) -> Vec<Candle> {
    let mut res: Vec<Candle> = vec![];

    let mut client: Client = match Client::connect(connection_string, NoTls) {
        Ok(res) => res,
        Err(_) => panic!("can't open db"),
    };

    for row in client
        .query(r#"select * from "LazyInvestor".sectors"#, &[])
        .unwrap()
    {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);        
    }

    res.push(Candle::new(dt::ymd(2023, 10, 3), 3.0, 4.0, 5.0, 6.0, 7));

    return res;
}

pub fn get_active_security(connection_string: &str) -> Vec<String> {
    let mut client: Client = Client::connect(connection_string, NoTls).unwrap();

    let mut res: Vec<String> = vec![];

    for row in client
        .query(r#"select sec_code from "LazyInvestor".security where is_active=true"#, &[])
        .unwrap()
    {
        let name: String = row.get("sec_code");
        res.push(name);
    }

    res
}
