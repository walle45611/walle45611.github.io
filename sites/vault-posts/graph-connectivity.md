---
title: "Graph Connectivity"
slug: graph-connectivity
topic_section: algorithms
description: "Graph Connectivity的重點整理。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Graph Connectivity.md"
---

https://web.ntnu.edu.tw/~algo/ConnectedComponent.html
https://nthu-cp.github.io/NTHU-CPP/graph/introduction_to_AP_bridge.html

- 相關定義
    
    - **1. Connected Component（連通分量）**：在無向圖的「極大連通」子圖。可切出更小的連通子圖，但那些不再是連通分量；孤立點也是一個連通分量。
        
    - **2. Biconnected Component 與 Articulation Point**：雙連通分量內不含關節點；不同分量可能重疊，重疊點正是原圖的關節點。
        
    - **3. Bridge-Connected Component**：無向圖中不含任何橋的連通分量；任兩點至少有兩條相異路徑，相當於存在環。
        
    - **4. SCC vs Weakly Connected Component（有向圖）**：SCC 內任兩點皆有雙向可達；弱連通分量只要求任兩點至少單向可達。
        
    - **5. 收縮 SCC 的好處**：把每個 SCC 縮為單點，可消除所有環，得到 DAG；在 DAG 上設計與實作演算法更容易、更高效。

## Tarjan's Algorithm to find Bridge

```cpp
#include <bits/stdc++.h>
using namespace std;

// CLRS 風格著色：WHITE→GRAY→BLACK
enum Color { WHITE, GRAY, BLACK };

struct Tarjan {
  int n, timer = 0;
  vector<vector<int>> adj;      // adjacency list
  vector<Color> color;          // DFS color
  vector<int> d, low, pi;       // d=discover time, low=low-link, pi=parent
  vector<bool> isAP;            // articulation point flags
  vector<pair<int,int>> bridges;// list of bridges (u,v)

  Tarjan(int n)
    : n(n), adj(n+1), color(n+1, WHITE),
      d(n+1, 0), low(n+1, 0), pi(n+1, -1), isAP(n+1, false) {}

  void addEdge(int u, int v){
    adj[u].push_back(v);
    adj[v].push_back(u);
  }

  void dfs(int u){
    color[u] = GRAY;
    d[u] = low[u] = ++timer;    // 進點：發現時間與 low 初值相同
    int children = 0;

    for(int v : adj[u]){
      if(color[v] == WHITE){    // 樹邊 (tree edge)
        pi[v] = u;
        ++children;
        dfs(v);

        // 子樹回傳後，用 child's low 降低自己的 low
        low[u] = min(low[u], low[v]);

        bool is_root = (pi[u] == -1);
        // 關節點：root 需有至少兩個子樹
        if(is_root && children >= 2) isAP[u] = true;
        // 關節點：非 root，若 child's low 回不到 u 之上
        if(!is_root && low[v] >= d[u]) isAP[u] = true;
        // 橋：child 無法經回邊到達 u 或更上層
        if(low[v] > d[u]) bridges.emplace_back(u, v);
      }
      else if(v != pi[u]){      // 回邊 (back edge) 到祖先
        // 已訪問且非父邊，代表能往上「勾」到 d[v]
        low[u] = min(low[u], d[v]);
      }
    }
    color[u] = BLACK;           // 離點
  }

  void run(){
    for(int u = 1; u <= n; ++u)
      if(color[u] == WHITE) dfs(u);
  }
};

int main(){
  // 範例圖：三角形 (1-2-3-1) + 尾巴 (2-4-5)
  Tarjan T(5);
  T.addEdge(1,2); T.addEdge(2,3); T.addEdge(3,1);
  T.addEdge(2,4); T.addEdge(4,5);

  T.run();

  cout << "u: d/low\n";
  for(int u=1; u<=5; ++u) cout << u << ": " << T.d[u] << "/" << T.low[u] << "\n";

  cout << "Articulation points:";
  for(int u=1; u<=5; ++u) if(T.isAP[u]) cout << " " << u;
  cout << "\nBridges:";
  for(auto &e: T.bridges) cout << " (" << e.first << "," << e.second << ")";
  cout << "\n";
}
```


