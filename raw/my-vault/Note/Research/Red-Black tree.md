---
blog: true
blog_title: "Red-Black tree"
blog_date: 2026-09-24
blog_url: https://blog.walle4561.com/articles/posts/red-black-tree/
---

# properties

1. 每個節點要嘛是紅色，要嘛是黑色。
    
2. 根節點是黑色。
    
3. 每個葉子（NIL）是黑色。
    
4. 如果一個節點是紅色，那麼它的兩個子節點都是黑色。
    
5. 對於每個節點，從該節點到所有後代葉子的所有簡單路徑上，必須包含相同數量的黑色節點。
    
補充說明：

- 葉子（leaf）實際上就是 NIL，不會存放真實 key，書裡的做法是用一個哨兵 (sentinel) 物件 T.nil 代表所有的 NIL，它的 color 固定是 BLACK，key 不存放有效資料；但不能說其他屬性都不會被用到。CLRS 刪除與 transplant 會設定 T.nil.p，讓修復程序能取得目前 NIL 所在位置的父節點。共享哨兵的 p 並不永久代表所有 NIL 的父節點。
    
- 好處是省空間與簡化程式碼：
    
    - 如果每個 NIL 都是獨立物件，就會有很多額外的節點。
        
    - 用一個全域哨兵替代，讓所有 NIL 指標都指向同一個物件即可。
        
    - 這樣程式中判斷條件就簡單，只要檢查是否等於 T.nil。

## 定理
![[Assets/Note/Research/Red-Black tree/01-定理.png]]
- 紅黑樹高度  為 $O(\lg n)$，更精確地 $h \le 2\lg(n+1)$。

### Proof

證明：含有 $n$ 個內部節點的紅黑樹，其高度 $h$ 滿足 $h \le 2\lg(n+1)$。

- **定義**

	1. 黑高（black-height）：$\operatorname{bh}(x)$ 為從節點 $x$ 到任一葉子 $\text{NIL}$ 的任一路徑上黑色節點的個數（不計入 $x$ 本身，但計入路徑終點 NIL）。因此 $\operatorname{bh}(\text{NIL})=0$。
		
	2. 紅黑性質（節錄）：
			
		* (P3) 每個葉子（$\text{NIL}$）皆為黑色。
			
		* (P4) 若節點為紅，則其兩個子節點皆為黑。


- **輔助命題（Sublemma）**

	對任一節點 $x$，以 $x$ 為根的子樹至少包含 $2^{\operatorname{bh}(x)}-1$ 個**內部**節點。
	
	- **證明（歸納法）**
		![[Assets/Note/Research/Red-Black tree/02-Proof - 證明(歸納法).png|500]]
		* **Base**：若 $x$ 高度為 0，則 $x=\text{NIL}$，$\operatorname{bh}(x)=0$。此時內部節點數為 $0=2^0-1$。
		
		* **IH**：對所有高度 $< h$ 的節點命題成立。
		
		* **Step**：取高度為 $h$ 的內部節點 $k$，令 $b=\operatorname{bh}(k)$。
	
			由黑高定義與性質 (P5)，任一子節點 $c$ 皆滿足
			
			$\operatorname{bh}(c) \ge bh(k)-1\quad (\text{黑子時等於 } bh(k)-1,\ \text{紅子時等於 } bh(k)).$
			
			由 **IH**：每個子樹至少有 $2^{bh(k)-1}-1$ 個內部節點。故左子數+右子數+root得以下公式
			$$
			\#\text{internal}(k) \ge 1 + (2^{b-1}-1) + (2^{b-1}-1) = 2^b - 1.
			$$
			得證。
			
- **高度上界推論**

	令 $h$ 為整棵樹的高度（root 到 $\text{NIL}$ 的邊數）。由 (P4) 可知紅節點不會相鄰，且路徑終點 NIL 為黑色。排除 root 後，路徑上有 $h$ 個節點；每個紅節點後面都接著一個黑節點，因此至少一半的節點是黑色。故根的黑高滿足 $\operatorname{bh}(\text{root}) \ge \frac{h}{2}.$
	
	內部節點數=$n$：
	
	$n \ge 2^{\operatorname{bh}(\text{root})}-1 \ge 2^{h/2}-1.$
	
	移項並取對數：
	
	$\lg(n+1) \ge \frac{h}{2} \quad \Rightarrow \quad h \le 2\,\lg(n+1).$

- **關於 “not including the root” 的說明**

	![[Assets/Note/Research/Red-Black tree/03-Proof.png]]

	- 「不計入起點」就是 CLRS 黑高定義的一部分，不是證明時臨時改變計數方式。
	- 例如只有一個黑色 root 的樹：root 到 NIL 的高度為 1，黑高也為 1；NIL 本身的黑高為 0。


