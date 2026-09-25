---
title: "單一起點最短路徑"
slug: single-source-shortest-paths
topic_section: algorithms
description: "My vault 演算法筆記：單一起點最短路徑。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Single-Source Shortest Paths Problem.md"
---

## Single-Source Shortest Paths 演算法總覽

| 項目                    | DAG（Directed Acyclic Graph） | Dijkstra                                                 | Bellman–Ford |
| --------------------- | --------------------------- | -------------------------------------------------------- | ------------ |
| 解決問題                  | 單源最短路                       | 單源最短路                                                    | 單源最短路        |
| 策略                    | 拓樸排序（Topological sort）      | 貪婪（Greedy）                                               | 動態規劃（反覆鬆弛）   |
| 圖可有 cycle             | 否（必須為 DAG）                  | 是                                                        | 是            |
| 圖可有負權邊（negative edge） | 是                           | 否                                                        | 是            |
| 圖可有負環（negative cycle） | 否                           | 否                                                        | 否（若存在則回報）    |
| 時間複雜度（鄰接矩陣）           | $O(V+E)$                    | $O(V^2)$                                                 | $O(V^3)$     |
| 時間複雜度（鄰接串列／堆）         | $O(V+E)$                    | $O(E\log V)$（binary heap）；$O(V\log V+E)$（Fibonacci heap） | $O(VE)$      |
## Relax

若經由 $u$ 可以改進 $v$ 的估計，就更新距離與前驅：
    
```
RELAX(u, v):
  if v.d > u.d + w(u,v):
	  v.d = u.d + w(u,v)
	  v.π = u
```
    
- 伴隨性質：
	    
	1. 上界：$v.d \ge \delta(s,v)$，一旦等於就不再變。
		
	2. 三角：$\delta(s,v)\le \delta(s,u)+w(u,v)$。
		
	3. 收斂：若 $u.d=\delta(s,u)$，鬆弛 $(u,v)$ 後得 $v.d=\delta(s,v)$。
        
## DAG shortest-paths algorithm

- **問題與條件**：<mark>單源最短路，圖為 DAG。允許負邊；無（負）環。輸入 $G=(V,E),\ w:E\to\mathbb{R},\ s\in V$。</mark>
    
- **核心觀念**：拓樸序一次走完，對每條邊恰鬆弛一次，總時間 $O(V+E)$。
	
- **DAG shortest-paths（列表版流程）**
		
    - 取得拓樸序列 $L$（$O(V+E)$）。
        
    - 依序掃 $L$ 中每個 $u$。
        
    - 對 $u$ 的每條外出邊 $(u,v)$ 呼叫 `RELAX(u,v)`。
- 演算法
	    
	- **初始化**
	    
		```text
		for all v ∈ V: v.d = ∞, v.π = NIL
		s.d = 0
		```
	    
	- **演算法本體**
	    
	    - 取得拓樸序列 $L=\text{TOPOLOGICAL SORT}(G)$。
	        
	    - 初始化單源。
	        
	    - 依序掃 $L$ 中每個 $u$；對每條外出邊 $(u,v)$ 執行 `RELAX(u,v)`。
	        
		```text
		DAG_SHORTEST_PATHS(G, w, s)
		  L ← TOPOLOGICAL_SORT(G)        // O(V+E)
		  for each v ∈ V: v.d ← ∞; v.π ← NIL
		  s.d ← 0
		  for each u in L: // 針對所有 adj list 的每個點去做 Relax
			  for each (u, v) ∈ adj[u]:
				  RELAX(u, v)
		```
    
- **複雜度**：拓樸排序 $O(V+E)$；每邊鬆弛一次 $O(E)$；合計 **$O(V+E)$**。空間 $O(V)$。
    
- **可與不可**
    
    - 可：負邊。
        
    - 不可：任意環（DAG 定義使然），因此不會有負環。
        
- **輸出讀法**
    
    - 距離：$v.d$（不可達則 $=\infty$）。
        
    - 路徑：從 $v$ 逆著 $v.π$ 回溯到 $s$。
        
- **對照 Dijkstra**
    
    - Dijkstra 要求邊非負，時間常見 $O(E\log V)$；適用一般圖。
        
    - DAG 法更快 $O(V+E)$，但僅限 DAG。
### 範例

![01-範例](/vault-assets/5c3cabc5919dc2c45739.png)

## Dijkstra Algorithm

### 資料結構版本 `adj matrix`
    
- **資料結構**
    
    - 成本矩陣：$COST[i,j] =$  $\begin{cases}邊權 ,w(i,j)，若 (i,j)\in E \\  \infty，若 (i,j)\notin E\\  0，若 i=j。\end{cases}$
        
    - 布林陣列：$S[1..n]$（是否「定稿」）。
        
    - 距離陣列：$DIST[1..n]$（目前最短估計）。
        
    - 可選：$P[1..n]$（前驅，用來回溯路徑）。
        
