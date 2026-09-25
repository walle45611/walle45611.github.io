---
title: "線性時間排序演算法"
slug: linear-time-sorting
topic_section: data-structures
description: "線性時間排序演算法的重點整理。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Linear-time sorting algo.md"
---

## Counting Sort

![01-Counting Sort](/vault-assets/8be1f9a5fbeac9640ff3.png)

![02-Counting Sort](/vault-assets/e0b1ad7bb3028da5a4b7.png)


### Counting Sort 分析

|指標|最佳|平均|最差|說明|
|---|--:|--:|--:|---|
|時間複雜度|$\Theta(n+k)$|$\Theta(n+k)$|$\Theta(n+k)$|初始化 $C$→$\Theta(k)$，計數與前綴和→$\Theta(n)+\Theta(k)$，回填 $B$→$\Theta(n)$|
|空間複雜度|$\Theta(n+k)$|$\Theta(n+k)$|$\Theta(n+k)$|需要 $B[1..n]$ 與 $C[0..k]$|
|穩定性|**Stable**|**Stable**|**Stable**|由右往左回填保持同值相對次序|
### Proof

- **符號**  $A[1..n]$ 輸入，每個鍵在 $0..k$; $B[1..n]$ 輸出; $C[0..k]$ 計數。
	
- **步驟**
		
	1. 初始化：對 $i=0..k$，令 $C[i]=0$。
	    
	2. 計數：掃描 $A$，對 $x=A[j]$ 執行 $C[x] \leftarrow C[x]+1$。完成後 $C[i]$ 為「值等於 $i$ 的個數」。
	    
	3. <mark>前綴和：對 $i=1..k$，令 $C[i] \leftarrow C[i]+C[i-1]$</mark>
	    
	4. 回填<mark>（由右往左）</mark>：對 $j=n..1$，令 $x=A[j]$，設 $B[C[x]]\leftarrow x$，再令 $C[x]\leftarrow C[x]-1$。
	
- **正確性**
		
	- 由上式，$C[i]$ 是「$\le i$ 的元素個數」。因此值為 $i$ 的元素應被放入區間 $(C[i-1]+1)..C[i]$。
	    
	- 在第 4 步中，$j$ 自右往左處理並遞減 $C[i]$，恰好把所有值為 $i$ 的元素依序填入該區間，且保持原相對次序<mark>（**穩定**）</mark>。
    
- **時間**
		
	- 初始化：$k+1$ 次賦值 ⇒ $\Theta(k)$。
	    
	- 計數：掃一次 $A$ ⇒ $\Theta(n)$。
	    
	- 前綴和：掃一次 $C$ ⇒ $\Theta(k)$。
	    
	- 回填：掃一次 $A$ ⇒ $\Theta(n)$。  
		
	- 合併： $\Theta(k)+\Theta(n)+\Theta(k)+\Theta(n)=\Theta(n+k)$
	    
	- <mark>那為什麼是 linear time 是因為可以使用兩個角度看</mark>
			
		1. 若 $k$ 的值域也是成 linear 分佈 $O(n)$，那麼 $O(n+k) \Rightarrow O(n+O(n))$
			
		2. 也可以把 $k$ 的值域限制在 $0$~$k$ 之間那麼可以把 $k$ 看成常數那麼 $O(n+k)=O(n+c)$
	
- **空間**  額外需要 $B[1..n]$ 與 $C[0..k]$，故為  $\Theta(n+k)$
	
- **備註** 非比較式排序，故不受比較模型的 $\Omega(n\log n)$ 下界限制。

## Radix Sort (CLRS) or LSD Radix Sort (DS)
### 核心概念

- 非比較式排序；以「位數」為鍵，由**最低位→最高位**依序處理（LSD）。
    
- 每回合使用**穩定**子排序（常用 Counting Sort），或是可以使用 Bubble 或是 Insertion。
    
- 流程：按當前位分桶 $0..r-1$ → 依 $0..r-1$、以 **FIFO** 合併回陣列。
    
- 不變量：處理到位 `pos` 後，**最低 `pos+1` 位已有序**；下一回合不破壞既有次序。
    
- 前提：每個鍵可表示為 `d` 位、基底 `r`（每位範圍 $0..r-1$）。
    
- 複雜度：子排序若為 $\Theta(n+r)$，則總成本 $\Theta\big(d(n+r)\big)$；空間視桶與子排序而定。

### 操作方式

- distribution：依據個資料的位數數值，<mark>分派到對應的桶子中</mark>。
	
- merge：依據桶子的編號 $0 \rightarrow (r-1)$ 桶子裡面把資料 merge <mark>然後變成下一回合的輸出</mark>

#### LSD Radix Sort — 虛擬碼（Pseudocode）

```text
RADIX-SORT-LSD(A, n, d, r):
  # A[1..n]，每個鍵有 d 位，基底 r（每位 0..r-1）
  for pos ← 0 to d-1:                 # 0=最低位
    STABLE-COUNTING-SORT-BY-DIGIT(A, n, pos, r)
```

