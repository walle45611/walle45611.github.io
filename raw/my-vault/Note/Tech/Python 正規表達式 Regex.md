## 線上測試工具

> [!info] regex101: build, test, and debug regex  
> Regular expression tester with syntax highlighting, explanation, cheat sheet for PHP/PCRE, Python, GO, JavaScript, Java, C\#/.  
> [https://regex101.com/](https://regex101.com/)  

# 基本的Regex

## 集合

|   |   |
|---|---|
|[ ]|用來比對集合|
|[0-9]|用來比對數字0-9|
|[a-z]|用來比對小寫字母a-z|
|[A-Z]|用來比對大寫字母A-Z|
|[^0-9]|用來比對非數字|
|[^a-z]|用來比對非小寫字母|
|[^A-Z]|用來比對非大寫字母|

```Shell

Expression : [Pp]ython3 第一個字有包括P或p的字串，其他的均不行
Text:
Python3 
python3
Ppyton3

Expression : [0-9] 所有的數字，也可以解釋0-9的數字就匹配，其他的均不行
Text:
1234234

Expression : [a-zA-z] 所有的英文字母都可以匹配，其他的均不行
Text:
asdfsdfa


Expression : [\[\]\(\)]* 特殊符號匹配，簡單來說加個跳脫字元\就好了
Text:
[]
----
()

Expression : [^0-9] 除了數字以外的都能匹配，簡單來說就是加了^反向了
Text:
12341234
12341234
12347980
12342343
asdfsadf
asdfsadfsa
sadfasdf
```

## 更快的方法

|   |   |
|---|---|
|\d|[0-9]|
|\D|[^0-9]|
|\s|whitespace|
|\S|non whitespace|
|\w|任何字母數字|
|\W|非任何(字母和數字)|

```Shell
Expression : [\w] 會將所有的字母或是數字都允許
Text:
//\
=-
<>()
master
987654321
CODE

Expression : [\d] 會將所有的數字都可以
Text:
asdfasdf1234
asdf
1324

Expression : [[Assets/Other/02fig01.gif]] 這樣可以匹配到空白鍵
Example: [code\s]
Text:
JIAONANG-MASTER
CO DEJIAONANG!
code jiaonang
code
code jiaonang
code www

Expression : [\b] 字串邊界
Example : [\bcode\b]
Text:
CODEINFO
codeasd/
codejiaonangA$
code
code jiaonang
code-jiaonang
www.code

Expression : [\W] 將字母和數字以外的都允許，也就是\w的反向
Text:
￥master

Expression : [\D] 將非數字的都允許，也就是\d的反向
Text:
12341234
12341243
code 
code 1234
code 1324 code

Expression : [\S] 空白鍵以外的，也就是\s的反向
Text:
12341234
12341243
code 
code 1234
code 1324 code
```

## 開頭結尾符號

|   |   |
|---|---|
|^|在開頭匹配|
|$|在結尾匹配|

```Shell
Expression : ^ 在開頭匹配
Example : ^python
Text:
python 1234 python

Expression : $ 在結尾匹配
Example : python$
Text:
python 1234 python

Example : ^python$
python
```

## 任意字符

|   |   |
|---|---|
|.|任意字|

```Shell
Expression : . 任意字符除了換行
Example: .ar
bar
aar
bbbar
```

## 或

```Python
Expression : | 直接看例子
Example: yes|no
yes
no
```

## Quantifiers

|   |   |
|---|---|
|?|單一字符出現1次或0次|
|*|出現0次或無限多次|
|+|至少出現1次|
|{n,m}|出現次數在n到m之間|

- ? **可選字符**
    
    ```Shell
    Expression : ? 直接看例子
    Example: favou?rite
    urite
    favouite
    favorite
    favourite
    favorite
    ```
    
- {n,m} **重复**
    
    ```Shell
    Expression : {number} 重複number次
    Example: \d{3}
    Text:
    1234
    1234
    132
    565
    
    Expression : {number}
    Example: \d{3,4}? 禁止貪婪模式 會先匹配先滿三個字的，之後的就不匹配了
    Text:
    123
    1234
    ```
    

## Boundary Matchers

|   |   |
|---|---|
|^|配對第一個位置|
|$|配對最後一個位置|
|\b|配對字符的邊界|
|\B|不配對字符的邊界|

## Group

|   |   |
|---|---|
|( )|括弧裡面的就是一個Group|

```Shell
Expression : "(\d+), Date: (.+)" 簡單來說()裡面包的東西就是群組
```

## example

```Python
^((https?|ftp|file):\/\/)?([\da-z\.\-]+)\.([a-z\.]{2,6})([[Assets/Other/02fig01.gif]\w\.\-]*)*\/?$
https://www.bilibili.com/danlaoshi/666/
```

---

# Regex advanced

## 簡單的python匹配

```Python
import re

pattern1 = "cat"
pattern2 = "bird"
string = "dog runs to cat"

print(pattern1 in string)
print(pattern2 in string)
'''
Output:

True
False
'''
```

## 常用的方法

- match
    
    ```Python
    import re
    
    pattern = re.compile(r'<HTML>')
    print(pattern.match("<HTML><head>"))
    另一種方法
    re.match(r'<HTML>',"<HTML><head>")
    '''
    Output:
    
    <re.Match object; span=(0, 6), match='<HTML>'>
    '''
    ```
    
- Search
    
    ```Python
    import re
    
    pattern = re.compile(r'<HTML>')
    print(pattern.search("a<HTML><head>"))
    '''
    Output:
    
    <re.Match object; span=(1, 7), match='<HTML>'>
    '''
    ```
    
- findall
    
    ```Python
    import re
    pattern = re.compile(r'<HTML>')
    results = pattern.findall("<HTML>_123_<HTML>")
    print(results)
    '''
    Output:
    
    ['<HTML>', '<HTML>']
    '''
    ```
    
- split
    
    ```Python
    import re
    pattern = re.compile(r'\W')
    results = pattern.split("Beautiful is better than-ugly")
    print(results)
    '''
    Output:
    
    ['Beautiful', 'is', 'better', 'than', 'ugly']
    '''
    ```
    
- sub
    
    ```Python
    import re
    pattern = re.compile(r'[0-9]+')
    results = pattern.sub("-","apple0-Lemon1-youtube3-")
    print(results)
    '''
    Output:
    
    apple--Lemon--youtube--
    '''
    ```
    

## Group and **Named Capturing Group**

- Group
    
    ```Python
    import re
    
    match = re.match(r"(\w+)\s(\w+)","Isacc Newton, physicist")
    print(match.group(0))
    print(match.group(1))
    print(match.group(2))
    print(match.group(1, 2))
    
    pattern = re.compile(r"(\w+)\s(\w+)")
    print(pattern.findall("Isacc Newton, physicist"))
    
    '''
    Output:
    
    Isacc Newton
    Newton
    Newton
    ('Isacc ', 'Newton')
    [('Isacc', 'Newton')]
    '''
    ```
    
- Named Capturing Group
    
    ```Python
    import re
    
    string='''
    ID: 021523, Date: Feb/12/2017
    '''
    
    match = re.search(r"(?P<id>\d+), Date: (?P<date>.+)",string)
    print(match.group())
    print(match.group('id'))
    print(match.group('date'))
    
    '''
    Output:
    
    021523, Date: Feb/12/2017
    021523
    Feb/12/2017
    '''
    ```
    

## **Backreference** 

- basic
    
    ```Python
    import re
    match = re.search(r'(hello) \1 \S+', 'This is a hello hello world!')
    print(match.group())
    '''
    Output:
    
    hello hello world!
    '''
    ```
    
- Backreference and Group change
    
    ```Python
    import re
    
    pattern = re.compile(r"(\d+)-(\w+)")
    pattern.sub(r"\2-\1","1-a\n20-bear\n34-afcr")
    
    '''
    Output:
    
    'a-1\nbear-20\nafcr-34'
    '''
    ```
    
- yes pattern | no - pattern
    
    ```Python
    import re
    
    pattern = re.compile(r"(\d\d-)?(\w{3,4})(?(1)(-\d\d))")
    
    print(pattern.match("34-erte-22"))
    print(pattern.search("erte"))
    
    '''
    Output:
    
    <re.Match object; span=(0, 10), match='34-erte-22'>
    <re.Match object; span=(0, 4), match='erte'>
    '''
    ```
    

  

## Look around

1. Positve Lookahead
    
    ```Python
    pattern = re.compile(r"\w+(?=,|\.)")
    pattern.findall("They were three: Felix, Victor, and Carlos.")
    
    '''
    Output:
    
    ['Felix', 'Victor', 'Carlos']
    '''
    ```
    
2. Negative Lookahead
    
    ```Python
    pattern = re.compile(r"John(?!\sSmith)")
    result = pattern.finditer("I would rather go out with John McLane than with John Smith or John Bon Jovi")
    
    for i in result:
        print(i.start(),i.end())
    '''
    Output:
    
    27 31
    63 67
    '''
    ```
    
3. Positive Lookbehind
    
    ```Python
    因為跟上面基本相同所以只說語法
    ?<=
    ```
    
4. Negative Lookbehind
    
    ```Python
    因為跟上面基本相同所以只說語法
    ?<!
    ```
    

- 總結
    
    1. ==Positve Lookahead X(?=Y) Y的前面必須是X==
    2. ==Negative Lookahead X(!=Y) Y的前面必須不是X==
    3. ==Positive Lookbehind (?<=Y)X Y的後面必須為X==
    4. ==Negative Lookbehind (?!=Y)X Y的後面必須不是X==
    
    推薦網站
    
    > [!info] RegExp 應用： lookahead , lookbehind  
    > 正規表示法一直都是我很推大家學習的東西，在字串處理上真的有很大很大的幫助以及好處，前幾天朋友工作上需要用到正規表示法處理字串，目的是：「在一個字串中找出連續數字 6~8 個」，這邊要注意的是，連續九個的話是不要的。 例如： 12345 XD Hi12345678ab666666cd987654321 要找出： 12345678 和 666666；但不可以找出 987654321 中的 98765432 或者 87654321 於是我第一個想到的東西就是 Lookahead 和 lookbehind。 先來看一下如果直接使用 \d{6,8} 會取出什麼： Visual Regex Tester 可以看到，直接使用 \d{6,8} 是會連後方的 987654321 取出來。 最後我給的的解法是：(?  
    > [http://darkk6.blogspot.com/2017/03/regexp-lookahead-lookbehind.html](http://darkk6.blogspot.com/2017/03/regexp-lookahead-lookbehind.html)