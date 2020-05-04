use crate::code2address::getBook;

/**
身份证号校验器
*/

fn validate_address(idcard: &String) -> String {
    let book = getBook();
    let code = &idcard[0..6];
    if book.contains_key(code) {
        "".to_string()
    } else {
        format!("没有省份{}", code)
    }
}

fn isLeapYear(year: i32) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

pub fn getDaysOfMonth(year: i32, month: i32) -> i32 {
    if month == 2 {
        return if isLeapYear(year) { 29 } else { 28 };
    }
    if [1, 3, 5, 7, 8, 10, 12].contains(&month) {
        31
    } else {
        30
    }
}

fn validate_year_month_day(idcard: &String) -> String {
    //校验年月日
    fn parseNumber(s: &str) -> i32 {
        return s.parse::<i32>().unwrap();
    }
    let year: i32 = parseNumber(&idcard[6..10]);
    let month = parseNumber(&idcard[10..12]);
    let day = parseNumber(&idcard[12..14]);
    if year < 1920 || year > 2040 {
        return format!("年份错误{}", year);
    }
    if !(month >= 1 && month <= 12) {
        return format!("月份错误{}", month);
    }
    let days = getDaysOfMonth(year, month);
    if !(day >= 1 && day <= days) {
        return format!("日期错误{},总天数{}", day, days);
    }
    "".to_string()
}

fn idcard2int(idcard: &String) -> Vec<i32> {
    let x: Vec<i32> = idcard.chars().map(|x| {
        if x == 'X' { 10 } else { x as i32 - '0' as i32 }
    }).collect();
    x
}

const WEIGHT_STRING: &str = "07 09 10 05 08 04 02 01 06 03 07 09 10 05 08 04 02 01";

#[allow(non_upper_case_globals)]
pub fn getCheckCode(idcard: &String) -> char {
    //获取校验码
    let x = idcard2int(&idcard);
    static mut weight: Vec<i32> = Vec::new();
    //初始化weight
    unsafe {
        if weight.len() == 0 {
            weight = WEIGHT_STRING.split_whitespace().map(|x| {
                if x == "X" {
                    10
                } else {
                    x.parse::<i32>().unwrap()
                }
            }).collect();
            weight = weight[0..weight.len() - 1].to_owned();
        }
    }
    unsafe {
        if idcard.len() != weight.len() {
            panic!("idcard length is different with weight {} {}", idcard.len(), weight.len());
        }
        let mut s = 0u128;
        for i in 0..weight.len() {
            s += (x[i] * weight[i]) as u128;
        }
        let checkCode = (12 - s % 11) % 11;
        if checkCode == 10 { 'X' } else { ('0' as u128 + checkCode) as u8 as char }
    }
}

#[allow(non_upper_case_globals)]
fn validate_checkcode(idcard: &String) -> String {
    static mut weight: Vec<i32> = Vec::new();
    unsafe {
        if weight.len() == 0 {
            weight = WEIGHT_STRING.split_whitespace().map(|x| {
                if x == "X" {
                    return 10;
                }
                x.parse::<i32>().unwrap()
            }).collect();
        }
    }
    let nums: Vec<i32> = idcard2int(&idcard);
    if nums.len() != unsafe { weight.len() } {
        panic!("length should be equal")
    }
    let mut s = 0i128;
    unsafe {
        for i in 0..weight.len() {
            s += (weight[i] * nums[i]) as i128;
        }
    }

    if s % 11 == 1 {
        return "".to_string();
    }
    "checkcode error".to_string()
}

pub fn validate(idcard: &String) -> String {
    let validators = [validate_address, validate_year_month_day, validate_checkcode, ];
    for i in validators.iter() {
        let res: String = i(&idcard);
        if !res.is_empty() {
            return res;
        }
    }
    "".to_string()
}