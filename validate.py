"""
身份证号校验器
"""
import numpy as np
import re
import datetime
from code2address import code2address


def get_check_code(x: str):
    assert len(x) == 17
    w = '07 09 10 05 08 04 02 01 06 03 07 09 10 05 08 04 02'.split()
    w = [int(i) for i in w]
    x = list(map(int, x))
    v = np.dot(x, w)
    check_code = (12 - v % 11) % 11
    return 'X' if check_code == 10 else str(check_code)


def valiate_check_code(x: str):
    check_code = get_check_code(x[:-1])
    if check_code != x[-1]:
        return f'校验位错误{v % 11}'


def validate_pattern(x: str):
    return re.match('\\d17[\\dX]', x)


def day_of(y, m):
    def is_leap(y):
        if y % 100 == 0:
            return y % 400 == 0
        else:
            return y % 4 == 0

    if m in (1, 3, 5, 7, 8, 10, 12):
        should = 31
    elif m == 2:
        should = 29 if is_leap(y) else 28
    else:
        should = 30
    return should


def validate_year_month_day(v: str):
    # 校验年月日
    y = int(v[6:10])
    m = int(v[10:12])
    d = int(v[12:14])
    now = datetime.datetime.now()

    if (y, m, d) > (now.year, now.month, now.day):
        return "不能在未来出生"
    if y < 1900:
        return "不可能如此长寿"
    if not 12 >= m >= 1:
        return "月份错误"

    if not day_of(y, m) >= d >= 1:
        return "日期错误"


def validate_address(x: str):
    # 校验地址
    address = x[:6]
    if address not in code2address:
        return '未知的地址'


def validate(x: str):
    for f in (validate_pattern,
              validate_year_month_day,
              valiate_check_code,
              validate_address):
        e = f(x)
        if e:
            return e


if __name__ == '__main__':
    x = 'xxx'
    print(validate(x))
