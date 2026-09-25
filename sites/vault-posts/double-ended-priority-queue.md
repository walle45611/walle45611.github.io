---
title: "雙端堆或是雙端優先隊列 (Double-ended Heap OR  double-ended priority queue，DEPQ)"
slug: double-ended-priority-queue
topic_section: data-structures
description: "My vault 資料結構筆記：雙端堆或是雙端優先隊列 (Double-ended Heap OR  double-ended priority queue，DEPQ)。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/雙端堆或是雙端優先隊列 (Double-ended Heap OR  double-ended priority queue，DEPQ).md"
---

### ✅ 定義規則

1. **The root contains no element**
    
2. **The left subtree is a Min-Heap**
    
3. **The right subtree is a Max-Heap**
    
4. **Constraint between the two trees**：
    
    - 令 $i$ 為左子樹中的任意節點，$j$ 為右子樹中對應的節點。
        
    - 若 $j$ 不存在（即 $j > n$），則 $j$ 的對應節點為其 **父節點**。
        
    - 則必須滿足：$i.\text{key} \leq j.\text{key}$

> **資訊**
> 是一棵 Complete Binary Tree
> 

---

### 🧲 對應節點計算公式

**左側對應節點（Min‐partner）**  
給定一個位於 **max‐heap** 中的節點索引 `n`（`n ≥ 2`），對應到 **min‐heap** 的索引 `i`：  
$$
i = \mathrm{min\_partner}(n)
  = n \;-\; 2^{\,\lfloor \log_{2} n\rfloor - 1}.
$$
- 若計算出來的 `i` 超過 `heap_size`，則  
  $$
  i = \Big\lfloor \frac{i}{2} \Big\rfloor.
  $$

**右側對應節點（Max‐partner）**  
給定一個位於 **min‐heap** 中的節點索引 `n`（`n ≥ 2`），對應到 **max‐heap** 的父節點索引 `j`：  
$$
j = \mathrm{max\_partner}(n)
  = i+ 2^{\lfloor \log_2 i \rfloor -1}
  \quad(\text{整數除法})
$$
- 若計算出來的 `j` 超過 `heap_size`，則  
  $$
  j = \Big\lfloor \frac{j}{2} \Big\rfloor.
  $$


---
### Insert X on the DEAP

#### 步驟流程

1. 將 x 插入於最後一個節點（即 n+1 位置）
    
2. 分成兩種情況討論：
    
#### Case 1：x 位於 Min-Heap（左子樹）

1. 找出其在 Max-Heap（右子樹）之對應節點 j
    
2. 判斷：
    
    - 如果 x.key>D[j].keyx.key > D[j].key
        
        ```
        D[x] := D[j];
        MaxHeapInsert(D, j, x);
        ```
        
    - 否則：
        
        ```
        MinHeapInsert(D, x, x);
        ```
        
#### Case 2：x 位於 Max-Heap（右子樹）

1. 找出其在 Min-Heap（左子樹）之對應節點 j
    
2. 判斷：
    
    - 如果 x.key<D[j].keyx.key < D[j].key
        
        ```
        D[x] := D[j];
        MinHeapInsert(D, j, x);
        ```
        
    - 否則：
        
        ```
        MaxHeapInsert(D, x, x);
        ```
        

> **簡單來說**
> 如果今天插入在 max-heap(左子樹)的話就去檢查 min-heap 對應的位置 $i$ 是不是比較大如果比較大就交換執行 Max-Heap 的插入調整，反之亦然

#### 時間複雜度

- $O(\log n)$，主要耗費在 min-heap 或 max-heap 的 insert 操作。

#### DEAP 插入範例修正：插入 4 與插入 60

##### ✅ 插入 4（x < H[j]，插入於 Max-Heap，與 Min-Heap 對應節點交換，進行 MinHeap 調整）

###### ⬇️ 初始結構（插入 4）

```text
           o
        /     \
      5         45
    /   \     /   \
  10     8   25     40
 /  \        /  \
15  19      20   x(4)
```

