#![allow(unused_imports)]

use crate::{validate, generate};
use crate::generate::{YEAR, go};
use crate::validate::getCheckCode;
use crate::code2address::getBook;
use crate::prime::{smallNumberMillerRabin, linearPrime};
use std::io::{stdout, Write};
use crate::progress::Progress;

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
fn total() {
    println!("{}年到{}年之间一共有{}个身份证号", YEAR.0, YEAR.1, generate::total());
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
    go(|idcard| {
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