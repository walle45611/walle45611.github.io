---
title: "二元搜尋樹 (Binary Search Tree)"
slug: binary-search-tree
topic_section: data-structures
description: "二元搜尋樹 (Binary Search Tree)的重點整理。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/二元搜尋樹 (Binary Search Tree).md"
---

### 定義

- 是一棵二元樹，可以為空。
    
- 若不為空，必須滿足：
    
    1. **左子樹**所有節點的值均小於根節點的值。
        
    2. **右子樹**所有節點的值均大於根節點的值。
        
    3. 左、右子樹本身也都是二元搜尋樹。
        

### 特性

- **Inorder Traversal (中序走訪)**：順序為 Left → Data → Right (L D R)。

- **排序應用**：
	
	1. 將輸入資料逐一插入，建構成 BST。
	
	2. 對 BST 進行 Inorder Traversal，可獲得升冪排序結果。

- **搜尋與重複值處理**：
	  
  - BST 不允許重複的 key 值，因為它是設計來進行「搜尋」的。
  - 插入一筆資料 `x` 時，**總是會先進行搜尋**：
	- 如果已經找到 `x`，就不需要再插入。
	- 如果找不到，才會插入 `x`。
  - ➤ 這確保了 BST 的 key 值都是唯一的。

- **不同的插入策略**：
	
  - **方法一**（常見）：
    - 若 `x <= 節點值` → 插入左子樹
    - 若 `x > 節點值` → 插入右子樹

  - **方法二**（記錄方式不同）：
    - 每個節點用 `key` 表示左子樹的值，用 `counter` 表示右子樹中該 key 的出現次數或統計資料（可用於統計或自訂搜尋邏輯）

#### 證明 Inorder Traversal 有序性

假設對任意節點 $v$：

1. 左子樹所有值小於 $v$，右子樹所有值大於 $v$。
    
2. 對左子樹 Inorder Traversal 得到有序序列 $L$，且所有 $L$ 中元素都小於 $v$。
    
3. 訪問 $v$。
    
4. 對右子樹 Inorder Traversal 得到有序序列 $R$，且所有 $R$ 中元素都大於 $v$。

合併後序列為：
$$L  ∥  [v]  ∥  R$$

因此整體輸出序列有序。

---

### 時間複雜度

令節點數為 n，BST 高度為 hh。

- **單次搜尋 / 插入 / 刪除**：
    
    - 最壞：$O(h)=O(n)$（BST 退化時）
        
    - 平均：$O(\log n)$（AVL、RB Tree 等）
        
- **建構 (插入 n 個元素)**：
    
    - 普通 BST 最壞：$O(n^2)$（若每次插入均退化為鏈尾）
        
    - 平衡 BST：$O(n\log n)$
        
- **Inorder Traversal**：$O(n)$
    

---

## 在 BST 中搜尋 X (Search X)

- **時間複雜度**：$O(h)$，其中 $h$ 為 BST 高度。
    
- **最壞情況**：$O(n)$。
    
- **最佳情況**：$O(1)$（命中根節點）。
    
- **平均 / 平衡**：$O(\log ⁡n)$。
    

> 插入與刪除操作時間複雜度同搜尋。

```c
int search(struct node *root, int x)
{
    if (root == NULL)
        return 0;               // 未找到
    else if (root->data == x)
        return 1;               // 找到
    else if (x > root->data)
        return search(root->right_child, x);
    else
        return search(root->left_child, x);
}
```

---

### 搜尋時間複雜度推導

這段推導其實是在算一棵理想（滿／完全）二元樹上，搜尋所有節點一次所需要的比較次數總和 $S$，然後再除以節點數 $n$ 得到「平均比較次數」。具體思路是：

1. **分層計算**  
   一棵高為 $k$ 的滿二元樹，層號從 1 到 $k$：  
   - 第 $j$ 層有 $2^{j-1}$ 個節點，  
   - 搜到第 $j$ 層的比較次數就是 $j$。  
   因此把所有層加起來：  
$$
   S = 1\cdot2^0 + 2\cdot2^1 + 3\cdot2^2 + \cdots + k\cdot2^{\,k-1}.
$$