- 名詞對應
    
    - $d[u]$：DFS discover time（發現次序）。
        
    - $low[u]$：從 $u$ 的子樹出發，沿樹邊並至多一條回邊可達到的最小 $d[\cdot]$。
        
    - $pi[u]$：父節點（parent）。
        
    - Color：`WHITE/GRAY/BLACK`（CLRS 著色）。
        
- 低鏈值（low-link）直覺
    
    - 不是距離，不是「更短路徑」。
        
    - 比的是時間戳：$low[u] = \min{ d[u],\ \min d[v]\ (\text{回邊}),\ \min low[\text{children}] }$。
        
    - 解讀：$u$ 的子樹能「往上勾回去」的最淺層發現時間。
        
- 為何 `v != pi[u]` 表示 back edge
    
    - 無向圖 DFS 只有「樹邊 + 回邊」，沒有 cross/forward。
        
    - 已訪問且非父的鄰居 $v$ 必為祖先，故 $(u,v)$ 是 back edge。
        
    - 更新規則：`low[u] = min(low[u], d[v])`。
        
- 判斷條件
    
    - 橋 $(u,v)$（令 $u$ 是 $v$ 的父）：若 $low[v] > d[u]$。
        
    - 關節點 $u$：
        
        - 根：子樹數 $\ge 2$。
            
        - 非根：存在子節點 $v$ 使 $low[v] \ge d[u]$。
            
- 你剛剛的疑問與釐清
    
    - Q：`else if (v != pi[u])` 為何可當 back edge？
        
        - A：因為無向圖無 cross/forward；已訪問且非父即祖先，故為 back edge。
            
    - Q：$low$ 是否代表「更短路徑」？
        
        - A：否。$low$ 只比較發現時間戳，不是距離或權重。
            
    - Q：圖上是鄰居，但 DFS tree 上可能是祖先？
        
        - A：是。原圖鄰接不代表 DFS tree 同層；非父且已訪問的鄰居就是祖先。
            
- 範例（本程式）
    
    - 圖：三角形 $(1,2,3)$ + 尾巴 $(2,4,5)$。
        
    - 結果：Articulation points = ${2,4}$；Bridges = ${(2,4),(4,5)}$。
        
    - 直覺：三角形內有回邊能彼此「補救」；尾巴沒有回邊，切就斷。
        
- 複雜度與實作細節
    
    - 時間 $O(V+E)$，空間 $O(V+E)$。
        
    - 多源圖要從所有 `WHITE` 節點啟動 DFS。
        
    - 想更貼近「回到當前遞迴棧上的祖先」可寫：`if (color[v]==GRAY && v!=pi[u])`。
        
    - 無向圖常見 bug：忘了略過父邊，導致把雙向邊誤當回邊。
        
- 與 Tarjan SCC（有向圖）對照（備忘）
    
    - 有向圖版本用 `index/lowlink + stack + inStack`。
        
    - 回邊條件變成「鄰居在棧中」。
        
    - 觸發 SCC 輸出條件：`lowlink[u] == index[u]`。
        
- 口訣
    
    - 橋：$low[\text{child}] > d[\text{parent}]$。
        
    - 關節：根看「子樹數」，非根看「child 的 low 是否上不去」。

### 手寫計算
> **手寫計算**
> ![01-手寫計算](/vault-assets/b620255e9261bb519bac.png)

## Tarjan’s Algorithm — SCC（有向圖）

- 目標：在有向圖分解 **Strongly Connected Components**，時間 $O(V+E)$。
    
- 狀態
    
    - $d[u]$：發現次序（index）。
        
    - $low[u]$：自 $u$ 出發，沿樹邊並經任意多條**指向堆疊中頂層節點**的回邊，能到的最小 $d[\cdot]$。
        
    - `stack<int> st`、`onStack[u]`：維持當前 DFS 路徑集合。
        
- 定義（$low$ 的更新來源）  
$$
    low[u] = \min\Big(
    d[u],
    \min_{u\to v\ \text{且}\ v\ \text{未訪問}} low[v],
    \min_{u\to v\ \text{且}\ onStack[v]} d[v]
    \Big)
