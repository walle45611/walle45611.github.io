---
title: "AVL 樹：平衡條件與旋轉"
slug: avl-tree
topic_section: data-structures
description: "My vault 資料結構筆記：AVL 樹：平衡條件與旋轉。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Adelson-Velsky and Landis tree(AVL Tree).md"
---

### 基本定義

- 在一般 BST 中可能退化為 skewed，致使 $\text{search/insert/delete} = O(n)$。
    
- AVL Tree 維持 height balanced，保證 $O(\log n)$。
    
- 平衡因子：$\mathrm{BF}(v) = H_L - H_R \in {-1, 0, 1}$。
    
- 任何節點滿足 $|H_L - H_R| \leq 1$，且左右子樹皆為 AVL Tree。
    
- 搜尋、插入、刪除：$O(\log n)$。
    
- 插入時旋轉次數：$O(1)$；刪除最壞情況：$O(\log n)$。
    
- 旋轉類型：$LL, RR, LR, RL$。
    
    - $LL, RR$ → 單旋轉（single rotation），約 2 個指標改動。
        
    - $LR, RL$ → 雙旋轉（double rotation），約 4 個指標改動。
        

### 判斷是不是 balanaced binary tree


#### Ex1：根節點失衡 → Not AVL

![diagram-01](/vault-assets/1c44961d461258246cc7.png)


$\Rightarrow\ \mathrm{BF}(10)=3-1=2>1\ \Rightarrow\ \text{Not AVL.}$


#### Ex2：內部節點失衡（根平衡但子樹不平衡）→ Not AVL

![diagram-02](/vault-assets/9a1ec1f3370e7a9ea3a0.png)


$\Rightarrow\ \mathrm{BF}(10)=2-(-1)=3>1\ \Rightarrow\ \text{Not AVL.}$


#### Ex3：違反 BST 順序 → Not BST


![diagram-03](/vault-assets/95865029ae04976eed10.png)


> 註：在 AVL 檢查中，需同時滿足「每個節點 $|H_L-H_R|\le 1$」與「整棵樹是 BST」。上例 Ex2 為內部失衡示例；Ex3 為 BST 條件不滿足的示例。

### Horowitz 調整原則

1. 取三節點重排：中間鍵值上提，$\text{small} \to \text{left}$，$\text{large} \to \text{right}$。
    
2. 旋轉後孤兒子樹依 BST 規則掛回正確位置。

#### unbalance 案例與修正（邊線標註 L/R，調整後不再標）

##### LL（單右旋）

![diagram-04](/vault-assets/1b588a6ee515181792e0.png)


→ 調整後

![diagram-05](/vault-assets/b058542284c7fe20d8b4.png)


##### RR（單左旋）

![diagram-06](/vault-assets/38740a5e330551d581f8.png)


→ 調整後

![diagram-07](/vault-assets/dc12448d2ca24c45de1b.png)


##### LR（雙旋：先左 @B 再右 @A）

![diagram-08](/vault-assets/0905c1e0b9e8bbb90a52.png)


→ 調整後

![diagram-09](/vault-assets/265a137237cf5e177cba.png)


##### RL（雙旋：先右 B 再左 @A）

![diagram-10](/vault-assets/1ce24336dff34f9996eb.png)


→ 調整後

![diagram-11](/vault-assets/8c7ff4831c36823c089c.png)


### CLRS 調整原則

- 只用 **指標重接**，不搬值。維持 BST 次序不變。
    
- 兩種基本操作：`Left-Rotate(x)` 與 `Right-Rotate(y)`。
    
- 單旋轉（LL、RR）只改 2 條邊；雙旋轉（LR、RL）改 4 條邊。
    
- 旋轉前後，子樹 `a,b,c` 的 **中序順序** 仍為 `a < b < c`。

#### 基本操作（CLRS 風格）

**Right-Rotate(x)**

![diagram-12](/vault-assets/52823c71a7ed55045307.png)


→ 執行 `Right-Rotate(x)` 後：

![diagram-13](/vault-assets/f444e964e270cf1982ae.png)


**Left-Rotate(x)**

![diagram-14](/vault-assets/54167a661644bb4de9a5.png)


→ 執行 `Left-Rotate(x)` 後：

![diagram-15](/vault-assets/20c242c96a986ddb065f.png)


#### 單旋轉（LL, RR）

**LL rotation**（在節點 A 失衡，形如 A←B←C）：對 `A` 做 `Right-Rotate(A)`

![diagram-16](/vault-assets/f13b26d0790a9b3af262.png)


→ 調整後

![diagram-17](/vault-assets/0139218897d46ad4965c.png)


**RR rotation**（在節點 A 失衡，形如 A→B→C）：對 `A` 做 `Left-Rotate(A)`

![diagram-18](/vault-assets/88dd459c9fabe4218ab6.png)


→ 調整後

![diagram-19](/vault-assets/d545ec97b3dd46282cda.png)


#### 雙旋轉（LR, RL）

**LR rotation**（先 Left‑Rotate(B)，再 Right‑Rotate(A)）

**Before**

![diagram-20](/vault-assets/6b603b58d621da499fd5.png)


**After Left‑Rotate(B)**

![diagram-21](/vault-assets/5428673eb816c1a52978.png)


**After Right‑Rotate(A)**

![diagram-22](/vault-assets/7f1c41940244f79a13ab.png)


**RL rotation**（先 Right‑Rotate(B)，再 Left‑Rotate(A)）

**Before**

![diagram-23](/vault-assets/4390ffe27bd3c91320f6.png)


**After Right‑Rotate(B)**

![diagram-24](/vault-assets/cd3395e8394b8cc0d96c.png)


