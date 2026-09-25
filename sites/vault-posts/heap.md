---
title: "堆積 (Heap)"
slug: heap
topic_section: data-structures
description: "My vault 資料結構筆記：堆積 (Heap)。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/堆積 (Heap).md"
---

- Heap 是一種 **Complete Binary Tree**
	
- 每個父節點一定大於或等於左右子節點（Max-Heap）
	
- 預設為 Max-Heap（你有註記）
	
- Root 是整棵樹中最大值
	
- 適合用陣列儲存（Array 表示）

---
### Bottom Up (heapify down)

#### Heap Adjust (heapify) 和 build

-  **Heapify（上浮）**：向上與 parent 比較
	
	- 若比 parent 大 → 交換 → 繼續上浮
		
	- 一路「挑戰」到 root 為止，或失敗為止
		
	- `x 往下 parent 挑戰直到 root 或失敗`

```c
// Max-Heap 的調整與建堆函式

// 調整以 i 為根的子堆，讓它符合 Max-Heap 規則（Heapify Down）
void adjust(int tree[], int i, int n) {
    int j = 2 * i;         // j 是左子節點
    int x = tree[i];       // 暫存當前節點的值

    while (j <= n) {
        // 若右子節點存在且比左子大，選右子
        if (j < n && tree[j] < tree[j + 1])
            j = j + 1;

        // 若已滿足 heap 性質（父比子大），就停止
        if (x >= tree[j])
            break;

        // 較大的子節點上移
        tree[j / 2] = tree[j];
        j = 2 * j;         // 繼續往下比較
    }

    // 將原值放到正確位置（j 已經超過邊界或比子大）
    tree[j / 2] = x;
}

// 建立 Max-Heap：從 n/2 就是父節點開始往上對每個節點做 adjust
void buildheap(int tree[], int n) {
    for (int i = n / 2; i >= 1; i--) {
        adjust(tree, i, n);
    }
}
```

##### 建立 Heap 的時間複雜度推導

- 前提假設：
		
	- 假設我們有一棵 **完全二元樹（Complete Binary Tree）**，有 $n$ 個節點， 樹的高度為 $k = \lceil \log(n+1) \rceil$。
		
	- 每個節點最多需要從目前位置下移（sift-down）到最底層，最多 $k - i$ 次比較（其中 $i$ 是該節點的層級，最底層為 $k$）。
	
- 層級資訊與每層節點數：
		
	- 第 $i$ 層（由上至下編號）最多有 $2^{i-1}$ 個節點。
		
	- 該層每個節點最多花費 $k - i$ 的時間進行 sift-down。
	
- 時間總和推導：
		
	- 令總時間為 $S$，我們將每層的花費總和加總：
		$$
		S = \sum_{i=1}^{k} 2^{i-1} \cdot (k - i)
		$$
	- 我們令：
		$$
		S = 2^0 \cdot (k - 1) + 2^1 \cdot (k - 2) + 2^2 \cdot (k - 3) + \cdots + 2^{k-2} \cdot 1 + 2^{k-1} \cdot 0
		$$
	
		- 令此為式 $(1)$
	
	- 再令：
		$$
		2S = 2^1 \cdot (k - 1) + 2^2 \cdot (k - 2) + \cdots + 2^{k-1} \cdot 1 + 2^{k} \cdot 0
		$$
	
		- 令此為式 $(2)$
	
	- $(2)$ 減 $(1)$ 得：
		$$
		S = -2^0 \cdot (k - 1) + 2^1 + 2^2 + \cdots + 2^{k-1}
		$$
	
	- 使用等比數列求和公式：
		$$
		\sum_{i=0}^{k-1} 2^i = 2^k - 1
		$$
	
	- 所以：
		$$
		S = - (k - 1) + (2^k - 1) = 2^k - k
		$$
	
	- 因為完全二元樹中最多有 $n = 2^k - 1$ 個節點，推出：
		$$
		S \leq n - \log_2(n+1) + 1
		$$
	- 所以時間複雜度為：
		$$
		\boxed{O(n)}
		$$


---
#### 插入操作：Insert X

1. **插入位置**：將新元素 `x` 插入至陣列最後一個位置（即最後一個節點）
		
	- `Insert: x 插到最後一個 the last node`
		
	- 例：目前 n = 10，則插入 `tree[11]`
		
2. **執行 Adjust (Heapify Up)**：
		
	- 插入後從該節點往上比較其父節點
		
	- 若 `x > parent`（Max-Heap），則交換並繼續往上
		
	- 一直進行到 root 或符合 heap 規則為止
		
3. **時間複雜度說明**：
		
	- Heap 是 **完全二元樹（Complete Binary Tree）**，節點數為 $n$ 時，高度為 $\lceil log₂(n + 1) \rceil$
		
	- 最差情況下，插入的元素會從最底層一路上浮到 root（即最大移動距離）
		
	- ⏱ 因此插入操作的時間複雜度為：**$O(\log n)$**

```c
// 插入新元素 x 到 Max-Heap
void insert(int tree[], int x, int *n) {
    int size = ++(*n);             // 增加 heap 大小
    tree[size] = x;                // 先插入最末尾
    int child = size;
    int parent = child / 2;

    // 若 x 大於 parent，則一路往上浮（heapify up）
    while (parent >= 1) {
        if (x > tree[parent]) {
            tree[child] = tree[parent];  // 父節點下移
            child = parent;
            parent = parent / 2;
        } else {
            break;                       // 若已滿足 heap 規則則停止
        }
    }

    tree[child] = x;                // 將 x 放入正確位置
}
```