```text
STABLE-COUNTING-SORT-BY-DIGIT(A, n, pos, r):
  # 以第 pos 位作穩定計數排序；輸出回寫到 A（可用輔助陣列 B）
  create array C[0..r-1] ← 0
  create array B[1..n]

  # 計數
  for j ← 1 to n:
    d ← DIGIT(A[j], pos, r)           # 取第 pos 位
    C[d] ← C[d] + 1

  # 前綴和（≤ i 的個數）
  for i ← 1 to r-1:
    C[i] ← C[i] + C[i-1]

  # 回填（由右往左，確保穩定）
  for j ← n downto 1:
    d ← DIGIT(A[j], pos, r)
    B[C[d]] ← A[j]
    C[d] ← C[d] - 1

  copy B[1..n] → A[1..n]
```

```text
DIGIT(x, pos, r):
  # 以基底 r 取第 pos 位（pos=0 為最低位）
  return ⌊ x / r^pos ⌋ mod r
```

![03-以基底 r 取第 pos 位(pos=0 為最低位)](/vault-assets/1dcc0ea7a0f8c3020443.png)


### Radix Sort 分析

|指標|最佳|平均|最差|說明|
|---|--:|--:|--:|---|
|時間複雜度|$\Theta\big(d(n+r)\big)$|$\Theta\big(d(n+r)\big)$|$\Theta\big(d(n+r)\big)$|每回合分派 $\Theta(n)$ + 合併/穩定子排 $\Theta(n+r)$，共 $d$ 回合|
|空間複雜度|$\Theta(n+r)$|$\Theta(n+r)$|$\Theta(n+r)$|需 $r$ 個 bucket 或一個輸出陣列 $B$ 與計數陣列 $C[0..r-1]$|
|穩定性|**需穩定子排序**|**需穩定子排序**|**需穩定子排序**|常用 Counting Sort（穩定）|

- **符號**：$n$ 筆資料；每鍵 $d$ 位、基底 $r$（每位 $0..r-1$）。

- **成本拆解**：一次回合 $=\Theta(n+r)$；總共 $d$ 回合 ⇒ $\Theta(d(n+r))$。

- **線性時間條件**：若 $d=C_1$ 為常數且 $r=\mathcal{O}(n)$，所以可視為 $O(O(n) \times (n+C_1))$，則為 $\Theta(n)$，。

- **備註**：桶合併需依 $0..r-1$ 且 FIFO；否則會破壞穩定性。


## Bucket Sort (CLRS) or MSD Radix Sort (DS)

![04-Bucket Sort (CLRS) or MSD Radix Sort](/vault-assets/0fd6eb40a954f3389849.png)


> 目標：在輸入獨立且均勻於 $[0,1)$ 的假設下，證明 **Bucket sort 的期望時間為 $\Theta(n)$**。

### 1) 問題設定與演算法

- 輸入：$x_1,\dots,x_n\in[0,1)$，彼此獨立且均勻。
    
- 建 $m$ 個桶，桶 $i$ 的區間為 $[\tfrac{i}{m},\tfrac{i+1}{m})$（CLRS 取 $m=n$）。
    
- 分配：元素 $x$ 放入桶 $b(x)=\big\lfloor m,x\big\rfloor$。
    
- 各桶內排序（常用 insertion sort）。
    
- 按桶號 0..$m-1$ 串接輸出。
    
```text
BUCKET-SORT(A[1..n], m):
  make buckets B[0..m-1] as lists
  for x in A:
    i ← floor(m * x)
    push x into B[i]
  for i = 0..m-1:
    insertion-sort(B[i])
  return concatenation of B[0],…,B[m-1]
```

### 2) 成本分解

令第 $i$ 桶大小為 $n_i$。建桶、分配與串接是線性；瓶頸在桶內排序：  
$$
T(n)=\underbrace{\Theta(n)}_{\text{建桶+分配+串接}} + \sum_{i=0}^{m-1} O\big(n_i^2\big).\tag{1}
$$

> 為何是 $O(n_i^2)$：插入排序在長度為 $n_i$ 的序列上成本 $\Theta(n_i^2)$。

### 3) 機率模型與分佈

固定桶 $i$，對每個元素定義**指示變數**，也就是該元素他會不會放在該桶 i 就是該桶 j 就是該元素的 index 然後有放在裡面就是 1，反之：  
$$
I^{(i)}_j = \mathbf 1{, x_j\in[\tfrac{i}{m},\tfrac{i+1}{m}),} \in {0,1}.
$$
均勻於 $[0,1)$ ⇒ $\Pr\big(I^{(i)}_j=1\big)=\tfrac{1}{m}$，且各 $I^{(i)}_j$ 彼此獨立。  
桶大小是這些指示變數的和：  
$$
n_i = \sum_{j=1}^{n} I^{(i)}_j \sim \mathrm{Bin}\left(n,\tfrac{1}{m}\right).\tag{2}
$$
因此有矩：  
$$
\mathbb E[n_i]=\tfrac{n}{m},\qquad \mathrm{Var}(n_i)=\tfrac{n}{m}\Big(1-\tfrac{1}{m}\Big),
$$
$$
\mathbb E[n_i^2]=\mathrm{Var}(n_i)+\big(\mathbb E[n_i]\big)^2
=\tfrac{n}{m}\Big(1-\tfrac{1}{m}\Big)+\Big(\tfrac{n}{m}\Big)^2.\tag{3}
$$