2. **乘以 2 再相減（消項技巧）**  
   為了化簡這個帶 $2^{j-1}$ 的級數，我們對整式乘以 2：  
$$
   2S = 1\cdot2^1 + 2\cdot2^2 + 3\cdot2^3 + \cdots + k\cdot2^{\,k}.
$$
   接著用 $2S - S$，中間大部分項會互相抵消，只剩下尾巴和開頭累加：  
$$
   2S - S
     = k\cdot2^k
       - \bigl(2^0 + 2^1 + \cdots + 2^{k-1}\bigr)
     = k\,2^k - (2^k - 1).
$$
   因此：
$$
   S = k\,2^k - (2^k - 1).
$$

3. **除以節點總數**  
   滿二元樹的總節點數為 $n = 2^k - 1$，所以平均比較次數：  
$$
   \frac{S}{n}
   = \frac{k\,2^k - (2^k - 1)}{2^k - 1}
   = \frac{k\,2^k}{2^k - 1} - 1
   \approx k - 1
   = \lceil \log_2(n+1)\rceil - 1.
$$

---

## 找最小值 / 最大值 (Find-min / Find-max)

- **時間複雜度**：
    
    - 最佳：$O(log⁡n)$
        
    - 最壞：$O(n)$
        

```c
struct node* find_minimum(struct node *root)
{
    if (root == NULL)
        return NULL;
    else if (root->left_child == NULL)
        return root;
    return find_minimum(root->left_child);
}

struct node* find_maximum(struct node *root)
{
    if (root == NULL)
        return NULL;
    else if (root->right_child == NULL)
        return root;
    return find_maximum(root->right_child);
}
```

---

## 插入 X (Insert X)

- 若 X 已存在，則不插入。
    
- 策略：
    
    - 若 $x \le root$->data → 插入右子樹。
        
    - 若 $x \gt root$->data → 插入左子樹。
        

```c
struct node* new_node(int x) {
    struct node *p;
    p = malloc(sizeof(struct node));
    p->data = x;
    p->left_child = NULL;
    p->right_child = NULL;
    return p;
}

struct node* insert(struct node *root, int x) {
    if (root == NULL)
        return new_node(x);
    else if (x > root->data)  // 往右子樹插入
        root->right_child = insert(root->right_child, x);
    else                      // 往左子樹插入
        root->left_child = insert(root->left_child, x);
    return root;
}
```

---

### 刪除節點 (Delete X in a BST)
1. **Degree 0（葉節點 leaf）**
   - 條件：無左子也無右子
   - 刪除方式：直接刪除該節點

2. **Degree 1**
   - 條件：僅有一個子節點（左或右其中之一為 NULL）
   - 刪除方式：用唯一的子節點取代自己，保留原本在父節點的位置（如果原本在右邊，就把子節點接到右邊）

3. **Degree 2**
   - 條件：同時擁有左子與右子節點
   - 刪除方式：
     1. 從右子樹中找最小值（或左子樹找最大值）
     2. 用該值取代當前節點
     3. 再去刪除那個被取代的值（此時會是 degree 0 或 degree 1）
	```
	       50
	      /  \
	    30    70
	   / \    / \
	 20  40  60  80
		 
	 刪除節點 50：
		 
		   60
	      /  \
	    30    70
	   / \      \
	 20  40     80
	```

```c
struct node* delete(struct node *root, int x) {
    if (root == NULL)
        return NULL;

    if (x > root->data) {
        root->right_child = delete(root->right_child, x);
    }
    else if (x < root->data) {
        root->left_child = delete(root->left_child, x);
    }
    else {
        // 找到欲刪除的節點
        if (root->left_child == NULL && root->right_child == NULL) {
            free(root);
            return NULL;
        }
        // x degree-1
        else if (root->left_child == NULL || root->right_child == NULL) {
            struct node *temp;
            if (root->left_child == NULL)
                temp = root->right_child;
            else
                temp = root->left_child;

            free(root);
            return temp;
        }
        // x degree-2
        else {
            struct node *temp = find_minimum(root->right_child);
            root->data = temp->data;
            root->right_child = delete(root->right_child, temp->data);
        }
    }

    return root;
}
