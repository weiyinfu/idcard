use crate::generate;
use crate::prime;

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
    generate::go(iterate)
}