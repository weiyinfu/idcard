use crate::progress::Progress;

pub fn qpow(x: i64, mut y: i64, p: i64) -> i64 {
    //快速幂及模数，x^y%p
    let mut ans = 1i128;
    let mut m: i128 = x as i128;
    while y != 0 {
        if (y & 1) != 0 {
            ans *= m;
            ans %= p as i128;
        }
        m *= m;
        m %= p as i128;
        y >>= 1;
    }
    return ans as i64;
}

pub fn check(x: i64, y: i64, p: i64) -> bool {
//二次探测
    let tmp = qpow(x, y, p);
    if tmp != 1 && tmp != p - 1 {
        return false;
    }
    if tmp == p - 1 {
        return true;
    }
    if tmp == 1 && (y & 1) != 0 {
        return true;
    }
    return check(x, y >> 1, p);
}
/*
MillerRabin算法，使用2,7,61作为底数进行探测，可以解决10^7以内的问题

另一个问题：最少选择几个底数就能够检测1～n之间的全部质数
*/
pub fn smallNumberMillerRabin(x: i64) -> bool {
    //小数据范围的miller-rabin算法，不需要随机，只需要确定性算法即可
    if x <= 1 {
        panic!("too small number ")
    }
    if x == 2 || x == 7 || x == 61 ||
        (check(2, x - 1, x)
            && check(7, x - 1, x)
            && check(61, x - 1, x)) {
        return true;
    }
    return false;
}

pub fn linearPrime(n: usize) -> Vec<bool> {
    let mut a = Vec::<bool>::new();
    let mut primes: Vec<usize> = Vec::with_capacity((n as f64).sqrt() as usize);
    a.resize(n, true);
    let mut bar = Progress::new(n, String::from("线性筛"));
    for i in 2..n {
        if a[i] {
            primes.push(i);
        }
        let mut j = 0;
        while j < primes.len() && (primes[j] * i) < n {
            a[primes[j] * i] = false;
            if i % primes[j] == 0 {
                break;
            }
            j += 1;
        }
        bar.update(1);
    }
    return a;
}

pub fn bruteforce(n: usize) -> Vec<bool> {
    let mut b = Vec::<bool>::new();
    b.resize(n, true);
    let mut bar = Progress::new(n, String::from("暴力"));
    for i in 2..n {
        if b[i] {
            let mut j = i + i;
            while j < n {
                b[j] = false;
                j += i;
            }
        }
        bar.update(1);
    }
    return b;
}