### 4) 期望時間的計算

對 (1) 取期望，並用線性期望：  
$$
\mathbb E[T(n)] = \Theta(n) + \sum_{i=0}^{m-1} O\big(\mathbb E[n_i^2]\big).
$$
將 (3) 代入並把常數吸收進 $O(\cdot)$：  
$$
\mathbb E[T(n)]
= \Theta(n) + \sum_{i=0}^{m-1} O\Big(\tfrac{n}{m}+\big(\tfrac{n}{m}\big)^2\Big)
= \Theta(n) + m\cdot O\Big(\tfrac{n}{m}+\tfrac{n^2}{m^2}\Big)
= \Theta(n) + O\Big(n + \tfrac{n^2}{m}\Big).\tag{4}
$$
**CLRS 標準選擇 $m=n$**：  
$$
\mathbb E[T(n)] = \Theta(n) + O\Big(n + \tfrac{n^2}{n}\Big)
= \Theta(n) + O(n) = \Theta(n).\tag{5}
$$
這證明了在均勻獨立且 $m=n$ 的設計下，Bucket sort 的**期望時間為線性**。

> 註：若取一般 $m$，(4) 給出 $\Theta(n)+O(n+n^2/m)$；只要 $m=\Omega(n)$，期望時間仍為 $\Theta(n)$ 級別。

### 5) 直觀範例（$m=4$ 桶）

區間：$[0,0.25)$、$[0.25,0.5)$、$[0.5,0.75)$、$[0.75,1)$。  
資料：$x={0.12,0.28,0.51,0.77,0.83}$。

分桶（$b(x)=\lfloor 4x\rfloor$）：

- 桶0：${0.12}\Rightarrow n_0=1$
    
- 桶1：${0.28}\Rightarrow n_1=1$
    
- 桶2：${0.51}\Rightarrow n_2=1$
    
- 桶3：${0.77,0.83}\Rightarrow n_3=2$
    

隨機模型下（若 $x_j$ 來自 $[0,1)$ 均勻）：

- 對固定桶 $i$，$I^{(i)}_j\sim\mathrm{Bernoulli}(1/4)$。
    
- $n_i\sim\mathrm{Bin}(n,1/4)$，故 $\mathbb E[n_i]=n/4$、$\mathrm{Var}(n_i)=n\cdot\tfrac14\cdot\tfrac34$。
    
- CLRS 常取 $m=n$，此時 $\mathbb E[n_i]=1$，$\mathrm{Var}(n_i)=1-\tfrac1n$，且  
    $$\mathbb E[n_i^2]=2-\tfrac1n,$$  
    進而得 $\sum_i \mathbb E[n_i^2]=\Theta(n)$，回到 (5)。
    
### 速記版

- 分桶：$b(x)=\lfloor m x\rfloor$；CLRS 取 $m=n$。
    
- 成本：$T=\Theta(n)+\sum O(n_i^2)$。
    
- 分佈：$n_i\sim\mathrm{Bin}(n,1/m)$，$\Rightarrow\ \mathbb E[n_i^2]=\tfrac{n}{m}(1-\tfrac1m)+(\tfrac{n}{m})^2$。
    
- 結論：$\mathbb E[T]=\Theta(n)+O(n+n^2/m)$，特別地 $m=n\Rightarrow \mathbb E[T]=\Theta(n)$，如果 n 和 m 相等那麼就是均勻放置那麼代表，會在 $\Theta(n)$，如果今天都放在同一個桶子那麼就會變成 $\Theta(n^2)$。
### Bucket Sort 比較次數小抄

**定義**

令某桶大小為 $t$。以 insertion sort 排此桶的最壞比較次數：

$$
C(t)=\sum_{j=2}^{t}(j-1)=1+2+\cdots+(t-1)=\frac{t(t-1)}{2}.
$$

**例：** $n=6,\ m=3$

* 全擠一桶 $(6,0,0)$：

$$
\text{總比較}=C(6)+C(0)+C(0)=\frac{6\cdot5}{2}=15=\Theta(n^2).
$$

* 均勻分 $(2,2,2)$：

$$
\text{總比較}=C(2)+C(2)+C(2)=1+1+1=3=\Theta\big(\tfrac{n^2}{m}\big).
$$

**一般情況：** $(t_1,\dots,t_m)$

$$
\text{總比較}=\sum_{i=1}^{m} C(t_i)=\frac{1}{2}\sum_{i=1}^{m} t_i(t_i-1)
=\Theta\Big(\sum_{i=1}^{m} t_i^2\Big).
$$

**直覺**

* 全擠同一桶：$t_1=n\Rightarrow\sum t_i^2=n^2\Rightarrow O(n^2)$。

* 均勻分配：$t_i\approx n/m\Rightarrow\sum t_i^2\approx m\cdot (n/m)^2=n^2/m$。