# Red-Black Tree 插入 (Top-Down Approach)

## 步驟
1. 搜尋 X 的適當插入位置。
	
2. 在搜尋過程中，若發現經過的節點 (例如 Y) 兩個子節點皆為紅色，則執行 **Color Change**將  ==Y 標紅色， 將 Y 的兩個子節點標黑色  ==
		![[Assets/Note/Research/Red-Black tree/04-步驟.png]]
	接著檢查是否出現「連續的紅節點」（即 Y 以及 Y 的父節點是否同為紅色）。若有，則進行 Rotation 調整。
3.  找到正確位置後，放置新節點 X，並將 X 標示為紅色。
4. 檢查是否有連續的紅色節點（即 X 與 X 的父節點同為紅色）。若有，則進行 Rotation 調整。
5. 最後，檢查 Root 是否為黑色；若是紅色，則將其改為黑色。

## 特性與版本差異

- Top-down 在向下搜尋途中分裂 4-node（變色），遇到紅紅衝突便立即修復；不同深度可能各自需要修復，不能直接套用 CLRS 的「整次插入至多兩次基本旋轉」。保守上界為 $O(\log n)$ 次局部修復，總時間仍為 $O(\log n)$。
- CLRS 採 Bottom-up：先插入紅色節點，再向上修復。一次插入至多兩次基本旋轉；雙旋轉由兩次基本旋轉組成，變色則可能沿祖先傳播 $O(\log n)$ 層。
- 每次基本旋轉花費 $O(1)$；搜尋、插入與刪除的最壞時間皆為 $O(\log n)$。
- CLRS 刪除至多三次基本旋轉，此界限需與使用的演算法版本一起說明。
- AVL 插入至多在一個失衡位置做單旋轉或雙旋轉；AVL 刪除可能在 $O(\log n)$ 個祖先位置旋轉。

## Rotation

- 與 AVL Tree 的 rotation 類似，分為四種：
		
	- LL 與 RR（單旋轉）
		
	- LR 與 RL（雙旋轉）

- 規則：若自己與父親皆為紅色，往上看祖父，根據情況做四種旋轉。
![[Assets/Note/Research/Red-Black tree/05-Rotation.png]]
- 與 AVL Tree 的差異：
		
	1. 調整後，中間鍵值往上拉並標示為黑色。
		
	2. 左右兩側的子節點標示為紅色。
		
	3. 加上 color 調整的規則，而不是單純依靠高度差。
### pseudocode
![[Assets/Note/Research/Red-Black tree/06-pseudocode.png]]

## 範例
![[Assets/Note/Research/Red-Black tree/07-範例.png]]
## ALGO CLRS（Bottom-up 插入）

以下 CLRS 演算法先完成 BST 插入，再向上修復；它與前面的 Top-down 搜尋途中修復流程不同。

![[Assets/Note/Research/Red-Black tree/08-ALGO CLRS.png|600]]
![[Assets/Note/Research/Red-Black tree/09-ALGO CLRS.png|700]]

# Horowitz version RB-Tree

## 定義

1. 是 2-3-4 Tree 對等的 BST  
	
2. 對應 link 的顏色：非黑即紅  
	
3. 同一個 2-3-4 節點內的多個 key，以紅色 link 連接其對應的 BST 節點；不同 2-3-4 節點之間的父子連結對應黑色 link。收縮紅色 link，即可還原 2-3-4 節點
	
4. 任意 path 不可出現連續的紅色 links  
	
5. Root 到不同 Leaf 的 path 上都要有相同數量的黑色 links

### 轉換規則 

![[Assets/Note/Research/Red-Black tree/10-轉換規則.png|500]]
### 範例
![[Assets/Note/Research/Red-Black tree/11-範例.png|500]]
## 2-3-4 Tree 高度分析

此處 $n$ 表示 key 的總數，$h$ 表示根到外部 NIL 的邊數（也等於含 key 的層數）。若高度改以根到最深含 key 節點的邊數表示，下列界限需減 1。

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

## 文字校正依據

- [Dartmouth：紅黑樹定義、黑高證明與 CLRS 插入／刪除](https://www.cs.dartmouth.edu/~thc/cs10/lectures/0519/0519.html)
- [University of Washington：紅黑樹旋轉次數](https://courses.washington.edu/css343/bernstein/2013-q4/lectures/lecture-05.html)

原始手寫圖片保留；若圖片採用不同定義或演算法版本，應按上述文字區分。
