//тут всякое для работы с датами

use chrono::{DateTime, Datelike, Local, TimeZone};

#[allow(dead_code)]
pub fn now() -> DateTime<Local> {
    return chrono::offset::Local::now();
}

#[allow(dead_code)]
pub fn get_day_of_week() -> u32 {
    return __get_day_of_week(chrono::offset::Local::now());
}

#[allow(dead_code)]
pub fn ymd(year: i32, month: u32, day: u32) -> DateTime<Local> {
    return chrono::Local
        .with_ymd_and_hms(year, month, day, 0, 0, 0)
        .unwrap();
}

fn __get_day_of_week(dt: DateTime<Local>) -> u32 {
    return dt.weekday().num_days_from_monday() + 1;
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Local, NaiveDate, NaiveWeek, TimeZone};

    use crate::dt::{__get_day_of_week, ymd};

    #[test]
    fn test_00() {
        assert_eq!(__get_day_of_week(ymd(2023, 3, 27)), 1, "1"); //понедельник
        assert_eq!(__get_day_of_week(ymd(2023, 4, 1)), 6, "1"); //суббота
    }
}
