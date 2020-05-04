关于身份证号的一切知识

# 身份证号有可能是素数吗？  
https://www.zhihu.com/question/365454876  

素数定理描述了素数在自然数中分布的近似分布情况。素数定理指出，给出随着数字的增大，素数的密度逐渐降低。f(x)表示不大于x的质数的个数
f(x)=x/ln(x)

# 给定身份证号特征，求身份证号
```plain
n/5是素数。
(n-1)/2是素数。
(n-2)/3是素数。
(n-3)/4是素数。
n是我本人的身份证号码。
```

# Rust和Python测速
尝试生成全部的身份证号。直观感觉：Rust Debug版是Python的两倍，Rust的Release版是Debug版的10倍。可以直接说，Rust比Python快20倍。

|方法|时间|
|---|---|
|Python|5万|
|Rust|10万|
|Rust Release|100万|