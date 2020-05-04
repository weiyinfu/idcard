use crate::generate;
use crate::prime;
use chrono::{Date, Utc, TimeZone};

pub(crate) fn go() {
    fn iterate(idcard: &String) {
        if idcard.ends_with("X") {
            return;
        }
        let num = idcard.parse::<i64>().unwrap();
        if (num & 1) == 0 {
            return;
        }
        let isPrime = prime::smallNumberMillerRabin(num);
        if isPrime {
            println!("found prime idcard {}", idcard);
        }
    }
    let begDate = Date::from(Utc.ymd(2020, 5, 1));
    let endDate = Date::from(Utc.ymd(2020, 6, 1));
    generate::go(begDate, endDate, iterate);
}