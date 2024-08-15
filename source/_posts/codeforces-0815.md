---
title: codeforces 刷題紀錄 08/15
date: 2024-08-15 15:33:27
tags: [codeforces, Algorithm, dfs]
categories: 刷題
---

## 解題程式

* [題目連結 580C](https://codeforces.com/problemset/problem/580/C)

* 我的想法是，他說 Kefa 怕貓貓，所以就等於說他怕有幾隻貓貓以上他就不去了，然後輸入是有幾個點 `n` 跟 `m` 只成承受幾隻貓貓，再來就是 a1...an 個點哪些有貓貓 `1` 有 `0` 沒有，接下來就是無相圖的部分了，那就廢話不多說直接上 code。

<!--more-->

```c++
#include <bits/stdc++.h>

using namespace std;

int max_count = 0;

void dfs(int node,const vector<vector<int>> &path, vector<bool> &visited,vector<int> &a,int count,int m){
  visited[node] = true;

  if(a[node]) count++;
  else count = 0;

  if (count > m){
    visited[node] = false;
    return;
  }

  bool is_leaf = true;
  for(int n : path[node]){
    if(!visited[n]){
      is_leaf = false;
      dfs(n,path,visited,a,count,m);
    }
  }

  if(is_leaf){
    max_count++;
  }

  visited[node]=false;
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(nullptr);

  int n, m;
  cin >> n >> m;
  vector<int> a(n+1);

  for(int i=1;i<=n;++i){
    cin >> a[i];
  }

  vector<vector<int>> path(n+1);
  for(int i=0;i<n-1;++i){
    int x,y;
    cin >> x >> y;
    path[x].push_back(y);
    path[y].push_back(x);
  }

  vector<bool> visited(n+1,false);
  dfs(1,path,visited,a,0,m);

  cout << max_count << endl;

  return 0;
}
```

------

* [題目連結  1996B](https://codeforces.com/contest/1996/problem/B)
* 這題的想法基本上就是把每次把 `i`、`j` 以 `k` 倍的方式往上加，加上輸出。

```c++
#include <bits/stdc++.h>
using namespace std;

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(nullptr);

  int t;
  cin >> t;
  while(t--){
    int n, k;
    cin >> n >> k;
    cin.ignore();

    vector<vector<int>> m(n, vector<int>(n));
    for(int i=0;i<n;++i){
      string tmp;
      getline(cin,tmp);
      for(int j=0;j<n;j++){
        m[i][j]=(tmp[j] - '0');
      }
    }

    for(int i = 0; i < n; i += k){
      for(int j = 0; j < n; j += k){
        cout << m[i][j];
      }
      cout << "\n";
    }
  }

  return 0;
}
```