---

#### 🔴 刪除最大值操作：Delete-Max（Max-Heap）

1. **刪除 Root**：
		
	- 將根節點（最大值）移除，暫存回傳
		
2. **替換 Root**：
		
	- 將最右下角節點（即最後一個元素）補上 root 位置
		
	- 原本位置刪除（heap size - 1）
		
3. **向下調整 Heap（Heapify Down）**：
		
	- 從 root 開始與左右子節點比較
		
	- 若不符合 Max-Heap 規則，則與較大者交換
		
	- 持續向下直到符合規則或成為葉節點為止
	
4. ⏱ 時間複雜度分析：
	
	- 最差情況下，**補上來的值會一路從 root 下沉到 leaf**
		
	- Heap 是 **完全二元樹（Complete Binary Tree）**，所以高度為 $\lceil log₂(n + 1) \rceil$
		
	- 因此刪除操作的時間複雜度為：**$O(\log n)$**


```c
// 刪除最大值（Root），並進行調整
int delmax(int tree[], int *n) {
    int max = tree[1];             // 取出 root 作為最大值
    int size = *n;

    tree[1] = tree[size--];        // 用最後一個節點補上 root
    *n = size;                     // 更新 heap 大小

    adjust(tree, 1, size);         // 從 root 開始往下調整（heapify down）
    return max;
}
```

---

### Top Down (flowing up)

- 前提假設：
		
	- Top-Down 是指從空的 heap 開始，**一筆一筆將資料插入**，每次插入都進行上浮調整（heapify-up）。
		
	- 每筆資料插入時，會從陣列尾端插入，然後與父節點比較並進行交換，直到符合 heap 性質。
		
	- 若有 $n$ 筆資料，則會執行 $n$ 次插入與上浮操作。

- 單次插入的時間分析：
		
	- Heap 是完全二元樹，最壞情況下一個元素從最底層往上走到 root。
		
	- 高度為 $\lceil \log(n+1) \rceil$，因此**單次插入的時間複雜度為**：$O(\log n)$

- 總插入時間推導：
		
	- 第 $i$ 筆資料插入時，最壞情況需要 $O(\log i)$ 時間。
		
	- 所以總時間為：
		$$
		T(n) = \sum_{i=1}^{n} \log i = \log(1) + \log(2) + \cdots + \log(n) = \log(n!)
		$$
		
	- 使用斯特林公式近似估計：
		$$
		\log(n!) = \Theta(n \log n)
		$$
		
	- 結論：Top-Down 建堆的總時間複雜度為：
		$$
		\boxed{O(n \log n)}
		$$
	
---

### Top-Down vs Bottom-Up 建堆方法比較

| 項目       | Bottom-Up（heapify down）                     | Top-Down（flowing up）          |
| -------- | ------------------------------------------- | ----------------------------- |
| 建堆方式     | 所有資料先放入陣列，從 $n/2$ 開始往上調整，簡單來說就是把二元樹建立好然後再調整 | 每次插入一筆資料，從尾端上浮，間單來說就是每次都調整    |
| 調整方向     | 向下調整（sift-down）                             | 向上調整（sift-up）                 |
| 每筆處理時間   | 根據層數而定，越底層越少                                | 最壞 $O(\log n)$                |
| 總建堆時間複雜度 | $O(n)$                                      | $O(n \log n)$                 |
| 優點       | 建堆效率極高，適合一次性整理大量資料                          | 程式簡單，適合動態逐筆處理                 |
| 缺點       | 難以用於線上插入場景                                  | 效率較差，重複調整次數多                  |
| 實作對應操作   | `buildheap()` + `adjust()`                  | `insert()` + `heapify up`     |
| 適用情境     | 大量資料批次建堆時，如：Heap Sort 初始堆建立                 | 資料逐筆產生時，如：Priority Queue 動態插入 |

---
###  常見操作與時間複雜度

| 操作說明                          | 時間複雜度      |
| ----------------------------- | ---------- |
| Insert X                      | $O(log n)$ |
| Delete-Max                    | $O(log n)$ |
| Heapify (or Adjust)           | $O(log n)$ |
| Find-Max                      | $O(1$)     |
| Build a Heap                  | $O(n)$     |
| Decrease key (Min-Heap)       | $O(log n)$ |
| Merge two heaps into one heap | $O(n)$     |
| Search X                      | $O(n)$     |

---
### 合併兩個 Heap 的方法：

  假設我們有兩個 Heap：
- H₁ 有 $n$ 個元素
- H₂ 有 $m$ 個元素

- 步驟如下：
		
	1. 將兩個 Heap 的資料依序放入新的陣列（時間 $O(n + m)$）
		  
	2. 對新的陣列使用 Bottom-Up 的 `buildheap()` 演算法來調整成合法 Heap

- 時間複雜度分析：
		
	- 將兩堆資料合併為一個陣列：$O(n + m)$
		
	- 使用 Bottom-Up 建堆：$O(n + m)$
		
	- 所以整體時間為：$O(n + m)$

- 結論：
		
	合併兩個 Heap 的方式是「直接合併成一個陣列後再建堆」，不需要逐一插入（否則會是 $O((n + m) \log(n + m))$）。
		
  ✅ 這種方法能保持時間複雜度為：$\boxed{O(n + m)}$
