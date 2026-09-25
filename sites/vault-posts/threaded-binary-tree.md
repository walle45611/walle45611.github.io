---
title: "引線二元樹 (Threaded Binary Tree)"
slug: threaded-binary-tree
topic_section: data-structures
description: "引線二元樹 (Threaded Binary Tree)的重點整理。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/引線二元樹 (Threaded Binary Tree).md"
---

### ✅ 背景說明

普通的二元樹有以下性質：

- 有 $n$ 個節點，就有 $n−1$ 條實際存在的 child links（邊）。
    
- 若每個節點使用指標儲存左右子節點，則需要 $2n$ 個指標。
    
- <mark>其中會有：$2n - (n - 1) = n + 1$ 個是 `null` 指標（空指標）。</mark>

👉 這些空指標可以被有效利用來儲存「**中序遍檢順序下的**先行者與**後繼者**」。

---

### 📌 中序線索二元樹規則（Threading Rules）

1. 若某節點的左子樹為空，則其 `left` 指標指向 **先行者（predecessor）**。
    
2. 若某節點的右子樹為空，則其 `right` 指標指向 **後繼者（successor）。

---

### 📆 Thread Binary Tree 節點結構

```
 ┌───────────────┬────────┬──────┬────────┬──────────────┐
 │  leaftthread  │ Lchild │ Data │ Rchild │ rightthread  │
 │     (bool)    │        │      │        │    (bool)    │
 └───────────────┴────────┴──────┴────────┴──────────────┘
```

使用額外兩個布林變數 `Lthread` 和 `Rthread` 來區分指標是「真實子節點」還是「線索」。

```cpp
struct ThreadNode {
    bool Lthread;          // True 表示 Lchild 是線索（非真子節點）
    ThreadNode* Lchild;

    int Data;

    ThreadNode* Rchild;
    bool Rthread;          // True 表示 Rchild 是線索（非真子節點）
};
```

#### 判斷規則：

- 若 `Lthread == false`，代表 `Lchild` 是左子樹。
    
- 若 `Lthread == true`，代表 `Lchild` 是先行者。
    
- 同理，`Rthread == true` 表示 `Rchild` 是後繼者。
    
---

#### 🔧 Head 節點（特殊節點）

Thread Binary Tree 中的 Head 節點具有特殊意義，負責協助非遞迴中序遍歷的開始與結束判斷。

##### 🔸 Case 1：空樹（empty）

若樹為空，Head 節點會指向自己：

```
Lthread = True          Rthread = False
     ↓                        ↓
 ┌────────┬────────┬──────┬────────┬────────┐
 │ True   │ Lchild │ Data │ Rchild │ False  │
 │        │   ↑    │      │   ↓    │        │
 └────────┴────────┴──────┴────────┴────────┘
          ↖───────────────┘
```

- `Lchild` 和 `Rchild` 都指回自己（Head）。
    
- `Lthread = True` 表示 `Lchild` 是線索（不是左子樹）。
    
##### 🔸 Case 2：非空樹（not empty）

若樹非空，Head 節點會指向實際的根節點：

```
Lthread = False         Rthread = False
     ↓                        ↓
 ┌────────┬────────┬──────┬────────┬────────┐
 │ False  │ Lchild │ Data │ Rchild │ False  │
 │        │   ↓    │      │        │        │
 └────────┴────────┴──────┴────────┴────────┘
          ↓
        root
```

- `Lchild` 指向整棵線索樹的根節點。
    
- 中序最左節點的前驅與最右節點的後繼會分別線索回 Head 節點。
        
#### 完整示意圖

![Threaded Binary Tree 完整示意圖](/vault-assets/a515d09e819cb509fd99.png)


---

### 📈 優點與用途

- 可進行 **非遞迴的中序遍檢**，不需使用額外 stack。
    
- 節省記憶體（充分利用原本的 null 指標）。
    
- 適合查找與遍檢頻繁的場景，如資料庫索引結構等。
    
---

### 🔄 中序後繼與中序遍歷（C++）

