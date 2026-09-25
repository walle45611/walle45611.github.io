---
title: "Red-Black tree"
slug: red-black-tree
topic_section: data-structures
description: "My vault 資料結構筆記：Red-Black tree。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Red-Black tree.md"
---

## properties

1. 每個節點要嘛是紅色，要嘛是黑色。
    
2. 根節點是黑色。
    
3. 每個葉子（NIL）是黑色。
    
4. 如果一個節點是紅色，那麼它的兩個子節點都是黑色。
    
5. 對於每個節點，從該節點到所有後代葉子的所有簡單路徑上，必須包含相同數量的黑色節點。
    
補充說明：

- 葉子（leaf）實際上就是 NIL，不會存放真實 key，書裡的做法是用一個哨兵 (sentinel) 物件 T.nil 代表所有的 NIL，它的 color 固定是 BLACK，其他屬性（p, left, right, key）值隨意，因為不會被用到。
    
- 好處是省空間與簡化程式碼：
    
    - 如果每個 NIL 都是獨立物件，就會有很多額外的節點。
        
    - 用一個全域哨兵替代，讓所有 NIL 指標都指向同一個物件即可。
        
    - 這樣程式中判斷條件就簡單，只要檢查是否等於 T.nil。

### 定理

![01-定理](/vault-assets/39f1d6b158c35a65feeb.png)

- 紅黑樹高度  為 $O(\lg n)$，更精確地 $h \le 2\lg(n+1)$。

#### Proof

證明：含有 $n$ 個內部節點的紅黑樹，其高度 $h$ 滿足 $h \le 2\lg(n+1)$。

- **定義**

	1. 黑高（black-height）：$\operatorname{bh}(x)$ 為從節點 $x$ 到任一葉子 $\text{NIL}$ 的任一路徑上黑色節點的個數（若 $x$ 為黑色則計入）。
		
	2. 紅黑性質（節錄）：
			
		* (P3) 每個葉子（$\text{NIL}$）皆為黑色。
			
		* (P4) 若節點為紅，則其兩個子節點皆為黑。


- **輔助命題（Sublemma）**

	對任一節點 $x$，以 $x$ 為根的子樹至少包含 $2^{\operatorname{bh}(x)}-1$ 個**內部**節點。
	
	- **證明（歸納法）**
		![02-Proof - 證明(歸納法)](/vault-assets/8cbd0d1778991eb4e507.png)
		* **Base**：若 $x$ 高度為 0，則 $x=\text{NIL}$，$\operatorname{bh}(x)=0$。此時內部節點數為 $0=2^0-1$。
		
		* **IH**：對所有高度 $< h$ 的節點命題成立。
		
		* **Step**：取高度為 $h$ 的內部節點 $k$。
	
			由 (P4) 與黑高定義，任一子節點 $c$ 皆滿足
			
			$\operatorname{bh}(c) \ge bh(k)-1\quad (\text{黑子時等於 } bh(k)-1,\ \text{紅子時等於 } bh(k)).$
			
			由 **IH**：每個子樹至少有 $2^{bh(k)-1}-1$ 個內部節點。故左子數+右子數+root得以下公式
$$
			\#\text{internal}(x) \ge 1 + (2^{b-1}-1) + (2^{b-1}-1) = 2^b - 1.
$$
			得證。
			
- **高度上界推論**

	令 $h$ 為整棵樹的高度（root 到 $\text{NIL}$ 的邊數）。由 (P4) 可知任一路徑上紅節點不會相鄰，也就是他下面不會有紅色一定會是兩個黑點，因此在任一路徑上不失一般性**不含 root** 計算時，至少一半的節點是黑色。故根的黑高滿足 $\operatorname{bh}(\text{root}) \ge \frac{h}{2}.$
	
	內部節點數=$n$：
	
	$n \ge 2^{\operatorname{bh}(\text{root})}-1 \ge 2^{h/2}-1.$
	
	移項並取對數：
	
	$\lg(n+1) \ge \frac{h}{2} \quad \Rightarrow \quad h \le 2\,\lg(n+1).$

