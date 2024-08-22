---
title: virtual-judge 刷題紀錄 08/21
date: 2024-08-20 18:32:04
tags: [virtual-judge, Algorithm, binary-search]
categories: 刷題
---

![1. blog image](https://imgur.com/LLJNmPR.png)

* [P2249 題目連結](https://www.luogu.com.cn/problem/P2249)
* 這題解題思路主要是使用二分搜尋法來解決問題。透過不斷縮小範圍來尋找目標數字的位置，當找到相同的數字時，即可將其所在的中間位置輸出。

<!--more-->

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
