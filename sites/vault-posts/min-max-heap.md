---
title: "最小最大堆積 (Min-Max Heap)"
slug: min-max-heap
topic_section: data-structures
description: "最小最大堆積 (Min-Max Heap)的重點整理。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/最小最大堆積 (Min-Max Heap).md"
---

### 定義

- 一個 **Min-Max Heap** 是一棵 **完全二元樹** (complete binary tree)，其第 $i$ 層為 **min-level** 或 **max-level**，並互相交錯 (alternating levels)
    
- **Root** 必然是全樹最小值
    
- 若節點 $x$ 在 **min-level**，則 $x$ 是其子樹的 **最小值**
    
- 若節點 $x$ 在 **max-level**，則 $x$ 是其子樹的 **最大值**
    
- 一個在 min-level 的節點稱為 **min node**，在 max-level 的節點稱為 **max node**
    
- 相對地，**max-min heap** 是 root 是最大值，子節點中有最小值
    
### 性質

- 因為是完全二元樹，可以用陣列形式 $A[1..N]$ 來表示
    
- 第 $i$ 個節點的父節點為 $\left\lfloor \frac{i}{2} \right\rfloor$，子節點為 $2i$ 和 $2i + 1$
    
### Min/Max 層級

- 使用 0-based 層級 index
    
    - 偶數層: $0, 2, 4, \dots$ 是 **min-level**
        
    - 奇數層: $1, 3, 5, \dots$ 是 **max-level**
        
### Min / Max 驗證

- 假設節點 $P$ 在 min-level，則  
    $P \le \min(\text{所有子孫節點的值})$
    
- 假設節點 $P$ 在 max-level，則  
    $P \ge \max(\text{所有子孫節點的值})$
    
### 插入操作（Insertion）

#### 步驟

1. 將 $x$ 插入在最後一個位置（即 $n+1$）
    
2. 設 $p = \left\lfloor \frac{n}{2} \right\rfloor$ 為 $x$ 的父節點
    
3. 根據 $p$ 所在層級分為兩種情況處理：
    
#### Case 1：父節點 $p$ 在 min-level

- 若 $x.key > H[p].key$，代表 $x$ 比父親大，不應留在 min-level：
    
    - 執行 `VerifyMax(H, n, x)`，檢查 $x$ 能否上升為 max-level 中的最大值
        
- 否則 $x$ 比父親小，有機會挑戰 min-level：
    
    - 令 $H[n] = H[p]$（父值下移），再執行 `VerifyMin(H, p, x)`
        
#### Case 2：父節點 $p$ 在 max-level

- 若 $x.key < H[p].key$，代表 $x$ 比父親小，不應留在 max-level：
    
    - 執行 `VerifyMin(H, n, x)`，檢查 $x$ 能否成為 min-level 中的最小值
        
- 否則 $x$ 比父親大，有機會挑戰 max-level：
    
    - 令 $H[n] = H[p]$，再執行 `VerifyMax(H, p, x)`
        

> ✅ 插入後，會根據父節點決定自己應該屬於 max-level 還是 min-level，然後只會在「自己的層級」中挑戰上去，保持那層的 heap 性質。

#### 時間複雜度

因為調整 min-level 或是 max-level 那麼他的複雜度為樹高 $O(h), \ h=\log(n)$ 所以爲 $O(\log n)$

#### 程式碼片段

```cpp
void VerifyMin(std::vector<int>& H, int index, int x) {
    while (index > 3) {
        int grandparent = index / 4;
        if (x < H[grandparent]) {
            H[index] = H[grandparent];
            index = grandparent;
        } else {
            break;
        }
    }
    H[index] = x;
}

void VerifyMax(std::vector<int>& H, int index, int x) {
    while (index > 3) {
        int grandparent = index / 4;
        if (x > H[grandparent]) {
            H[index] = H[grandparent];
            index = grandparent;
        } else {
            break;
        }
    }
    H[index] = x;
}

void insert(std::vector<int>& H, int x) {
    H.push_back(x); // 插入到最後一個位置
    int n = H.size() - 1;
    if (n == 1) return; // root 無需調整

    int p = n / 2;
    if (isMinLevel(n)) {
        if (x > H[p]) {
            VerifyMax(H, n, x);
        } else {
            H[n] = H[p];
            VerifyMin(H, p, x);
        }
    } else { // max level
        if (x < H[p]) {
            VerifyMin(H, n, x);
        } else {
            H[n] = H[p];
            VerifyMax(H, p, x);
        }
    }
}

bool isMinLevel(int index) {
    int level = 0;
    while (index > 1) {
        index /= 2;
        level++;
    }
    return level % 2 == 0;
}
```

