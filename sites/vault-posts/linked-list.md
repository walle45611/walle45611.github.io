---
title: "鏈結串列（Linked List）"
slug: linked-list
topic_section: data-structures
description: "My vault 資料結構筆記：鏈結串列（Linked List）。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Link List.md"
---

## 1. 基本概念與記憶體配置

Array (陣列) 是連續記憶體，優點是隨機存取 ($O(1)$)，缺點是插入刪除要搬移資料 ($O(n)$)。

Linked List 是非連續記憶體，透過 Pointer (指標) 串接。

- **優點**：插入、刪除只需修改指標，不需要搬移資料 ($O(1)$，前提是已知位置)。
    
- **缺點**：不支援隨機存取 (Random Access)，必須從頭走訪 ($O(n)$)；每個節點多一個 Pointer 的空間浪費。
    

---

## 2. 單向連結串列 (Singly Linked List, SLL)

### 結構定義

```
typedef struct Node {
    int data;
    struct Node *next; // 指向下一個節點
} Node;
```

### 關鍵操作詳解

#### A. 插入 (Insert)

要在節點 `x` 後面插入新節點 `new_node`：

1. `new_node->next = x->next;` (新節點先抓住後面的人)
    
2. `x->next = new_node;` (前面的人再放手去抓新節點)
    

- **口訣**：**先連後，再斷前** (避免斷鍊找不到後面的資料)。
    

#### B. 刪除 (Delete)

要刪除 `x` 後面的節點 `del_node`：

1. `del_node = x->next;` (先標記要刪除的人)
    
2. `x->next = del_node->next;` (跳過 `del_node`，直接連下下一個)
    
3. `free(del_node);` (釋放記憶體)
    

#### C. 反轉 (Reverse) ★★★ (考試必考，一定要會寫)

題目：原地反轉一個 SLL。需要三個指標：`prev` (前), `curr` (現), `next` (後)。

**演算法步驟**：

1. 初始：`prev = NULL`, `curr = head`
    
2. 迴圈 (`while curr != NULL`)：
    
    - `next = curr->next` (先保留後面，不然會斷鍊)
        
    - `curr->next = prev` (**關鍵動作**：將箭頭反指)
        
    - `prev = curr` (指標整體往右移)
        
    - `curr = next` (指標整體往右移)
        
3. 結束：`head = prev` (最後 `prev` 會停在原本的尾巴，現在的新頭)。
    

---

## 3. 雙向連結串列 (Doubly Linked List, DLL)

### 結構定義

```
typedef struct DNode {
    int data;
    struct DNode *LLink; // Left Link (Prior)
    struct DNode *RLink; // Right Link (Next)
} DNode;
```

### 特性

- 空間換時間：每個節點多一個 Pointer。
    
- 支援雙向走訪。
    
- **刪除優勢**：在 SLL 中，要刪除 `curr` 必須知道 `prev`；但在 DLL 中，因為 `curr` 知道 `curr->LLink`，所以刪除任意節點是 $O(1)$。
    

### 插入與刪除 (指標操作比較繁瑣，容易錯)

假設要在 `x` 後面插入 `new`：

1. `new->LLink = x;`
    
2. `new->RLink = x->RLink;`
    
3. `x->RLink->LLink = new;` (注意：如果 x 是最後一個節點，這行會出錯，要檢查 NULL)
    
4. `x->RLink = new;`
    

---

## 4. 環狀連結串列 (Circular Linked List)

### 結構

- **Single Circular**：最後一個節點的 `next` 指回 `Head`。
    
- **Double Circular**：Head 的 `LLink` 指向 Tail，Tail 的 `RLink` 指向 Head。
    

### 考試重點

1. **如何判斷結束**：迴圈條件不再是 `ptr != NULL`，而是 `ptr != Head`。
    
2. **串接 (Concatenation) 的優勢**：
    
    - 若我們只有指向 Head 的指標，串接兩個串列需要走訪到尾巴，耗時 $O(n)$。
        
    - **技巧**：若 Circular List 使用 **指向尾部 (Rear Pointer/Tail Pointer)** 來代表整個串列。
        
        - `Rear->next` 就是 Head。
            
        - 串接 List A 和 List B ($O(1)$)：
            
            1. `Temp = RearA->next` (存 A 的頭)
                
            2. `RearA->next = RearB->next` (A 尾連 B 頭)
                
            3. `RearB->next = Temp` (B 尾連 A 頭)
                
            4. `RearA = RearB` (新的尾巴是 B 的尾巴)
                

---

# 第二部分：General List (一般化串列) —— 遞迴與邏輯

這部分是 Lisp/Scheme 等函數式語言的基礎，資料結構考的是**定義與 Head/Tail 操作**。

## 1. 定義

一般化串列 $L$ 定義為有限序列 $L = (e_1, e_2, \dots, e_n)$，其中 $e_i$ 可以是：

- **Atom (原子)**：單純的資料 (如整數、字元)。
    
- **List (子串列)**：另一個一般化串列。
    
- 這是一個**遞迴 (Recursive)** 的定義。
    

## 2. 結構實作

通常使用 `tag` 欄位來區分：

```
struct Node {
    int tag; // 0: Atom, 1: List
    union {
        char data;       // Atom 的值
        struct Node *dlink; // 指向子串列 (Down Link)
    };
    struct Node *rlink;  // 指向同層下一個節點 (Right Link)
};
```

