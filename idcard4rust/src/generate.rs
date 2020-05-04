use std::ops::{Add, Sub};

use chrono::{Date, Duration, Utc};

use crate::code2address::getBook;
use crate::progress;
use crate::validate::getCheckCode;

pub fn total(daysCount: i32) -> usize {
    let book = getBook();
    book.len() * daysCount as usize * 365 * 1000
}

pub fn go(begDate: Date<Utc>, endDate: Date<Utc>, callback: fn(&String)) {
    let book = getBook();
    let days = endDate.sub(begDate).num_days();
    let mut bar = progress::Progress::new(total(days as i32), String::from("生成身份证号"));
    for addr in book.keys() {
        let mut ymd = begDate;
        while ymd < endDate {
            //序列号
            for serie in 0..=999 {
                let mut idcard = format!("{}{}{:03}", addr, ymd.format("%Y%m%d"), serie);
                let checkCode = getCheckCode(&idcard);
                idcard.push(checkCode);
                callback(&idcard);
                bar.update(1);
            }
            ymd = ymd.add(Duration::days(1));
        }
    }
}