---

### Delete-Min on Min-Max Heap

#### 操作目標：

> ✅ **刪除最小值時，會將最後一個節點的值 $x$ 補到 root，然後從 root 開始向下調整位置，直到找到 $x$ 合法的落點，確保整棵 Min-Max Heap 結構正確。**

從 Min-Max Heap 中**刪除最小值（即 root）**，並保持 heap 結構與 min-max 層級規則正確。

#### 步驟說明：

1. **移走 root 的資料值**（即最小值）。
    
2. **將最後一個節點的值（記作 $x$）暫時補到 root 上**，並將最後節點從陣列中刪除。
    
    - $x$ 是被移上來等待調整的新值，接下來的任務就是讓 $x$ 在 heap 中重新落位
        
3. 將 `x` 插入到 root 為起點的子樹中，視子節點情況分成 3 類：
    
#### Case 1：Root 無子節點

- 表示 heap 只剩一個節點，直接移除即可。
    
#### Case 2：Root 有子但無孫子節點（即 root 的 children 是葉節點）

- 找出 **root 兩個子節點中較小者 `k`**
    
- 若 `x.key > k.key`，則 `x` 與 `k` 交換位置，`x` 成為子節點
    
- 否則：`x` 留在 root
    
#### Case 3：Root 有孫子存在（完整三層以上）

- 找出整個子孫中 **最小值所在的節點 `k`**
    
- 設 `p` 為 `k` 的父節點
    
##### 接下來判斷：

- 如果 `x.key <= k.key`：
    
    - `x` 就留在 root 上（已經比全部子孫都小）
        
- 否則：
    
    - 將 `k` 放上 root
        
    - 接著：如果 `x > p.key`，表示 `x` 不適合當 max-level 子 ⇒ 交換 `x` 與 `p` 再繼續操作
        
    - 否則：將 `x` 放到 `k` 位置結束
        

> 💡 重點：因為 min-max heap 層級交錯，若 `x` 被推到孫子層，可能會違反該層級的 min/max 性質，所以要再與其父 `p` 檢查一次

- 整體類似於 min-heap 的 delete-min，但需額外考慮 **min-max 層級正確性**
    
- 若 `x` 被放到 max-level，要保證它不是比其父還大（因為那是 min 層）

#### 時間複雜度

如果已 case 3 討論，那麼從孫子節點中找到最大或是最小，所以基本就是找到樹的每層 $h/2$ 所以基本是樹高的 $O(\log n)$

#### C++ 程式碼片段：Delete-Min 操作

```cpp
void deleteMin(std::vector<int>& H) {
    int n = H.size() - 1;
    if (n == 0) return; // 空 heap

    int minValue = H[1];
    int x = H[n]; // 取最後一個值
    H.pop_back();
    if (n == 1) return; // 只剩一個元素被刪除完就結束

    int index = 1;

    while (true) {
        int m = -1; // 最小孫子或子的位置
        int minKey = x;

        // 找出所有子與孫子中的最小值位置
        for (int i = 2 * index; i <= std::min(2 * index + 1, n); ++i) {
            if (i <= n && H[i] < minKey) {
                minKey = H[i];
                m = i;
            }
        }
        for (int i = 4 * index; i <= std::min(4 * index + 3, n); ++i) {
            if (i <= n && H[i] < minKey) {
                minKey = H[i];
                m = i;
            }
        }

        if (m == -1 || x <= H[m]) {
            H[index] = x;
            break;
        }

        H[index] = H[m];
        int parent = m / 2;

        if (x > H[parent]) {
            std::swap(H[m], H[parent]);
            index = parent;
        } else {
            index = m;
        }
    }
}
```


---

### 範例樹（插入後驗證）

```
              8                ← Min level (root)
         /         \
       31          16         ← Max level
     /    \       /    \
   46     51    13     71     ← Min level
```

- Root (8) 是全樹最小值 ✅
    
- 31, 16 在 max-level：
    
    - 31 > 46, 51 ✅
        
    - 16 < 71 ❌（無法成為子樹最大值）
        
- 46, 51, 13, 71 為 min-level 節點
    

### 最大值位置

- 由於 root 是 min-value，**max-value 會出現在 root 的子節點中**，即 $A[2]$ 與 $A[3]$
    $$\text{max\_value} = \max(A[2], A[3])$$
