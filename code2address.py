"""
地址编码的repo：https://github.com/cn/GB2260
"""
import numpy as np

code2address = dict(np.reshape(open('idcard.txt').read().split(), (-1, 2)))