## 3. Head 與 Tail 的運算 (考試必考，切記規則)

- **Head(L)**：取出 L 的**第一個元素**。 (結果可能是 Atom，也可能是 List)。
    
- **Tail(L)**：取出 L **除去第一個元素後，剩下的所有元素形成的 List**。 (結果**一定**是 List，也就是外面會多一層括號)。
    

### 實戰演練 (重要)

令 $L = (A, (B, C), D)$

1. **Head(L)**
    
    - 第一個元素是 $A$。
        
    - 結果：$A$ (Atom)。
        
2. **Tail(L)**
    
    - 拿掉 $A$，剩下 $(B, C)$ 和 $D$。
        
    - 把它們包成一個 List。
        
    - 結果：$((B, C), D)$。
        
3. **Head(Tail(L))**
    
    - 先做 Tail(L) $\rightarrow ((B, C), D)$。
        
    - 對這個結果取 Head $\rightarrow$ 第一個元素是 $(B, C)$。
        
    - 結果：$(B, C)$。
        
4. **Tail(Head(Tail(L)))**
    
    - Head(Tail(L)) 是 $(B, C)$。
        
    - 對 $(B, C)$ 取 Tail $\rightarrow$ 拿掉 $B$，剩下 $C$，包成 List。
        
    - 結果：$(C)$。 **注意！不是 C，是 (C)。**
        
5. **Head(Tail(Head(Tail(L))))**
    
    - Tail(Head(Tail(L))) 是 $(C)$。
        
    - Head((C)) $\rightarrow$ 第一個元素是 $C$。
        
    - 結果：$C$。
        

---

# 第三部分：Sparse Matrix (稀疏矩陣) —— 空間壓縮

當矩陣中大部分元素為 0 時，用二維陣列 `A[m][n]` 儲存太浪費空間，改用只存「非零元素」的方法。

## 1. 3-Tuple (三元組 / Array Term) 表示法

用一個陣列 Terms 儲存，每個元素包含 (row, col, value)。

通常 `Terms[0]` 不存資料，而是存 metadata：Total_Rows, Total_Cols, Total_NonZero_Terms。

|**Index**|**Row**|**Col**|**Value**|
|---|---|---|---|
|0|6|6|4|
|1|0|3|22|
|2|1|0|15|
|...|...|...|...|

### 矩陣轉置 (Transpose)

將 $M_{ij}$ 變成 $M_{ji}$。

- **Simple Transpose (慢)**：
    
    - 針對每一個 Column (從 0 到 n)，跑迴圈去掃描整個 3-Tuple 陣列，找到對應的 Col 並交換存入新陣列。
        
    - 複雜度：$O(\text{cols} \times \text{terms})$。若矩陣很滿，terms 接近 $rows \times cols$，則變為 $O(rows \times cols^2)$，比傳統陣列轉置 $O(rows \times cols)$ 還慢！
        
- **Fast Transpose (快)**：
    
    - 使用兩個輔助陣列：
        
        1. `RowSize[]`：統計原本矩陣每一行(col)有幾個非零項。
            
        2. `RowStart[]`：計算轉置後，原本的 col (現在的 row) 應該從哪裡開始存。
            
    - 邏輯：只掃描一次資料，直接放到正確位置。
        
    - 複雜度：$O(\text{cols} + \text{terms})$。這是**線性時間**，非常快。
        

## 2. 十字連結串列 (Orthogonal List)

用 Linked List 實作稀疏矩陣。

每個節點包含 5 個欄位：

- `Row`, `Col`, `Value` (資料)
    
- `Down` (指向同 Column 下一個非零項)
    
- `Right` (指向同 Row 下一個非零項)
    

---

# 第四部分：Polynomial (多項式) —— List 的應用

## 1. 表示法比較

- **陣列**：$A[i]$ 存 $x^i$ 的係數。
    
    - 缺點：若 $P(x) = x^{1000} + 1$，陣列要開 1001 格，只用頭尾，浪費空間。
        
- **Linked List**：每個 Node 存 `(coef, exp, next)`。
    
    - 只存非零項。$P(x) = x^{1000} + 1$ 只需要 2 個節點。
        

## 2. 多項式相加演算法 (Poly Add)

這是 Linked List 操作的經典考題。

令指標 a 指向多項式 A，b 指向多項式 B (皆已按次方由大到小排序)。

`C = A + B` 的邏輯：

- **Case 1: `a->exp > b->exp`**
    
    - A 的這一項次方比較大，直接將 A 的這一項複製(或移)到 C。
        
    - `a` 往後移。
        
- **Case 2: `a->exp < b->exp`**
    
    - B 的這一項次方比較大，直接將 B 的這一項複製(或移)到 C。
        
    - `b` 往後移。
        
- **Case 3: `a->exp == b->exp`**
    
    - 次方相同，係數相加 `sum = a->coef + b->coef`。
        
    - **若 `sum != 0`**：產生新節點 `(sum, a->exp)` 接到 C 後面。
        
    - **若 `sum == 0`**：相抵消，不產生節點。
        
    - `a` 和 `b` **同時**往後移。
        

**複雜度**：$O(m + n)$，其中 $m, n$ 分別為 A, B 的項數。
