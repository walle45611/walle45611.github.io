---
title: "Splay Trees"
slug: splay-trees
topic_section: data-structures
description: "My vault 資料結構筆記：Splay Trees。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Splay Trees.md"
---

## 定義

- **Splay Tree**：一種二元搜尋樹 (BST)。
    
- 每次執行 **Search / Insert / Delete** 操作後，會對相關節點做 **splay (伸展)**，也就是透過一連串旋轉，將該節點移到樹根。
    
- 目標：利用平攤分析 (amortized analysis)，保證平均操作時間為 **O(log n)**。
### Splay Tree Lemma 與 Theorem 證明整理

#### Lemma：單次 splay 的 amortized cost

**敘述** 對於起始節點 $q$，單次 splay 的平攤成本至多為 $3(\log_2 n - r(q)) + 1$

- **位能法公式**
$$
	\text{Amortized cost} = (\text{actual cost}) + (\Phi_i - \Phi_{i-1})
$$
	- 其中：
			
		* $r(i) = \log_2s(i)$，$s(i)$ 是以節點 $i$ 為根的子樹大小。
			
		* 位能 $\Phi = \sum_i r(i)$。

**分情況討論**

1. **Zig**：
		
	* $q$ 為 root 或無祖父。
		
	* 成本 $=1$，位能不變。
		
	* Amortized cost $=1$。