**After Left‑Rotate(A)**

![diagram-25](/vault-assets/503cd4f3ff3565616175.png)

**複雜度與不變量**

- 單旋轉改 2 條指標，雙旋轉改 4 條指標。
    
- 旋轉保持中序次序與 BST 性質；高度在首個失衡點減少或維持，整體操作 $O(1)$，插入/刪除恢復平衡在 $O(\log n)$ 路徑內完成。

### 相關定理


**定理（root level = 1）**

- 高度 $h$ 的 AVL Tree：
    
    - 最少節點數：$N_{\min}(h)=F_{h+2}-1$。
        
    - 最多節點數：$N_{\max}(h)=2^{h}-1$。
        

#### 證明
##### 最少 Node 建構 AVL Tree
$$
\begin{aligned}
&\textbf{定理}\,:~\text{高度 }h\text{ 的 AVL 樹（root level}=1,\ \mathrm{height}(\varnothing)=0)\text{ 之最少節點數 }N_{\min}(h)=F_{h+2}-1,\\
&\text{其中 }F_0=0,\ F_1=1,\ F_{n+2}=F_{n+1}+F_n.
\end{aligned}
$$

$$
\begin{aligned}
&\textbf{證明} \\[2mm]
&(1)\ \text{基礎情形：}\ h=0\Rightarrow N_{\min}(0)=0=F_{2}-1. \\[2mm]
&(2)\ \text{歸納假設：}\ \forall\,k\le h-1,\ N_{\min}(k)=F_{k+2}-1. \\[2mm]
&(3)\ \text{當高度 }h\text{ 時，最少節點必發生在左右子樹高差 }1\text{ 的情況：}\\
&\qquad\text{若左右皆為 }h-1\text{，節點較多；若皆為 }h-2\text{，整樹高度}<h.\\
&\qquad\text{不失一般性令 }H_L=h-1,\ H_R=h-2\ (\text{且兩子樹皆取最少構形}).\\
&\qquad\text{依歸納假設： }N_{\min}(h-1)=F_{h+1}-1,\quad N_{\min}(h-2)=F_h-1.\\[1mm]
&\Rightarrow\ N_{\min}(h)=1+N_{\min}(h-1)+N_{\min}(h-2)\\
&\qquad\qquad\ =1+(F_{h+1}-1)+(F_h-1)=F_{h+1}+F_h-1=F_{h+2}-1. \\[2mm]
&(4)\ \text{由 (1)--(3) 得證。}\ \square
\end{aligned}
$$

$$
\begin{aligned}
&\textbf{補充：最多節點數}\quad\text{第 }i\text{ 層至多 }2^{i-1}\text{ 個（Lemma）}\\
&\Rightarrow\ N_{\max}(h)=\sum_{i=1}^{h}2^{\,i-1}
=\sum_{i=0}^{h-1}2^i=\frac{1-2^h}{1-2}=2^h-1.
\end{aligned}
$$

最瘦合法（一側 $h-1$，另一側 $h−2$）

![diagram-26](/vault-assets/f941ae76e6eb07f12d2d.png)


違反例（若一側降到 $h−3$ 即非 AVL）

![diagram-27](/vault-assets/11f54ea260957007d761.png)


##### 最大的高度 AVL 需要幾個 NODE 大約 $1.44 \times \log(N)$

#### 例 1｜高度 5 的 AVL Tree 之最少節點數目？

- 最少節點：$N_{\min}(5)=F_7-1=13-1=12$。
    
- 最多節點：$N_{\max}(5)=2^5-1=31$。
    

**圖｜高度 5，最少節點構型（左右高度差皆 1）**

![diagram-28](/vault-assets/d88585a1643d38e4f496.png)



#### 例 2｜$n=100$ 個節點，求最小高度與最大高度

- $h_{\min}=\lceil\log_2(n+1)\rceil=\lceil\log_2 101\rceil=7$。
    
- $h_{\max}=$ 最大滿足 $F_{h+2}-1\le n$ 的 $h$。因 $F_{11}=89\le100<F_{12}=144$，故 $h_{\max}=9$。
    

**圖｜高度 9 的最少節點構型（節點以占位字母表示）**

![diagram-29](/vault-assets/ddf5145938b6a5426acc.png)


**速查表**（$F_0=0, F_1=1$）

|$h$|$F_{h+2}$|$N_{\min}(h)$|$N_{\max}(h)$|
|--:|--:|--:|--:|
|5|13|12|31|
|6|21|20|63|
|7|34|33|127|
|8|55|54|255|
|9|89|88|511|
|10|144|143|1023|


### 複雜度比較

**Priority Queue ADT（複雜度）**

|操作|heap|AVL tree|
|---|---|---|
|`Q = new-empty-queue()`|Θ(1)|Θ(1)|
|`Q.insert(x)`|Θ(lg n)|Θ(lg n)|
|`x = Q.deletemin()`|Θ(lg n)|Θ(lg n)|
|`x = Q.findmin()`|Θ(1)|Θ(lg n) → Θ(1)*|

- 在 AVL 維護一個指到最小鍵的指標即可達到 Θ(1)。


**Predecessor / Successor ADT（複雜度）**

|操作|heap|AVL tree|
|---|---|---|
|`S = new-empty()`|Θ(1)|Θ(1)|
|`S.insert(x)`|Θ(lg n)|Θ(lg n)|
|`S.delete(x)`|Θ(lg n)|Θ(lg n)|
|`y = S.predecessor(x)`（next-smaller）|Θ(n)|Θ(lg n)|
|`y = S.successor(x)`（next-larger）|Θ(n)|Θ(lg n)|
