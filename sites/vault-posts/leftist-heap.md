---
title: "左偏堆（Leftist Heap）"
slug: leftist-heap
topic_section: data-structures
description: "My vault 資料結構筆記：左偏堆（Leftist Heap）。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Leftist heap or min Leftist Tree.md"
---

## 定義

* **Null-path length（`shortest` / `npl`）**

$$
\text{shortest}(x)=\begin{cases}
0, & \text{x 為外部節點（空）}\\
1+\min\{\text{shortest}(x.\text{left}),\;\text{shortest}(x.\text{right})\}, & \text{否}
\end{cases}
$$

* **Leftist tree（左傾樹）**：對每個內部節點 $x$，$\text{shortest}(x.\text{left})\;\ge\;\text{shortest}(x.\text{right}).$

* **Leftist heap**：同時滿足 $\text{key(parent)}\;\le\;\text{key(child)}\quad\text{(min-heap)}$ 與上式左傾條件。

> 備註：Leftist heap 不是完全二元樹（Complete BT）。

### 定理

對任意 leftist tree，令 $S(x)=shortest(x)$ 到外部節點(leaf or null node)。若根為 $x$ 且 ，則

$$
N(x) \;\ge\; 2^{S(X)}-1.
$$

亦即：$k\le \lfloor \log_2(N(x)+1)\rfloor$，所以右鏈長度 $=O(\log N)$。

#### 證明（數學歸納法 on $k$）

**Base $k=0$**：$x$ 為外部節點，$N(x)=0\ge2^{0}-1=0$，成立。

**Induction step**：假設對所有 $k$ 皆成立。若 $\text{shortest}(x)=k$，依 leftist 性質，至少有一邊子樹（左或右）滿足 $\text{shortest}(\text{sub})=k-1$

由歸納假設，該子樹之節點數 $\ge 2^{k-1}-1$。另一邊子樹的節點數 $\ge 0$。因此
$$
\begin{aligned}
N(x)
&= 1 + N(\text{left}) + N(\text{right})\\[2pt]
&\ge 1 + (2^{k-1}-1) + 0\\[2pt]
&= 2^{k-1}\\[2pt]
&\ge 2^{k}-1\quad (\text{當左右皆有 }\text{shortest}=k-1\text{ 時可達 } 2^{k}-1).
\end{aligned}
$$
更緊的標準推法（兩邊最短皆為 $k-1$）可得：

$$
N(x)\ge 1 + (2^{k-1}-1) + (2^{k-1}-1) = 2^{k}-1.
$$

故命題成立。

## 操作
### Merge（核心）

1. 若 $h_1$ 或 $h_2$ 為空，回傳另一個。

2. 令根鍵較小者為主堆 $H$。遞迴合併另一堆到 $H.\text{right}$。

3. 若 $\text{shortest}(H.\text{left}) < \text{shortest}(H.\text{right})$，交換左右子樹。

4. 更新 $\text{shortest}(H)$：
$$
\text{shortest}(H)=1+\min\{\text{shortest}(H.\text{left}),\;\text{shortest}(H.\text{right})\}.
$$

**為何是 $O(\log n)$**：遞迴只沿右鏈下行，且左傾條件保證右鏈長度 $=O(\log n)$，因此合併與其餘兩操作皆為 $O(\log n)$。

![01-Merge(核心)](/vault-assets/abc334b02d8fe538f16b.png)

### insert 

![02-insert](/vault-assets/fdcdb85be80fec3d2ff8.png)


### Delete-min

![03-Delete-min](/vault-assets/c59110e08296bfc49b5a.png)


### 複雜度
| 操作         | 複雜度         |
| ---------- | ----------- |
| Insert     | $O(\log n)$ |
| Delete-min | $O(\log n)$ |
| Merge      | $O(\log n)$ |