- 插入的節點 4 落在 Max-Heap（右子樹）
    
- 對應的 Min-Heap 節點為 19
    
- 因 4 < 19，違反 DEAP 規則（左子樹 ≤ 右子樹）
    

###### 🔁 執行 swap（將 4 與 19 對調）

```text
           o
        /     \
      5         45
    /   \     /   \
  10     8   25     40
 /  \        /  \
15   4      20   19
```

###### ⬆️ 將 4 插入 Min-Heap → MinHeap Insert 調整

```text
ＺＺ
```

##### ✅ 插入 60（x > H[j]，插入於 Max-Heap，與 Min-Heap 對應節點交換後，進行 MaxHeap 調整）

###### ⬇️ 初始結構（插入 60）

```text
           o
        /     \
      4         45
    /   \     /   \
   5     8   25     40
 /  \        /  \     
15  10      20   19
                          x(60)
```

- 插入的節點 60 落在 Max-Heap（右子樹）
    
- 對應 Min-Heap 的節點為 40
    
- 因 60 > 40，符合方向，直接插入 MaxHeap
    
###### 🔁 執行 swap（與 40 對調）

```text
           o
        /     \
      4         60
    /   \     /   \
   5     8   25     45
 /  \        /  \    \
15  10      20   19   40
```

###### ⬆️ 將 60 插入 Max-Heap → MaxHeap Insert 調整

- 60 > 45 → 上浮
    

```text
           o
        /     \
      4         60
    /   \     /   \
   5     8   25     45
 /  \        /  \    \
15  10      20   19   40
```

### DEAP Delete Min 

1. **移除最小元素**  
   - 將 `D[2]`（deap 的最小值）取出，留下空缺位置 `i = 2`。

2. **取出最後一個節點**  
   - 將陣列末端節點 `x` 取出並刪除，heap size 減 1。

3. **填補空缺（在 min-heap 中上移）**  
   ```text
	while i 有子節點:
	   c = 左右孩子中值較小者
	   if D[c] < D[i]:
		   D[i] = D[c]
		   i = c
	   else:
		   break
	```

4. **重插節點 `x`（DeapInsert）**
    
    ```text
    DeapInsert(D, i, x)
    ```
    
    - 將 `x` 放入空缺的葉節點位置 `i`，再依序做上浮或下沉，以恢復 deap 的雙堆序性。
#### 範例刪除

```
                  (root)
               /         \
             [ 5 ]      [ 70 ]
            /    \      /    \
         [15]   [10]  [60]   [40]
         / \     / \   /
      [20][25][30][35][50]
```

```
                  (root)
               /         \
             [ 10 ]      [ 70 ]
            /    \      /    \
         [15]    []  [60]   [40]
         / \     / \   /
      [20][25][30][35][]
```

```
                  (root)
               /         \
             [ 10 ]      [ 70 ]
            /    \      /    \
         [15]    [30]  [60]   [40]
         / \     / \ 
      [20][25]  [][35]
```

```
                  (root)
               /         \
             [ 10 ]      [ 70 ]
            /    \      /    \
         [15]    [30]  [60]   [40]
         / \     / \ 
      [20][25]  [50][35]
```

```
                  (root)
               /         \
             [ 10 ]      [ 70 ]
            /    \      /    \
         [15]    [30]  [60]   [50]
         / \     / \ 
      [20][25]  [40][35]
```

1. 刪除 5 所以， x = 50，然後 5 調整 10 跟 15 比較 所以 10 ， 再來空缺是  30 和 35 比較 30 比較小，現在插入 50 在空缺的位置，然後調整 50 對應的位置是 40 所以交換，所以 

#### 時間複雜度分析

- deap-delete-min 無論最後一個節點原本是在 min-heap 還是 max-heap，都能正確運作。
    
- 由於 deap 的高度為 $O(\log n)$，整體時間複雜度為 **$O(\log n)$**。
