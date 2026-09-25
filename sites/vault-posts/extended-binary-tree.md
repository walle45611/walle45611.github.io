---
title: "延伸二元樹 (Extended Binary Tree)"
slug: extended-binary-tree
topic_section: data-structures
description: "My vault 資料結構筆記：延伸二元樹 (Extended Binary Tree)。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/延伸二元樹 (Extended Binary Tree).md"
---

### 📌 定義

Extended Binary Tree 是一種擴充的二元樹，其特點如下：

> 在原本的 Binary Tree 中，凡是遇到 null subtree（即原本沒有左子或右子的地方），就插入一個特殊節點（special node）補足，使每個內部節點都有兩個子節點。

#### 🧠 節點分類

- **Internal Nodes（內部節點）**
    
    - 來自原本二元樹的節點（非葉節點）
        
    - 可能有左右子節點
        
- **External Nodes（外部節點）**
    
    - 新增的特殊節點，用來替代 null 子樹
        
    - 全部都是葉節點
        

#### 🍃 節點特性

- 所有 **external nodes** 一定是 **leaf nodes**（葉節點）
    
- 所有 **internal nodes** 一定是 **non-leaf nodes**（非葉節點）
    

#### 📐 關鍵性質公式

> **筆記**
> 延伸二元樹中：
>  $N_0 = N_2+1$

- ：外部節點數（External Nodes）
	  
- ：內部節點數（Internal Nodes）

✅ 這個公式說明，在任何一棵 extended binary tree 中，**外部節點數會永遠比內部節點數多 1 個。**


### 🔁 Internal Path Length vs External Path Length

#### 📌 定義

- **Internal Path Length（I）**：
	- 白話文就是 internal node 到 root 的路近加總
    - 定義為：  
        $I(T) = \sum_{i=1}^{n} \text{(從 root 到第 } i \text{ 個內部節點的 path 長度)}$
        
- **External Path Length（E）**：
	- 白話文就是 external 到 root 的路近加總
    - 定義為：  
        $E(T) = \sum_{j=0}^{n} \text{(從 root 到第 } j \text{ 個外部節點的 path 長度)}$
        
#### 📐 關係公式：

對於一棵有 $n$ 個內部節點的 extended binary tree：

$$E=I+2n$$

##### 🧪 證明方式：數學歸納法

1. **Base Case**：
	$n = 0 \ \text{代表內部 node=0 也表示 empty tree}$
	$I = E = 0 \quad \Rightarrow \quad E = I + 2n = 0 + 0 = 0 \quad \checkmark$

2. **Inductive Hypothesis**：  
	
   假設 $k < n$ 時成立，即： $E = I + 2k$
	
3. **Inductive Step**：
	
   - 將樹切為三部分：root、左子樹 $L$、右子樹 $R$
     
   - 設左子樹有 $n_l$ 個 internal nodes，右子樹有 $n_r$ 個，則：$n = n_l + n_r + 1$
     
   - 則： 
$$
	 \begin{aligned}
	 I(T) &= I(L) + n_l + I(R) + n_r \\
	 E(T) &= E(L) + (n_l + 1) + E(R) + (n_r + 1) \\
		  &= E(L) + E(R) + 2 + n_l + n_r \\
		  &= (I(L) + 2n_l) + (I(R) + 2n_r) + 2 + n_l + n_r \\
		  &= I(L) + I(R) + n_l + n_r + 2(n_l + n_r + 1) \\
		  &= I(T) + 2n
	 \end{aligned}
$$

   所以對 $n$ 也成立，證畢。✅

#### 🧪 範例：具有 $n$ 個 internal nodes 的 Extended Binary Tree（極端情況）

考慮一棵 **skewed** 的 extended binary tree（即所有節點都只有一個子節點，形成單邊直線），如下：

```
0
 \
  1
   \
    2
     \
     ...
       \
        n-1
```

##### 📌 Internal Node 總深度 $I$：

所有 internal nodes 的深度為：
$$I = 0 + 1 + 2 + \cdots + (n-1) = \frac{n(n-1)}{2}$$

##### 📌 External Node 總深度 $E$：