2. **Zig（有父無祖父）**：
	* 實際成本 = $1$。
		
	* 位能變化：$\Delta \Phi =\Phi_i - \Phi_{i-1} = (r'(p) + r'(q)) - (r(p) + r(q))$
			
		* 單調性
				
			- $p$ 的子樹變小：$s'(p) \le s(p) \;\Rightarrow\; r'(p) \le r(p)$
				
			- $q$ 的子樹變大：$s'(q) \ge s(q) \;\Rightarrow\; r'(q) \ge r(q)$
				
	* 由性質可得：$\text{Amortized cost}=1+\Delta \Phi \leq 1 + (r'(q) - r(q))$

3. **Zig-Zig / Zig-Zag**：
		
	* 影響節點 $q, p, gp$。
		
	* 位能變化：
$$
		\Delta \Phi = r'(q) + r'(p) + r'(gp) - r(q) - r(p) - r(gp)
$$
	
	* 分析可得：
$$
		\text{Amortized cost} \leq 3(r'(q) - r(q)) - 1
$$
綜合各情形得 Access Lemma：$\text{amortized cost} \le 3\big(r(\text{root}) - r(q)\big) + 1 \le 3(\log_2 n - r(q)) + 1$。 因此為 $O(\log n)$，且給出更精確的常數界。

#### Theorem 10.1：n 次操作的總時間

**敘述** 在一棵空的 splay tree 上進行 $n$ 次操作（search, insert, delete），總時間為 $O(n \log n)$。

**推導**

* 單次操作時間：

$$
T_i = (\text{amortized cost}) + (\Phi_{i-1} - \Phi_i)
$$

* n 次操作總和：

$$
T = \sum_{i=1}^n T_i = \sum_{i=1}^n (\text{amortized cost}) + (\Phi_0 - \Phi_n)
$$

* 因為 $\Phi_0 = 0$ 且 $\Phi_n \geq 0$：

$$
T \leq \sum_{i=1}^n (\text{amortized cost})
$$

* 根據 Lemma：

$$
T \leq O(n \log n)
$$

**結論** : n 次操作的總時間複雜度為 $O(n \log n)$。

## Splay 起點

- **Search**：找到的節點。
    
- **Insert**：新插入的節點。
    
- **Delete**：被刪除節點的父節點（若存在），若刪除的是根則為 `NULL`。
	
- 那麼做完所有操作都要做 `Splay`，並且觀察 `gp`和 `p` 是調整的哪種 case，調整的 root
### Splay 操作規則

1. **Zig**
    ![01-Splay 操作規則 - Zig](/vault-assets/54fc895da97f4504bd3c.png)
    - 條件：節點 `q` 有父節點 `p`，但沒有祖父節點 `gp`。
        
    - 動作：對 `p` 和 `q` 做單次旋轉。
        
    - 註解：只需一步旋轉就能將節點提到根．
	
2. **Zig-Zig**
	![02-Splay 操作規則 - Zig-Zig](/vault-assets/a2e0fdf3b8ee0f9375ec.png)
    - 條件：節點 `q` 有父節點 `p` 和祖父節點 `gp`，且 `q` 和 `p` 在同一方向（LL 或 RR）。
        
    - 動作：先旋轉 `gp-p`，再旋轉 `p-q`。
        
    - 註解：連續兩次同方向旋轉，像「之」字往同一邊彎，稱為 Zig-Zig。
        
3. **Zig-Zag**
	![03-Splay 操作規則 - Zig-Zag](/vault-assets/e04fa165c821ebbbd2c5.png)
    - 條件：節點 `q` 有父節點 `p` 和祖父節點 `gp`，但 `q` 和 `p` 在不同方向（LR 或 RL）。
        
    - 動作：先旋轉 `p-q`，再旋轉 `gp-q`。
        
    - 註解：方向交錯，先往一邊再往另一邊，像「之」字折返，稱為 Zig-Zag。

### 範例

![04-範例](/vault-assets/767b5337623cd46263dc.png)

![05-範例](/vault-assets/6d6d02494761d588a9ae.png)


| 節點  | $s(i)$ 計算                                | $r(i)$ 計算               |
| --- | ---------------------------------------- | ----------------------- |
| 5   | $s(5)=1+\lvert e\rvert+\lvert f\rvert=1$ | $r(5)=\log_2(1)=0.0000$ |
| 4   | $s(4)=1+\lvert d\rvert+s(5)=1+0+1=2$     | $r(4)=\log_2(2)=1.0000$ |
| 3   | $s(3)=1+\lvert c\rvert+s(4)=1+0+2=3$     | $r(3)=\log_2(3)=1.5850$ |
| 6   | $s(6)=1+s(3)+\lvert g\rvert=1+3+0=4$     | $r(6)=\log_2(4)=2.0000$ |
| 7   | $s(7)=1+s(6)+\lvert h\rvert=1+4+0=5$     | $r(7)=\log_2(5)=2.3219$ |
| 2   | $s(2)=1+\lvert b\rvert+s(7)=1+0+5=6$     | $r(2)=\log_2(6)=2.5850$ |
| 8   | $s(8)=1+s(2)+\lvert i\rvert=1+6+0=7$     | $r(8)=\log_2(7)=2.8074$ |
| 9   | $s(9)=1+s(8)+\lvert j\rvert=1+7+0=8$     | $r(9)=\log_2(8)=3.0000$ |
| 1   | $s(1)=1+\lvert a\rvert+s(9)=1+0+8=9$     | $r(1)=\log_2(9)=3.1699$ |
$$
\Phi \,=\, r(1)+r(9)+r(8)+r(2)+r(7)+r(6)+r(3)+r(4)+r(5)\,\approx\,18.4691
$$

## 複雜度比較

| 資料結構           | 調整方式 | 單次最壞情況     | 平攤時間       |
| -------------- | ---- | ---------- | ---------- |
| AVL Tree       | 高度平衡 | $O(log n)$ | $O(log n)$ |
| Red-Black Tree | 弱平衡  | $O(log n)$ | $O(log n)$ |
| **Splay Tree** | 自我調整 | $O(n)$     | $O(log n)$ |
- splay tree $O(n)$ 是樹高 $n$
## 應用

- 適合存取 **局部性高** 的資料（快取、符號表、字典）。
    
- 不需額外存平衡資訊，比 AVL / RB-Tree 簡單。
