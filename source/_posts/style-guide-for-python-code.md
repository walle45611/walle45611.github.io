---
title: Python 程式碼風格指南
tags:
  - Python
categories: IT 技術
abbrlink: 815d
date: 2024-09-10 14:38:19
---

## 前情提要

PEP8 是在撰寫 Python 中的一些寫作規範標準，也就是說不遵照也不會出錯誤，那我也很常在想到底怎麼樣寫才是對的，然後看書看一看然後想說我都沒有看過 Python PEP8 相關的規定，我就突然跑去官方網站開始看，因為我想說看了也會對之後要寫專案的時候比較有幫助，所以就順便寫下了這邊篇 Blog。

<!--more-->

![圖 1. : Blog Image](https://imgur.com/DDM5EXN.png)

## PEP 257

在官網中有寫道 [PEP 257](https://peps.python.org/pep-0257/) 這項規定，這項規定就是在說，如果要寫 (Docstring) 也就是文件註解的時候要使用以下格式 Docstring 是模組、函數、類別或方法定義中的第一個字串常量。這些 docstring 會成為該對象的 __doc__ 屬性。所有模組通常應該有 docstring，模組中匯出的所有函數和類別也應該有 docstring。

### Docstring 格式

* 單行的 docstring : 在簡單的情況下，應該只會有一行，但是必須使用三個雙引號包括，即使一行也是要使用三個雙引號

    ```python
    def kos_root():
        """Return the pathname of the KOS root directory."""
        global _kos_root
        if _kos_root: return _kos_root
        ...
    ```

* 多行的 docstring : 由摘要行和詳細描述組成，
摘要行後要有一個空行。

    ```python
    def complex(real=0.0, imag=0.0):
        """Form a complex number.

        Keyword arguments:
        real -- the real part (default 0.0)
        imag -- the imaginary part (default 0.0)
        """
        if imag == 0.0 and real == 0.0:
            return complex_zero
        ...
    ```

* 範例

    ```python
    def sum(x :int,y :int) -> int:
        """_summary_

        Args:
            x (int): _description_
            y (int): _description_

        Returns:
            int: _description_
        """
        return x + y

    print(sum(1,2))
    ```

    ![圖 2 : Docstring 範例](https://imgur.com/MJpNw89.png)

## PEP8

### 縮排 (Indentation)

* Indentation

  * 每一層縮排應該使用 4 個空格。

  * Python 不允許混用 Tab 和空格進行縮排，所以要避免混用。

* Continuation Lines 也就是說當一行程式碼過程的時候，應該要按照以下方式處理
  * 可以使用垂直對齊括號內的元素，稱為這個是 `implicit line joining`

    ```python
    foo = long_function_name(var_one, var_two,
                             var_three, var_four)
    ```

  * 可以使用 `Hanging Indent`，第一行不應包含參數，要進行縮排。

    ```python
    def long_function_name(
            var_one, var_two, var_three,
            var_four):
        print(var_one)
    ```

* 以下皆為錯誤的寫法

    ```python
    # 沒有正確垂直對齊或進一步縮排
    foo = long_function_name(var_one, var_two,
        var_three, var_four)

    def long_function_name(
        var_one, var_two, var_three,
        var_four):
        print(var_one)
    ```

* if-statement : 當 `if` 太過長的時候需要換行。
  
  * 可以直接寫成多行，但是要保持縮排一致

    ```python
    if (this_is_one_thing and
        that_is_another_thing):
        do_something()
    ```

  * 也可以使用註解

    ```python
    if (this_is_one_thing and
        that_is_another_thing):
        # 註解增加區分
        do_something()
    ```

  * 或是也可以縮排，進行區分

    ```python
    if (this_is_one_thing
        and that_is_another_thing):
    do_something()
    ```

* 多行結構的括號

  * 左括號可以對其最後一個字

    ```python
    my_list = [
        1, 2, 3,
        4, 5, 6,
        ]
    ```

  * 或是可以與第一個字對齊
  
    ```python
    my_list = [
        1, 2, 3,
        4, 5, 6,
    ]
    ```

> __Note__:  
> 空格是 __首選的縮排方式__。  
> 然而，__如果代碼中已經使用 Tab 進行縮排__，那麼應保持一致，繼續使用 Tab 縮排。  
> Python 不允許在縮排中混用 Tab 和空格。

### Maximum Line Length

我那時候看到才知道有這件事情哈哈哈，原來有嚴格限制每行的長度，在 PEP8 中有規定每行最多長度就是 79 個字元，那對於比較沒有語法限制的文本區塊例如 docstring 或是註解，長度限制應該字 72 個字元內，給定的理由是大部分 IDE 或是 Editor 差不多會在寬度 80 的時候出現換行，使得程式碼難以閱讀。

那麼在 Python 3.10 之前 `with` 是沒有 `implicit line joining` 的所以需要使用 `\` 連結，以下是範例

```python
with open('/path/to/input_file') as file_1, \
     open('/path/to/output_file', 'w') as file_2:
    file_2.write(file_1.read())

# 在 Python 3.12 及之後的版本，你可以這樣寫：
with (
    open('/path/to/input_file') as file_1,
    open('/path/to/output_file', 'w') as file_2
):
    file_2.write(file_1.read())
```

### 二元運算子是要放在換行後還是前？

* 錯誤

    ```python
    income = (gross_wages +
            taxable_interest +
            (dividends - qualified_dividends) -
            ira_deduction -
            student_loan_interest)
    ```

* 正確

    ```python
    income = (gross_wages
          + taxable_interest
          + (dividends - qualified_dividends)
          - ira_deduction
          - student_loan_interest)
    ```

### imports
  
* import 的順序應該按照
  * 標準函式庫
  * 相關的第三方函式庫
  * 本地的或是專用的函式庫
  * 每種模組都應該留一個空行

* 建議使用絕對路徑的方式 import

    ```python
    import mypkg.sibling
    from mypkg import sibling
    from mypkg.sibling import example
    ```

  * 如果是遇到很複雜的 import 情況，有的時候是可以使用相對的，這都是可以接受的

    ```python
    from . import sibling
    from .sibling import example
    ```

* 正確

    ```python
    import os
    import sys
    ```

    ```python
    from subprocess import Popen, PIPE
    ```

* 錯誤

    ```python
    import os, sys
    ```

### Whitespace in Expressions and Statements

* 這邊的重點就是不要在表達式裡面亂加空白，會讓人看的很不舒服。

* 正確

    ```python
    spam(ham[1], {eggs: 2})
    foo = (0,)
    if x == 4: print(x, y); x, y = y, x
    ```

* 錯誤

    ```python
    spam( ham[ 1 ], { eggs: 2 } )
    bar = (0, )
    if x == 4 : print(x , y) ; x , y = y , x
    ```

* 在切片中

  * 正確

    ```python
    ham[1:9], ham[1:9:3], ham[:9:3], ham[1::3], ham[1:9:]
    ham[lower:upper], ham[lower:upper:], ham[lower::step]
    ham[lower+offset : upper+offset]
    ham[: upper_fn(x) : step_fn(x)], ham[:: step_fn(x)]
    ham[lower + offset : upper + offset]
    ```

  * 錯誤

    ```python
    ham[lower + offset:upper + offset]
    ham[1: 9], ham[1 :9], ham[1:9 :3]
    ham[lower : : step]
    ham[ : upper]
    ```

### Python 命名規範

#### 一般命名

* __單個小寫字母__：`b`
* __單個大寫字母__：`B`
* __小寫__：`lowercase`
* __小寫帶下底線__：`lower_case_with_underscores`
* __大寫__：`UPPERCASE`
* __大寫帶下底線__：`UPPER_CASE_WITH_UNDERSCORES`
* __駝峰命名法__：`CapitalizedWords` 或 `CamelCase`
  * 注意：使用縮寫時，所有字母應大寫，例如 `HTTPServerError` 而不是 `HttpServerError`
* __混合大小寫__：`mixedCase`，與駝峰命名法不同的是首字母小寫

#### 特殊命名

* __單下底線開頭__：`_single_leading_underscore`，用於指示內部使用的 variable 或 method。
* __單下底線結尾__：`single_trailing_underscore_`，用來避免與 Python 關鍵字的衝突（例如 `class_`）。
* __雙下底線開頭__：`__double_leading_underscore`，用於啟動 Python 的名稱改編機制，通常用於避免屬性名稱衝突。
* __雙下底線前後__：`__double_leading_and_trailing_underscore__`，稱為「魔術方法」或「魔術屬性」，這些名稱是 Python 的特殊名稱，如 `__init__`、`__file__`。

#### Class、Function 與 variable 命名

* __class__：使用 `CapitalizedWords` 命名規範。
* __method 和 variable__：使用小寫，單詞之間用下底線分隔（`lower_case_with_underscores`）。
* __constant__：使用全大寫，單詞用下底線分隔（`UPPER_CASE_WITH_UNDERSCORES`）。

#### 方法參數

* 實例方法的第一個參數應命名為 `self`。
* 類方法的第一個參數應命名為 `cls`。

## 參考

* [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
* [PEP 8 – Style Guide for Python Code](https://peps.python.org/pep-0008/)
