use crate::validate::{getDaysOfMonth, getCheckCode};
use crate::code2address::getBook;
use crate::progress;

//考虑的年份
pub const YEAR: (i32, i32) = (1990, 2020);

pub fn total() -> usize {
    let book = getBook();
    book.len() * (YEAR.1 - YEAR.0) as usize * 365 * 1000
}

pub fn go(callback: fn(&String)) {
    let book = getBook();
    let mut bar = progress::Progress::new(total(), String::from("生成身份证号"));
    for addr in book.keys() {
        for year in YEAR.0..YEAR.1 {
            for month in 1..12 {
                for day in 1..getDaysOfMonth(year, month) {
                    //序列号
                    for serie in 0..=999 {
                        let mut idcard = format!("{}{:04}{:02}{:02}{:03}", addr, year, month, day, serie);
                        let checkCode = getCheckCode(&idcard);
                        idcard.push(checkCode);
                        callback(&idcard);
                        bar.update(1);
                    }
                }
            }
        }
    }
}