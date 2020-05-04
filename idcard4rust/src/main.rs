#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate chrono;

use crate::generate::go;
use crate::prime::{linearPrime, bruteforce, smallNumberMillerRabin};
use crate::progress::Progress;
use chrono::{Date, Utc, TimeZone};

mod progress;
mod validate;
mod code2address;
mod test;
mod generate;
mod prime;
mod find_prime_idcard;


fn iterate_idcard() {
    /*
Python是5万个iter每秒
Rust才是10万个iter每秒
Release 是110万个iter每秒需要16h才能迭代完
*/

    fn print(idcard: &String) {
        println!("{}", idcard);
    }
    #[allow(unused_variables)]
    fn nothing(idcard: &String) {}
    let beg = Date::from(Utc.ymd(1993, 01, 01));
    let end = Date::from(Utc.ymd(2020, 01, 01));
    go(beg, end, nothing);
}

fn testPrimeSpeed() {
    let n = 5000000usize;
    let a = linearPrime(n);//37s
    println!();
    let b = bruteforce(n);//34s
    println!();
    println!("now will compare result");
    let mut bar = Progress::new(n, String::from("compare"));
    for i in 2..n {
        if a[i] != b[i] {
            println!("error {}", i);
            unreachable!();
        }
        bar.update(1);
    }
}

fn testSmallMillerRabin() {
    //测试小数据范围的miller-rabin算法
    //实践证明小数据集下的miller-rabin算法只用很少几个数字进行检测即可。
    let n = 10000_0000;
    let a = bruteforce(n);
    println!();
    println!("已经生成");
    let mut bar = Progress::new(n, "compare".to_string());
    for i in 2..n {
        let his = smallNumberMillerRabin(i as i64);
        if his != a[i] {
            println!("\nerror {}", i);
        }
        bar.update(1);
    }
}

fn findPrimeIdcard() {
    find_prime_idcard::go();
}

fn main() {
    // testPrimeSpeed();
    // testSmallMillerRabin();
    // findPrimeIdcard();
    iterate_idcard();
}