- **關於 “not including the root” 的說明** 
	
	![03-Proof](/vault-assets/9e7bc388142a6e9c8cc1.png)
	
	- 在上述「至少一半是黑色」的陳述中，CLRS 為了使不等式整潔，臨時以「不含 root」計數；root 依性質 (P2) 一定為黑。
	
	* 黑高的正式定義仍是：若節點本身是黑，即計入黑節點數。此技術性處理不影響最後結論。



## Red-Black Tree 插入 (Top-Down Approach)

### 步驟
1. 搜尋 X 的適當插入位置。
	
2. 在搜尋過程中，若發現經過的節點 (例如 Y) 兩個子節點皆為紅色，則執行 **Color Change**將  <mark>Y 標紅色， 將 Y 的兩個子節點標黑色  </mark>
		![04-步驟](/vault-assets/3a8f178bdee92461dd7c.png)
	接著檢查是否出現「連續的紅節點」（即 Y 以及 Y 的父節點是否同為紅色）。若有，則進行 Rotation 調整。
3.  找到正確位置後，放置新節點 X，並將 X 標示為紅色。
4. 檢查是否有連續的紅色節點（即 X 與 X 的父節點同為紅色）。若有，則進行 Rotation 調整。
5. 最後，檢查 Root 是否為黑色；若是紅色，則將其改為黑色。

### 特性

- 在步驟 (2) 與 (4) 的過程中，Rotation 可能發生一次或沒有發生，不會同時多次發生。  
	
- Insertion 過程中，<mark>最多發生 1 次 Rotation</mark>（不論是 `single` 或 `double rotation`）。  
	
- 時間複雜度：  
		
	- **Insertion**: $O(\log n)$  
		
	- **Rotation**: $O(1)$  
		
- 備註
		
	- **RB-Tree Delete X**: $O(\log n)$ 時間 + $O(1)$ rotation    
		
	- **AVL Tree**: 插入刪除可能需要 $O(\log n)$ 次 rotation

### Rotation

- 與 AVL Tree 的 rotation 類似，分為四種：
		
	- LL 與 RR（單旋轉）
		
	- LR 與 RL（雙旋轉）

- 規則：若自己與父親皆為紅色，往上看祖父，根據情況做四種旋轉。

![05-Rotation](/vault-assets/10a4b92431ae938c8e3c.png)

- 與 AVL Tree 的差異：
		
	1. 調整後，中間鍵值往上拉並標示為黑色。
		
	2. 左右兩側的子節點標示為紅色。
		
	3. 加上 color 調整的規則，而不是單純依靠高度差。
#### pseudocode

![06-pseudocode](/vault-assets/4d44647ff7d1f3087208.png)


### 範例

![07-範例](/vault-assets/8b9312064b2ae18ec949.png)

### ALGO CLRS

![08-ALGO CLRS](/vault-assets/bb277ade34be892e0f52.png)

![09-ALGO CLRS](/vault-assets/00d145e557b08e1b444c.png)


## Horowitz version RB-Tree

### 定義

1. 是 2-3-4 Tree 對等的 BST  
	
2. 對應 link 的顏色：非黑即紅  
	
3. 如果某個 link 在 2-3-4 Tree 存在，則在 RB Tree 視為紅色 link  
	
4. 任意 path 不可出現連續的紅色 links  
	
5. Root 到不同 Leaf 的 path 上都要有相同數量的黑色 links

#### 轉換規則 

![10-轉換規則](/vault-assets/61d741a66a675f1e3c70.png)

#### 範例

![11-範例](/vault-assets/a256976bd3785c023237.png)

### 2-3-4 Tree 高度分析

1. 若全部為 2-node：

$$
h = \log_{2}(n+1)
$$

2. 若全部為 4-node：

$$
h = \log_{4}(n+1) = \tfrac{1}{2}\log_{2}(n+1)
$$

3. 因此高度範圍：

$$
\tfrac{1}{2}\log_{2}(n+1) \;\leq\; h \;\leq\; \log_{2}(n+1)
$$

* 2-node：最差情況，高度較高。

* 4-node：最佳情況，高度較低。

* 混合：實際高度介於上述兩者之間。
