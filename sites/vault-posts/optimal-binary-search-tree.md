---
title: "OBST (Optimal Binary Search Tree)"
slug: optimal-binary-search-tree
topic_section: algorithms
description: "My vault 演算法筆記：OBST (Optimal Binary Search Tree)。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/OBST (Optimal Binary Search Tree).md"
---

## 問題背景

你要做英→拉脫維亞字典查詢。每個英文單字出現頻率不同；常見字應該更接近根，以降低平均查詢步數。有些查詢不在字典內，也要計入「失敗查詢」的機率。

## 正式定義（CLRS）

* 排序後的 **n 個鍵**：$K=\langle k_1<k_2<\cdots<k_n\rangle$。

* **成功查詢機率**：對每個鍵 $k_i$ 有 $p_i$。

* **失敗查詢機率（dummy keys）**：$n+1$ 個 dummy 鍵 $d_0, d_1,\ldots,d_n$。

* $d_0$：小於 $k_1$ 的所有值。

* 對 $1\le i\le n-1$，$d_i$：介於 $k_i$ 與 $k_{i+1}$ 的所有值。

* $d_n$：大於 $k_n$ 的所有值。

* 對每個 $d_i$ 給定失敗查詢機率 $q_i$。

* 機率總和：$\sum_{i=1}^{n} p_i + \sum_{i=0}^{n} q_i = 1.$

* 目標：構造一棵 BST，使 **期望查詢成本** 最小。

### 成本模型

* 真實成本＝搜尋時**被檢視的節點數**。

* 若搜尋命中鍵 $k_i$ 或 dummy $d_i$，成本 = 該節點在樹中的 **深度 depth** $+1$。

### 期望查詢成本（CLRS 公式 14.11）

令 $\operatorname{depth}_T(x)$ 為節點 $x$ 在樹 $T$ 的深度（根深度 0）。則

$$
\begin{aligned}
\mathbb{E}[\text{search cost in }T]
&= \sum_{i=1}^{n} (\operatorname{depth}_T(k_i)+1)\,p_i \\
&\quad + \sum_{i=0}^{n} (\operatorname{depth}_T(d_i)+1)\,q_i \\
&= \boxed{\,1 + \sum_{i=1}^{n} \operatorname{depth}_T(k_i)\,p_i
+ \sum_{i=0}^{n} \operatorname{depth}_T(d_i)\,q_i\, }.
\end{aligned}
$$

> 解讀：常見鍵的深度要小；dummy 也要安排在淺層以降低失敗查詢成本。

### 圖 14.9 的資料（範例）

![01-圖 14.9 的資料(範例)](/vault-assets/78cc42f006e3ac60e24c.png)

$$
\begin{array}{c|cccccc}
i   & 0    & 1    & 2    & 3    & 4    & 5 \\ \hline
p_i &      & 0.15 & 0.10 & 0.05 & 0.10 & 0.20 \\
q_i & 0.05 & 0.10 & 0.05 & 0.05 & 0.05 & 0.10
\end{array}
$$
* $n=5$。

* 成功機率：$p_1=0.15,\; p_2=0.10,\; p_3=0.05,\; p_4=0.10,\; p_5=0.20.$

* 失敗機率：$q_0=0.05,\; q_1=0.10,\; q_2=0.05,\; q_3=0.05,\; q_4=0.05,\; q_5=0.10.$

* 兩棵候選 BST 的期望成本：

	* (a) $\mathbb{E}=2.80$

	* (b) $\mathbb{E}=2.75$（**最優**）

#### 範例重點

* 最優樹會把高機率鍵與高機率 dummy 安排在較淺層。

* 僅靠「平衡」不夠，必須用機率加權的期望成本最小化，目標是最小化上述期望值，而不只是樹高或純平衡。

- OBST 同時考慮 **命中** 與 **未命中** 的頻率。

- 期望成本 = 1 + 加權深度和（鍵與 dummy）。

## 證明

### Step 1. Optimal Substructure

* **子樹必須連續區間**：任何 BST 的子樹都包含一段連續鍵值 $k_i, \ldots, k_j$。

* **包含 dummy keys**：子樹也必須包含對應的 dummy keys $d_{i-1}, \ldots, d_j$。

* **最優子結構性質**：若整體最優 BST $T$ 的子樹 $T'$ 含鍵 $k_i, \ldots, k_j$，則 $T'$ 本身對應的子問題也必須是最優解。否則若存在更好的子樹替代，就會降低總期望成本，矛盾。

- **有關於 Empty BST 的考慮**：那麼左樹和右樹就是對稱的說，因為 $j=i-1$ 的話代表數已經是空的了，所以也就代表沒有 $key$，所以一定要有 $\text{dummy key}$ 做支持不然這樣結構會出錯誤
	![02-Step 1. Optimal Substructure](/vault-assets/0f6bc14135b522fbcf6b.png)![03-Step 1. Optimal Substructure](/vault-assets/4eb3edb0c12b42bcdfae.png)

* **結論**：Optimal BST 問題具備最優子結構。

### Step 2. Recursive Subproblem

![04-Step 2. Recursive Subproblem](/vault-assets/58f0f79be1ef45b052ec.png)

* **子問題定義**：令 $e[i,j]$ 表示含鍵 $k_i, \ldots, k_j$ 與 dummy keys $d_{i-1}, \ldots, d_j$ 的最小期望成本。

* **邊界情況**：當 $j = i-1$，子問題只包含 dummy key $d_{i-1}$，此時 $e[i, i-1] = q_{i-1}.$

* **權重定義**：對區間 $[i,j]$，定義 $w(i,j) = \sum_{t=i}^j p_t + \sum_{t=i-1}^j q_t.$ 表示區間內所有查詢機率總和。

* **遞迴轉移**：若以 $k_r$ 作為根節點（其中 $i \le r \le j$），則 $e[i,j] = e[i,r-1] + e[r+1,j] + w(i,j).$

	* 左子樹：$k_i, \ldots, k_{r-1}$，成本 $e[i,r-1]$
	
	* 右子樹：$k_{r+1}, \ldots, k_j$，成本 $e[r+1,j]$
	
	* 由於子樹深度整體 +1，總額外成本 = 區間機率和 $w(i,j)$

* **最優選擇**：取使成本最小的根：$e[i,j] = \min_{i \le r \le j} \{ e[i,r-1] + e[r+1,j] + w(i,j) \}.$

### Step 3. Recursive Formulation (CLRS 公式 14.14)

$$
e[i,j] =
\begin{cases}
q_{i-1}, & j = i-1, \\
\min_{i \le r \le j} \{ e[i,r-1] + e[r+1,j] + w(i,j) \}, & i \le j.
\end{cases}
$$

* 此公式定義了所有子問題的最小期望成本。

* 可同時計算 `root[i,j]`，記錄最優根節點位置。
#### ALGO

![05-ALGO](/vault-assets/2f2617fed3a60217b505.png)

- $O(n^3)$ 可以想 $O(n^2) \times O(n)$ 因為 $n \times n$ 格格子，那麼在跑 n 次所以就是 $O(n^3)$ 
- 可以想像 $l$ 是滑動視窗跟 $i$ 配合 $j$ 負責最後的位置
#### 計算方式

![06-計算方式](/vault-assets/5042277e4f69939c8c4d.png)
