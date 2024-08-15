---
title: 刷題紀錄 leetcode-3106
date: 2024-08-15 12:20:29
tags: [leetcode, Algorithm, string]
categories: 刷題
---

## 解題程式

* [題目連結](https://leetcode.com/problems/lexicographically-smallest-string-after-operations-with-constraint/description/)

* 我的想法是，既然他說，要有 `cyclic` 這個字眼了所以我當然需要判斷哪個距離比較小 `minimum distance` 題目也有說到，所以我就要把 `abs(s[i] - new_char)`、`26 - abs(s[i] - new_char)`兩個方向的距離都算出來，在判斷有沒有 `小於等於`，然後就把 `k` 每次更新就是把 `k-=dist`，並且把 `t` 也更新，最後回傳 `t` 就好了。

<!--more-->

* code

    ```C++
    class Solution {
    public:
        string getSmallestString(string s, int k) {
            string t = s;
            int n = s.length();
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < 26; j++) {
                    char new_char = 'a' + j;
                    int dist = min(abs(s[i] - new_char), 26 - abs(s[i] - new_char));
                    if (dist <= k) {
                        t[i] = new_char;
                        k -= dist;
                        break;
                    }
                }
            }
            return t;
        }
    };
    ```