每個 internal node 都會補上一個 external node（即 null 子樹），再加上最後那個 null 也會變 external node，所以 external nodes 總數為：

$$E_{count} = n + 1$$

每個 external node 的深度會是其父節點的深度加一。

由於 Internal nodes 的總深度為：

$$I = 0 + 1 + 2 + \cdots + (n-1) = \frac{n(n-1)}{2}$$

因此 External nodes 的總深度為：

$$E = I + n = \frac{n(n-1)}{2} + n = \frac{n(n+1)}{2}$$


##### ✅ 總結：

$$\text{Internal nodes 總深度} = I = \frac{n(n-1)}{2}$$
$$\text{External nodes 總深度} = E = \frac{n(n+1)}{2}$$

這個例子顯示了在「最壞情況」（完全偏斜的樹）下的節點深度總和成長速度為二次方等級。

---
#### 📘 延伸應用

##### 🧪 WEPL（Weighted External Path Length）

當 external node 帶有權重 $p_i$ 時，其加權外部路徑長定義為：

$$\text{WEPL} = \sum_{i=1}^{n} (\text{path length from root to external node } i \times p_i)$$

給定權重集合：$W={2,4,5,15}$

###### (a) 範例樹：

```
        ○ ---- 0
       /
      ○ ---- 1
     / \
    ○   5 ---- 2
   / \
  2   4 ---- 3
```

- 加權計算：$\text{WEPL}_a = 3 \times 2 + 3 \times 4 + 2 \times 5 + 1 \times 15 = 6 + 12 + 10 + 15 = 43$

###### (b) 範例樹：

```
        ○ ---- 0
       / \
     ○     ○ ---- 1
    / \   / \
   2   4 5  15 ---- 2
```

- 加權計算：$WEPL_b=2×2+2×4+2×5+2×15=4+8+10+30=52$

✅ **結論**：  
即使 (b) 的樹「高度較小」，但 WEPL 更大。說明「樹高小」不一定代表「WEPL 小」。

##### 🔍 霍夫曼編碼（Huffman Encoding）

Huffman(W: weight values of nodes, N: number of nodes)

> **Assume**: W is maintained as a **MinHeap**

```c
for i = 1 to (N - 1) do {
    new(t);
    t->leftchild  = Del-min(W);
    t->rightchild = Del-min(W);
    t->weight = t->leftchild->weight + t->rightchild->weight;
    Insert(W, t);
}
```

###### 🧠 說明：

- 每次迴圈都會選出 **兩個最小權重** 的節點合併成一個新節點，並重新放回 MinHeap 中。
    
- 合併的節點成為新節點的左右子節點，其權重為左右子節點權重和。
    
- 最後形成的樹為 **Huffman Tree**，可最小化 WEPL（加權外部路徑長）。
    

###### ⏱️ Time Complexity：

- 迴圈執行 $(N - 1)$ 次
    
- 每次包含：
    
    - 兩次 `Del-min(W)` → $O(\log N)$
        
    - 一次 `Insert(W, t)` → $O(\log N)$
        
- 所以總時間複雜度為：
    $$O(Nlog⁡N)O((N - 1) \cdot \log N) = O(N \log N)$$

> **筆記**
>  Huffman Tree 一定是 **strict binary tree**（每個 internal node 恰有兩個子節點）
> 有些定義版本會稱其為 complete binary tree，但實際結構不一定符合 complete binary tree 的排列特性。

###### 範例：Huffman 編碼實作

根據下表提供的字元頻率，使用 Huffman 編碼來解以下問題：

|Character|Frequency|
|---|---|
|A|22|
|B|3|
|C|10|
|D|5|
|E|60|

```
             [100]
         0  /     \ 1
         [40]      E(60)
       0 /    \ 1
     [16]     A(22)
  0  /    \ 1
   [8]    C(10)
0 /   \ 1
B(3) D(5)
```

| 字元  | 編碼   |
| --- | ---- |
| A   | 01   |
| B   | 0000 |
| C   | 001  |
| D   | 0001 |
| E   | 1    |

> 題目指定：D 的編碼為 `0001`，符合推導。