#### ✅ 中序後繼函式 `Insuc`

```cpp
ThreadNode* Insuc(ThreadNode* x) {
    ThreadNode* temp = x->Rchild;

    // 如果 Rthread == false，代表 x 有右子樹
    // 需要往右子樹中最左側節點尋找中序後繼
    if (x->Rthread == false) {
        // 沿著左子樹往下找，直到找到線索節點（即最左節點）
        while (temp->Lthread == false) {
            temp = temp->Lchild;
        }
    }

    return temp; // 若 x 是中序最後一個節點，這裡可能會回傳 head
}
```

#### ✅ 非遞迴中序遍歷 `InorderTraversal`

```cpp
void InorderTraversal(ThreadNode* head) {
    ThreadNode* temp = head;

    do {
        temp = Insuc(temp);
        if (temp != head)
            std::cout << temp->Data << " ";
    } while (temp != head);
}
```

#### 📘 補充說明

- 若某節點 `x` 的 `Rthread == false`，代表 `Rchild` 是右子樹，要往該子樹中最左節點找中序後繼。
    
- 若 `Rthread == true`，則 `Rchild` 已直接是中序後繼節點。
    
- `head` 是特殊的「頭節點」，用來標示起點與終點。
    
* 時間複雜度：每個節點都恰好拜訪一次 $O(N)$ ，中序遍歷不需要使用遞迴或額外 stack。

#### 🧐 圖解說明

![Threaded Binary Tree 示意圖](/vault-assets/d7a21fe029cbe99610ab.png)


以下是圖中的二元樹中序遍檢順序：

$H \rightarrow D \rightarrow I \rightarrow B \rightarrow E \rightarrow A \rightarrow F \rightarrow C \rightarrow G \rightarrow head$

---

### 引線二元樹插入右子

#### Case 1：`S` 原本沒有右子樹（`S.Rthread == true`）

![引線二元樹插入 case 1](/vault-assets/4fff67d262d6faa38c23.png)


#### Case 2：`S` 原本有右子樹（`S.Rthread == false`）

![引線二元樹插入 case 2](/vault-assets/c64eaad4f825717153df.png)


#### 實作程式碼

下面的程式碼依據中的 **①…⑤** 步驟編號對應：

```cpp
void InsertRight(ThreadNode* S, ThreadNode* t) {
    // ① 複製 S 的原右線索到 t
    t->Rthread = S->Rthread;
    t->Rchild  = S->Rchild;

    // ② 設定 t 的左線索為指向 S（先行者）
    t->Lthread = true;
    t->Lchild  = S;

    // ③ 把 t 掛為 S 的右子節點
    S->Rchild  = t;
    S->Rthread = false;

    // ④ 若 t 繼承到的是一棵真右子樹（原 S 有右子樹）
    if (!t->Rthread) {
        // ⑤ 找到原右子樹的最左節點 p，修正其先行者線索到 t
        ThreadNode* p = t->Rchild;
        while (!p->Lthread) {
            p = p->Lchild;
        }
        p->Lchild = t;
    }
}
```

1. **S 無右子樹 時**（原本 `S.Rthread == true`）
    
    - ①、②：先把 S 的後繼線索複製到 t，並讓 t 指回 S 作為先行者。
        
    - ③：把 t 掛為 S 的右子節點，並將 `S.Rthread = false`。
        
    - 因為原 S 沒有真右子樹，④ 條件不成立，不用做後續修正。
        
2. **S 有右子樹 時**（原本 `S.Rthread == false`）
    
    - ①~③：流程同上，先複製線索、掛載 t。
        
    - ④~⑤：因為繼承到的是一棵真右子樹，必須往該子樹最左邊一路走到底，找到節點 `p`，再把 `p->Lchild` 設為 `t`，維持中序先行者正確。

> **Note**: 若要將 T 插入 S 的左邊，將上述步驟中：
> 
> - `left`/`right` 意義互換
>     
> - `Insuc(t)` 改為 `Inpre(t)` 即可完成左插操作。