- **初始化（源點 $s$）**
    
    - 對所有 $i$：$S[i]=false$，$DIST[i]=COST[s,i]$。
        
    - $S[s]=true$，$DIST[s]=0$，$P[s]=NIL$。
        
- **核心步驟（重複直到全數定稿）**
    
    - 選 $u=\min{DIST[i]\mid S[i]=false}$。
        
    - 設 $S[u]=true$（此時 $DIST[u]=\delta(s,u)$ 已定）。
        
    - 對所有尚未定稿的 $v$：若 $DIST[v] > DIST[u] + COST[u,v]$，則  
        $DIST[v] = DIST[u] + COST[u,v]$，$P[v]=u$。
        
- **矩陣版 Pseudocode（$O(n^2)$）**
    
```text
DIJKSTRA_MATRIX(COST, n, s)
  for i = 1..n:
	  S[i]    = false
	  DIST[i] = COST[s, i]
	  P[i]    = (COST[s, i] < ∞ and i ≠ s) ? s : NIL
  S[s] = true
  DIST[s] = 0

  repeat (n-2) times:
	  u = argmin_i { DIST[i] | S[i] = false }       // O(n)
	  S[u] = true
	  for v = 1..n:                                 // 掃一整列 O(n)
		  if S[v] = false and COST[u, v] < ∞ and DIST[v] > DIST[u] + COST[u, v]:
			  DIST[v] = DIST[u] + COST[u, v]
			  P[v] = u
```
    
- **時間與空間**
    
    - 每輪找 $u$ 掃一遍頂點 $O(n)$；鬆弛掃一整列 $O(n)$；共 $n-1$ 輪 ⇒ **$O(n^2)$**。
        
    - 空間：矩陣 **$O(n^2)$**，$DIST,S,P$ 各 $O(n)$。
        
    - 適合**稠密圖**或沒有堆結構時的教學實作；稀疏圖建議「鄰接串列 + 最小堆」版本，時間 $O((V+E)\log V)$。
        
- **必要條件與產出**
    
    - 權重限制：$w(i,j)\ge 0$。
        
    - 結束後：$DIST[v]=\delta(s,v)$；由 $P[v]$ 回溯得最短路徑樹。
        
- **不變式小抄**
    
    - 上界：對所有 $v$，$DIST[v]\ge \delta(s,v)$。
        
    - 正確性：每次選出的 $u$ 滿足 $DIST[u]=\delta(s,u)$，之後不再改變。
        
    - 鬆弛充足：矩陣版每回合檢查所有 $v$，不會漏掉可改進的鄰接。

![02-資料結構版本 adj matrix](/vault-assets/180cfc82b7d34ba6dbf4.png)

### 演算法版本

![03-演算法版本](/vault-assets/63c110d4bf6d5629a20b.png)

![04-演算法版本](/vault-assets/c7b0bcdd8504060f8425.png)

### 複雜度分析

- 前提：以鄰接串列表示圖，$w(u,v)\ge 0$。優先佇列 $Q$ 存所有頂點，鍵為 $v.d$。
    
- 成本分解
    
    - 初始化 `INITIALIZE-SINGLE-SOURCE`：$O(V)$。
        
    - 建立空的最小優先佇列 $Q$ 並把頂點放入：$O(V)$。
        
    - 重複 `EXTRACT-MIN(Q)` 共 $V$ 次：每次 $O(\log V)$ ⇒ $O(V\log V)$。
        
    - 外層 for/while 會掃過所有邊一次（針對每個 $u$ 的鄰接串列）：總計 $E$ 次檢查。
        
    - 對每條邊觸發一次 `DECREASE-KEY`：
        
        - Binary heap：每次 $O(\log V)$ ⇒ **$O(E\log V)$**。
            
        - Fibonacci heap：每次攤銷 $O(1)$ ⇒ **$O(E)$**。
            
- 總時間
	    
    - 若 $Q$ 用 **binary heap**：$O(V)$（初始化） $+$ $O(V\log V)$（extract） $+$ $O(E\log V)$（decrease-key）  
        ⇒ **$O((V+E)\log V)$**（常見寫法也可記成 $O(E\log V)$）。
        
    - 若 $Q$ 用 **Fibonacci heap**：$O(V)$ $+$ $O(V\log V)$ $+$ $O(E)$  
        ⇒ **$O(V\log V + E)$**。
        
- 備註：若用鄰接矩陣且不建堆，經典實作為 **$O(V^2)$**。
### Dijkstra's Algorithm 不能用在含負權邊的圖

![diagram-01](/vault-assets/d6e0f2126f66d91087e6.png)


- **條件**：來源 $A$；邊權：$A\to B=3,\ A\to C=5,\ C \to B=-4,\ B\to D=1$。
    
