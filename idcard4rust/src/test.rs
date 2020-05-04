#![allow(unused_imports)]

use std::io::{stdout, Write};
use std::ops::Add;

use chrono::{Date, Duration, TimeZone, Utc};

use crate::{generate, validate};
use crate::code2address::getBook;
use crate::generate::go;
use crate::prime::{linearPrime, smallNumberMillerRabin};
use crate::progress::Progress;
use crate::validate::getCheckCode;

#[test]
fn testBook() {
    let book = getBook();
    println!("{}", book.len());
}

#[test]
fn testValidate() {
    let idcard = "130533199410173510";
    let res = validate::validate(&idcard.to_string());
    println!("{}={}$", idcard, res);
    //132235是历史上的行政区划编码，现在已经废弃了
    let idcard = "13223519640606352X";
    let res = validate::validate(&idcard.to_string());
    println!("{}={}$", idcard, res);
}

#[test]
fn checkCode() {
    let idcard = "13223519640606352";
    let checkCode = getCheckCode(&idcard.to_string());
    println!("{}", checkCode);
}

#[test]
fn generateAll() {
    //生成所有的身份证号
    generate::go(Date::from(Utc.ymd(1999, 10, 01)),
                 Date::from(Utc.ymd(2020, 10, 1)),
                 |idcard| {
                     println!("{}", idcard);
                 });
}

#[test]
fn testPrime() {
    let n = 5000;
    let mut a = Vec::new();
    a.resize(n, false);
    for i in 2..a.len() {
        let is = smallNumberMillerRabin(i as i64);
        a[i] = is;
        if is && i < 20 {
            print!("{},", i);
        }
    }
    println!();
    let b = linearPrime(n);
    for i in 2..20 {
        if b[i] {
            print!("{},", i);
        }
    }
    println!();
    for i in 2..n {
        if a[i] != b[i] {
            println!("error {}", i);
        }
    }
    println!("over");
}


#[test]
fn testIsPrime() {
    let num = "110102200001011159".parse().unwrap();
    let res = smallNumberMillerRabin(num);
    println!("{}", res);
}

#[test]
fn maxIdcard() {
    //最大的身份证号也没有超过i64
    let num: i128 = "999999999999999999".parse().unwrap();
    println!("{}", num);
    println!("{}", (1i128 << 63));
}

#[test]
fn iterateDay() {
    //迭代日期
    let now = chrono::Date::from(Utc.ymd(1993, 10, 01));
    for i in 1..10 {
        let t = now.add(Duration::days(i));
        println!("{}", t.format("%Y-%m-%d"));
    }
    println!("{}",now.format("%Y-%m-%d"));
}