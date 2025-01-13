---
title: virtual-judge 刷題紀錄 08/15
tags:
  - virtual-judge
  - Algorithm
  - dfs
categories: 刷題
abbrlink: ceb
date: 2024-08-15 18:31:40
---

這篇文章紀錄了 Virtual Judge 在 08/15 的刷題心得，挑戰了題目 HDU-1232，並使用 深度優先搜尋 (DFS) 解決問題。本題的核心在於判斷一個圖是否為連通圖，並計算需要新增的邊數來使其變成完全連通的圖。

## 解題程式

* [題目連結 HDU-1232](https://vjudge.net/problem/HDU-1232)

* 我的想法是 dfs 走過的所有點的連通分量加總就可以知道是不是連通圖了，再減去 1 是為了題目所要的描述，他需要加上幾條邊才能使得變成連通圖 (Connected graph)，因為如果不是連通圖 `connect` > 1。

<!--more-->

```c++
#include <bits/stdc++.h>

using namespace std;

void dfs(int node, vector<vector<int>>& graph, vector<bool> &visited){
  visited[node] = true;
  for(int n: graph[node]){
    if(!visited[n]){
      dfs(n,graph,visited);
    }
  }
}

int findMinRoads(int n,vector<pair<int,int>> & roads){
  if(n==0) return 0;

  vector<vector<int>> graph(n+1);
  for(const auto& road : roads){
    int u = road.first;
    int v = road.second;
    graph[u].push_back(v);
    graph[v].push_back(u);
  }

  vector<bool> visited(n+1,false);
  int connect = 0 ;

  for(int i=1;i<=n;++i){
    if(!visited[i]){
      dfs(i,graph,visited);
      ++connect;
    }
  }

  return connect-1;
}


int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(nullptr);

  while(true){
    int n,m;
    cin >> n >> m;
    if(n== 0) break;
    vector<pair<int,int>> roads(m);

    for(int i=0;i<m;++i){
      int u,v;
      cin >> u >> v;
      roads[i] = {u,v};
    }

    cout << findMinRoads(n,roads) << endl;

  }

  return 0;
}
```