- **關鍵**：Dijkstra 會把先彈出的頂點視為「定稿」。若之後經由負邊能把它再變更小，演算法不會回頭改，因而錯誤。
    
- **示例最短路**：$A\to C \to B \to D$ 長度 $5+(-4)+1=2$，但 Dijkstra 會輸出 $A \to B \to D$ 長度 $4$。
    
- **正確做法**：含負邊用 Bellman–Ford；可偵測負環。
    
- **一步步（精簡版）**
    
    1. 初始：$B.d=3,\ C.d=5,\ D.d=\infty$。
        
    2. 取出 $B$ 定稿，鬆弛 $B \to D$ 得 $D.d=4$。
        
    3. 取出 $D$ 無事可做；再取出 $C$，鬆弛 $C \to B$ 得 $1<3$，但 $B$ 已定稿，不會改。
        
    4. 結束；輸出錯誤的 $4$。
## Bellman-ford Algorithm
### 定義

- 採用 DP 的方式
	
- $k$ 表示**允許的邊數上限**。
	
- 定義 $\mathrm{Dist}^k[i]$ 為：從起點 $s$ 到頂點 $i$、**最多使用 $k$ 條邊**的最短距離。  
	
- 則遞迴式： 
$$
	\mathrm{Dist}^k[i]
	=\min\Big(
	\underbrace{\mathrm{Dist}^{k-1}[i]}_{\text{路徑不增邊，走原路}},
	\ \underbrace{\min_{(j,i)\in E}\big(\mathrm{Dist}^{k-1}[j]+ \mathrm{cost}[j,i]\big)}_{\text{經由某前驅 $j$ 多走一條邊做 relaxation，更好就更新 }(j,i)}
	\Big).
$$
	
- 理由：用最多 $k$ 邊到 $i$ 的最短路，只可能是
		
	1. 不用第 $k$ 條邊，與 $k-1$ 邊解相同；或
	    
	2. 恰用 $k$ 條邊，最後一步走 $(j,i)$，其代價是到 $j$ 的 $k-1$ 邊最短路加上 $\mathrm{cost}[j,i]$。
	![05-定義](/vault-assets/918e1c4480e3c8d03867.png)
- 步驟
		
	1. 初始：$\mathrm{Dist}^0[s]=0,\ \mathrm{Dist}^0[i\ne s]=\infty$。 
		
	2. 重複 $k=1\ldots |V|-1$ 即得 $\mathrm{Dist}^{|V|-1}$ 為答案（任何簡單路徑至多 $|V|-1$ 邊）。  
		
	3. 再做一次鬆弛仍能下降則存在負環。
### 演算法實作

#### DS 版本
    
- **用途**：單源最短路，允許負邊；可偵測負環。
    
- **輸入**：頂點數 $n$，源點 $s$，成本矩陣 $COST[1..n,1..n]$  
    （$COST[i,j]=w(i,j)$；若無邊則 $=\infty$；$COST[i,i]=0$）。
    
- **輸出**：$DIST[v]=\delta(s,v)$ 與 $P[v]$（前驅）；若有負環則回報。
    

```text
BELLmanFord_MATRIX(COST, n, s)
  # 初始化 O(n)
  for i = 1..n:
      DIST[i] = COST[s,i]        # s 到 i 的一跳估計
      P[i]    = (COST[s,i] < ∞ and i ≠ s) ? s : NIL
  DIST[s] = 0

  # 主要迴圈：做 (n-1) 輪鬆弛；矩陣版每輪掃所有有向邊 O(n^2)
  for k = 1..(n-1):
      for u = 1..n:
          for v = 1..n:
              if  DIST[v] > DIST[u] + COST[u,v]:
                  DIST[v] = DIST[u] + COST[u,v]
                  P[v]    = u

  # 第 n 輪檢查是否仍可改善 → 有可達負環
  hasNegCycle = false
  for u = 1..n:
      for v = 1..n:
          if DIST[v] > DIST[u] + COST[u,v]:
              hasNegCycle = true   # s 可達的負環存在
  return (DIST, P, hasNegCycle)
```

- **時間**：矩陣版掃邊 $n^2$ 次、共 $n-1$ 輪 ⇒ **$O(n^3)$**。  
    鄰接串列版為 **$O(VE)$**。
    
- **正確性關鍵**：在無可達負環時，任一路徑最多含 $n-1$ 條邊；逐輪把長度 $\le k$ 的最佳路徑推進到第 $k$ 輪收斂。
    
- **路徑輸出**：對任一 $t$，自 $t$ 逆著 $P[\cdot]$ 回溯到 $s$。

#### CLRS 版本

![06-CLRS 版本](/vault-assets/f34db5018f420d1e6684.png)

時間複雜度 $O(VE)$，那麼是因為 base on adj list
### 範例

![07-範例](/vault-assets/320e7ff5dd2ba1cab4bf.png)

![08-範例](/vault-assets/8c5c17d7aa4da7364539.png)
