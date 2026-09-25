---
blog: true
blog_title: "多路搜尋樹與 B-Tree"
blog_date: 2026-09-24
blog_url: https://blog.walle4561.com/articles/posts/m-way-search-tree-and-b-tree/
---

# M-Way Search 

## 定義

**M-way Search Tree（M ≫ 2）重點**

- 主要用於 external search：資料量大，節點通常存於外部儲存（如磁碟），存取以 I/O 次數為主。
    
- 一個節點的 **degree** 介於 $0$ 與 $m$。
    
- 若節點的 degree 為 $m$，則該節點內的 **keys 數**為 $(m-1)$。
    
- 同一節點內的 keys 以 **遞增**排序。
    
- `search/insert/delete x` 的時間為 $O(h)$，其中 $h$ 是樹高；若結構 skewed，實際效能最壞也取決於 $h$。
    

## **定理**（高度為 $h$ 的 $m$-way search tree）

- 節點數上界：  $\sum_{i=1}^{h} (m^{i-1}) \,=\, \frac{m^{h}-1}{m-1}$
    
- Key 上界：  $m^{h}-1$
    

> 直觀：每層最多有 $m^{i-1}$ 個節點；每節點最多含 $(m-1)$ 個鍵，故總鍵數最多為 $(m-1)\cdot\frac{m^{h}-1}{m-1}=m^{h}-1$。


下例：degree = 4，keys = 3；根鍵為 `5 | 30 | 46`，四個子節點分別承接區間 `<5`、`[5,30)`、`[30,46)`、`≥46`。


![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/diagram-01.png]]

**設定**

- root level = 1。
    
- 每個節點最多 $m$ 個 children、至多 $(m-1)$ 個 keys。
    
- 高度 $h$（共有 $h$ 層）。
    

**(A) 最多節點數**  
第 $i$ 層（從 1 起算）最多節點數：$m^{i-1}$.  
總節點：

$$\sum_{i=1}^{h} m^{i-1} = 1 + m + m^2 + \cdots + m^{h-1} = \frac{m^{h}-1}{m-1}.$$

**(B) 最多鍵數**  
每節點最多 $(m-1)$ keys，故：

$$\bigg(\frac{m^{h}-1}{m-1}\bigg)\cdot(m-1) = m^{h}-1.$$

> 板書對照：右側的兩個 boxed 結果分別是 $\dfrac{m^{h}-1}{m-1}$（節點）、$m^{h}-1$（keys）。  
> 左側樹形圖：每層的最大節點計數為 $m^0, m^1, m^2, \dots, m^{h-1}$。

![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/diagram-02.png]]

# B Tree of Order M

## 定義 (Definition)

B-Tree of order m 是一種 **balanced m-way search tree**，主要應用於外部搜尋 (external search) 與外部排序 (external sort)。若非空，需滿足以下條件：

1. 根節點 (root) 的 degree：  $2 \leq \deg(\text{root}) \leq m$
    
2. 除根以外的內部節點 (internal nodes) 的 degree：  $\left\lceil \tfrac{m}{2} \right\rceil \leq \deg(\text{node}) \leq m$
    
3. 所有葉節點 (leaf nodes) 必須位於同一層，確保樹保持平衡。

## 定理 (Theorem)

對於高度為 $h$ 的 m-way search tree (root level = 1)：

1. **Maximum number of nodes**  $N_{max}⁡=\sum_{i=1}^h m^{i-1} = \frac{m^h - 1}{m - 1}$
    
2. **Maximum number of keys**  $K_{max} = m^h - 1$
    
3. **Minimum number of nodes**  $N_{\min} = 1 + 2 \cdot \frac{\left(\left\lceil \tfrac{m}{2} \right\rceil^{h-1} - 1 \right)}{\left\lceil \tfrac{m}{2} \right\rceil - 1}$
    
4. **Minimum number of keys**  $K_{\min} = 2 \cdot \left\lceil \tfrac{m}{2} \right\rceil^{h-1} - 1$
		
	- **最少 keys**：  $K_{\min}=2\,t^{\,h-1}-1=2\cdot\left\lceil\frac{m}{2}\right\rceil^{h-1}-1$
		
	    - 例如 $m=3$：  $K_{\min}=2\cdot\left\lceil\tfrac{3}{2}\right\rceil^{h-1}-1=2\cdot 2^{h-1}-1=2^{h}-1$
### 證明    

![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/01-證明.png]]

#### 最少 nodes（包含葉）

層別節點數（root 取 degree = 2，其餘皆取 $\lceil m/2\rceil$）：

