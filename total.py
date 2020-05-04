from validate import validate, day_of, get_check_code
from code2address import code2address
from tqdm import tqdm

YEAR = (1960, 2020)


def id_number_total():
    """
    估算身份证号一共有多少个
    """
    year_month_day = (YEAR[1] - YEAR[0]) * 365  # 一百年时间
    address = len(code2address)
    s = year_month_day * address * 1000  # 1000表示最后三位序列号
    return s


def iterate_idnumber():
    """
    迭代身份证号
    """
    for address in code2address.keys():
        for year in range(YEAR[0], YEAR[1]):
            for month in range(1, 12):
                for day in range(1, day_of(year, month) + 1):
                    for serie in range(1000):
                        s = f"{address}{year:04d}{month:02d}{day:02d}{serie:03d}"
                        check_code = get_check_code(s)
                        s += check_code
                        yield s


total = id_number_total()  # 69685800000,696亿个身份证号
print(total)
"""
全部迭代完需要300小时，只能使用Rust进行探索了
"""
for i in tqdm(iterate_idnumber(), total=total):
    pass