$$
    
- 規則
    
    1. 進點：`d[u]=low[u]=++time; push(u); onStack[u]=true`
        
    2. 掃邊 $u \to v$
        
        - 若 `d[v]==0`：`dfs(v)` 後 `low[u]=min(low[u], low[v])`
            
        - 否則若 `onStack[v]`：`low[u]=min(low[u], d[v])`
            
    3. 若 `low[u]==d[u]`：`u` 為一個 SCC 的根，從棧頂彈出直到 `u`，形成一個 SCC
        
- 為什麼 `low[u]==d[u]` 觸發輸出
    
    - 若 $low[u]<d[u]$，代表子樹能「勾回」更早棧上祖先，`u` 屬於更大的連通塊，不能切。
        
    - $low[u]==d[u]$ 表示再也回不到更早棧上點；以 `u` 為界閉合，棧上從頂到 `u` 正是一個極大 SCC。
        
- 常見陷阱
    
    - 少了 `onStack[v]` 判斷會把 forward/cross edge 誤算進 `low`。
        
    - 多連通分量要從所有 `d[u]==0` 的點啟動 `dfs`。
        
    - 索引一致性（0/1-based）與容器大小。
        

```cpp
#include <bits/stdc++.h>
using namespace std;

enum Color { WHITE, GRAY, BLACK };

struct TarjanSCC {
  int n, timer = 0;
  vector<vector<int>> adj;      // 有向圖鄰接表
  vector<Color> color;          // CLRS 著色：WHITE/GRAY/BLACK
  vector<int> d, low;           // d = 發現次序, low = low-link
  vector<bool> onStack;         // 是否在堆疊（表示仍在當前 DFS 路徑上）
  stack<int> st;
  vector<vector<int>> sccs;     // 輸出：每個 SCC 的節點集合

  TarjanSCC(int n)
    : n(n), adj(n+1), color(n+1, WHITE),
      d(n+1, 0), low(n+1, 0), onStack(n+1, false) {}

  void addEdge(int u, int v) { adj[u].push_back(v); } // 只加 u->v

  void dfs(int u){
    color[u] = GRAY;
    d[u] = low[u] = ++timer;    // 進點：d 與 low 同步為新時間戳
    st.push(u); onStack[u] = true;

    for(int v : adj[u]){
      if(d[v] == 0){            // 樹邊
        dfs(v);
        low[u] = min(low[u], low[v]);     // 子樹回傳降低 low[u]
      } else if(onStack[v]) {   // 僅當 v 還在棧上，才視為「真回邊」
        low[u] = min(low[u], d[v]);
      }
      // 若 v 已出棧（BLACK & onStack[v]==false），不影響 low[u]
    }

    // u 為一個 SCC 的根：收割一個 SCC
    if(low[u] == d[u]){
      vector<int> comp;
      while(true){
        int x = st.top(); st.pop();
        onStack[x] = false;
        comp.push_back(x);
        if(x == u) break;
      }
      sccs.push_back(move(comp));
    }

    color[u] = BLACK;
  }

  void run(){
    for(int u = 1; u <= n; ++u)
      if(d[u] == 0) dfs(u);
  }
};

int main(){
  // 範例：1->2->3->1 為一個 SCC；3->4->5 為鏈，(4)、(5) 各自成 SCC
  TarjanSCC G(5);
  G.addEdge(1,2); G.addEdge(2,3); G.addEdge(3,1);
  G.addEdge(3,4); G.addEdge(4,5);

  G.run();

  cout << "u: d/low\n";
  for(int u=1; u<=5; ++u) cout << u << ": " << G.d[u] << "/" << G.low[u] << "\n";

  cout << "SCCs:\n";
  for(auto &c: G.sccs){
    sort(c.begin(), c.end());   // 只為了穩定輸出觀察
    for(int x: c) cout << x << " ";
    cout << "\n";
  }
}
```

- 複雜度
    
    - 時間：$O(V+E)$；空間：$O(V+E)$（含堆疊與鄰接表）。
        
- 速記
    
    - 回邊條件（有向圖，SCC 版）：`onStack[v]`。
        
    - 觸發輸出：`low[u]==d[u]`。