$$

\#\text{level}_1=1,\quad \#\text{level}_2=2,\quad \#\text{level}_k=2\Big\lceil \frac{m}{2}\Big\rceil^{\,k-2}\;(k\ge2).

$$

因此

$$

\begin{aligned}

N_{\min}(h)

&=1+\sum_{k=2}^{h}2\Big\lceil \frac{m}{2}\Big\rceil^{k-2}

=1+2\sum_{i=0}^{h-2}\Big\lceil \frac{m}{2}\Big\rceil^{i}

=1+2\,\frac{\Big\lceil \tfrac{m}{2}\Big\rceil^{h-1}-1}{\Big\lceil \tfrac{m}{2}\Big\rceil-1}\,.

\end{aligned}

$$

#### 最少 keys

* 非根每個節點的最少 keys，簡單來說就是你會需要 3 個 $key$ 切開 4 個子樹：$\left\lceil \frac{m}{2} \right\rceil-1$

* 最少「非根」節點數：$\Big[\,2\cdot\frac{\left\lceil \tfrac{m}{2} \right\rceil^{\,h-1}-1}{\left\lceil \tfrac{m}{2} \right\rceil-1}\,\Big]$（由最少 nodes：$1+2\cdot\dfrac{\left\lceil \tfrac{m}{2} \right\rceil^{h-1}-1}{\left\lceil \tfrac{m}{2} \right\rceil-1}$ 減去 root 得到）

因此最少 keys：

$$

\begin{aligned}

K_{\min}(h)

&=\underbrace{1}_{\text{root}}+\Big[\,2\cdot\frac{\left\lceil \tfrac{m}{2} \right\rceil^{\,h-1}-1}{\left\lceil \tfrac{m}{2} \right\rceil-1}\,\Big]\cdot\Big(\left\lceil \tfrac{m}{2} \right\rceil-1\Big)\\[4pt]

&=1+2\Big(\left\lceil \tfrac{m}{2} \right\rceil^{\,h-1}-1\Big)\\[2pt]

&=2\,\left\lceil \tfrac{m}{2} \right\rceil^{\,h-1}-1\,.

\end{aligned}

$$
### 備註

- 名稱「2-3 Tree」、「2-3-4 Tree」等，來自允許的 **degree 範圍**：
    
    - Order 3 → degree = 2, 3 → 2-3 Tree
        
    - Order 4 → degree = 2, 3, 4 → 2-3-4 Tree
        
    - Order 5 → degree = 3, 4, 5 (root 可為 2)，**不是 2-3-4-5 Tree**


![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/diagram-03.png]]

### 判斷有沒有此種樹

| order mm | 根的 degree | 非根節點 degree | 允許的節點型別（degree） |              常見名稱               |       是否存在       |
| -------: | :-------: | :---------: | :-------------: | :-----------------------------: | :--------------: |
|        3 |    2–3    |     2–3     |      2, 3       |          **2-3 Tree**           |      **✓**       |
|        4 |    2–4    |     2–4     |     2, 3, 4     |         **2-3-4 Tree**          |      **✓**       |
|        5 |    2–5    |     3–5     |  3, 4, 5（根可 2）  | _order-5 B-Tree_ 或 _3-4-5 Tree_ | **✗ 對「2-3-4-5」** |

> 說明：m=5m=5 時，非根下限為 $\lceil 5/2\rceil=3$。一般節點**不能**有 degree=2，因此**沒有「2-3-4-5 Tree」**這個名稱。

### 例：2-3 Tree 給定 key 數 n 求高度界

- **最小高度**（最滿）：  $3^{h}-1\ge n\;\Rightarrow\; h=\left\lceil\log_{3}(n+1)\right\rceil$
    
- **最大高度**（最稀）：  $2^{h}-1\ge n\;\Rightarrow\; h=\left\lceil\log_{2}(n+1)\right\rceil$

## 插入 X 到 B 樹 (Order M)

1. Step 1. Search for X 由於 $X$ 不在樹中，會找到一個 external node (null)。  將 $X$ 放入該 external node 的 parent。  （找到 $X$ 的位置，在 node 放入 $X$）
	
2. Step 2. Check the node
	- 是否 overflow？key 數 $= m > m-1$？
		
	- 如果 **沒有 overflow** → 結束。  
		
	- 如果 **overflow** → 做 **split action**，並且往上檢查 parent。

3.  Split action
		
	1. 選出 $\lceil m/2 \rceil$ 的 key $k$  
		
	2. 把 $k$ 移到 parent  
		
	3. 其餘 key 分左右 child  

