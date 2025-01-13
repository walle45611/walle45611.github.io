---
title: virtual-judge 刷題紀錄 08/21
tags:
  - virtual-judge
  - Algorithm
  - binary-search
categories: 刷題
abbrlink: 3fea
date: 2024-08-20 18:32:04
---

## 前情提要

這篇文章是關於 Virtual Judge 刷題的紀錄，主要記錄了解題的過程和心得。在 08/21 的刷題中，挑選了 P2249 這道題目，並使用了經典的 二分搜尋法 (Binary Search) 來解題。文章會分享解題的邏輯、程式碼，以及一些關於該題的細節，幫助讀者了解如何高效地透過二分搜尋法解決問題。

題目要求我們在一個排序的數列中，尋找某些目標數字的首次出現位置。為了達到高效搜尋，程式利用二分搜尋法縮小範圍，並在找到匹配的數字時，記錄它的索引位置。接下來將分享這段程式碼的完整實現與運作流程！

<!--more-->

![1. Blog Image](https://imgur.com/LLJNmPR.png)

* [P2249 題目連結](https://www.luogu.com.cn/problem/P2249)
* 這題解題思路主要是使用二分搜尋法來解決問題。透過不斷縮小範圍來尋找目標數字的位置，當找到相同的數字時，即可將其所在的中間位置輸出。

```c++
#include <bits/stdc++.h>

using namespace std;

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(nullptr);

  long long n,m;
  cin >> n >> m;

  vector<int> a(n);
  for(auto i =0; i < n;++i){
    cin >> a[i];
  }

  bool first = true;

  while(m--){
    int q,left=0,right=n-1;
    int ans = -1;
    cin >> q;

    while(left <= right){
      int mid = left + (right - left) / 2 ;
      if (a[mid] >= q){
        if(a[mid] == q){
          ans = mid + 1;
        }
        right = mid - 1;
      }
      else {
        left = mid + 1;
      }
    }
    if(!first) cout << " ";
    cout << ans;
    first = false;
  }
  cout << "\n";
}
```