---

![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/diagram-04.png]]
### Example 

- B tree of order 3 (or 2-3 tree), what's the result after insert 55, 37


![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/diagram-05.png]]
1.  插入 55，但是因為 overflow 最多只能 $\left\lceil \tfrac{m}{2} \right\rceil \leq \deg(\text{node}) \leq m=2$, 所以需要 split action
	![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/02-Example.png|500]]
2. Insert 5,18 and 12 in the 2-3 tree ![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/03-Example.png]]
## Delete X in B-Tree of order m

> 目標：刪除鍵值 xx。先搜尋到包含 xx 的節點 NN。

1. Case I｜$N$ 是 **leaf**
		
	1. **刪除**：在 $N$ 直接移除 $x$。t6
		
	2. **檢查是否 underflow**（鍵數是否太少）。
			
		* **Not underflow**：若 $|N| \ge \lceil m/2\rceil - 1$ → **完成**。
			
		* **Underflow**：若 $|N| < \lceil m/2\rceil - 1$ → 進行修復：
				
			1. **Rotation（借鍵）**：
			
				* 嘗試向**兄弟節點**（相鄰的左或右）借一鍵；
				
				* 透過**父節點分隔鍵**旋轉；
				
				* 若成功 → **完成**。
			
			2. **Combine / Merge（合併）**：
			
				* 若無法旋轉，與相鄰兄弟**合併**，並把父節點中的分隔鍵**下移**到合併後的新節點；
				
				* 令 $N \leftarrow$ 該**父節點**（父節點少了一鍵），**回到本 Case 的步驟 2** 續檢查（可能向上連鎖）。

2. Case II｜$N$ 是 **non-leaf**
		
	1. **選替代鍵** $y$：
		
		* 取 **前驅**（左子樹中的**最大**鍵），或
		
		* 取 **後繼**（右子樹中的**最小**鍵）。

	2. **交換並遞迴刪除**：

		* 用 $y$ 取代 $x$（位置在 $N$）。
		
		* 到 $y$ 所在的 **leaf** 刪除 $y$。該 leaf 的鍵數減一，
		
		* 因此回到 **Case I** 的 underflow 檢查與修復流程。
### 註記

* $\lceil m/2\rceil - 1$ 為**非根節點**允許的最少鍵數。

* **Rotation** 與 **Merge** 的選擇依鄰接兄弟是否有多於最小鍵數而定。

* 可能需**一路向上**修復直到根；若根空了且只有一個子樹，則將該子樹提升為新根。

![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/04-註記.png]]
### Example 
-  B-Tree Order 3, delete 58 55 and 40![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/05-Example.png]]
- Delete 15,70 ![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/06-Example - Delete 15,70.png]]

# $B^+$ Tree of order m

## 定義 (Definition)

- 用途：ISAM（Index Sequential Access Method）。
    
- 僅分 **兩大層**：Index level、Data blocks level。
	1.  Index level
			
		- 結構：**B-tree of order mm**。
		    
		- 功能：**純索引**，不放資料（只放 **key** 與子指標）。
			
	2. Data blocks level
			
		- 作用：**存放資料**（records）。
		    
		- 連結：各 **data block 以 linked list 串接**，方便順序讀取。
		    
		- 容量：每個 block 的資料數 **可依題目/規格自行訂**，**不必**與 index 的 order m 相同，**==但是題目沒有特別說就是一樣==**。

![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/07-定義 (Definition).png]]
## Insert a data 
![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/08-Insert a data.png]]

## Delete a data
![[Assets/Note/Research/M-Way(Degree) Search tree And B-Tree/09-Delete a data.png]]
1. 尋找要刪除的節點，例如刪除 24 → 在葉節點 `[22 | 24]` 中找到。
    
2. 從葉節點刪除 24 → 節點剩下 `[22]`。
    
3. 檢查節點鍵值數，規則為 2 ≤ key ≤ 4。此時只有 1 個 key，不足。
    
4. 嘗試向兄弟節點借 key，如果兄弟節點也不足，則進行合併。
    
5. `[22]` 與兄弟節點 `[27 | 29]` 合併 → 得到 `[22 | 27 | 29]`。
    
6. 父節點 index 需要更新，原本有 `[17 | 27 | 30]`，因為合併，27 作為 index 不再需要 → 父節點變為 `[17 | 30]`。
    
7. 再檢查父節點是否滿足 2 ≤ key ≤ 4，如果不足則繼續合併或往上更新。
    
8. 若 root 最後只剩一個子節點，則降低樹高，新的 root 成為唯一的子節點。