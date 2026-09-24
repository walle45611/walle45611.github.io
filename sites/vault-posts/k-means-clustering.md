---
title: "K-means Clustering"
slug: k-means-clustering
topic_section: algorithms
description: "My vault 演算法筆記：K-means Clustering。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/K-means Clustering.md"
---

## 1. 核心目標：K-means Problem

解釋：

給定 $n$ 個 $d$ 維的資料點集合 $S$（$S \subset \mathbb{R}^d$），以及一個正整數 $k$，我們希望找到 $k$ 個「中心點」(centers) $C$，並將 $S$ 中的點分成 $k$ 個叢集 (clusters) $S^{(1)}, \ldots, S^{(k)}$，使得每個點到其所屬叢集之中心點的「平方距離總和」最小。

目標函式 (Cost Function) $f(S,C)$：

我們的目標是找到一組 centers $C = \langle \mathbf{c}^{(1)}, \ldots, \mathbf{c}^{(k)} \rangle$ 來最小化 $f(S,C)$：

$$f(S,C) = \sum_{\mathbf{x} \in S} \min_{1 \le j \le k} \Delta(\mathbf{x}, \mathbf{c}^{(j)})$$

其中 $\Delta(\mathbf{x}, \mathbf{c})$ 是點 $\mathbf{x}$ 和 center $\mathbf{c}$ 之間的平方歐幾里得距離：

$$\Delta(\mathbf{x}, \mathbf{c}) = \|\mathbf{x} - \mathbf{c}\|^2 = \sum_{a=1}^{d} (x_a - c_a)^2$$

目標函式也可以寫成 $k$ 個叢集內變異數 (variance) 的總和：

$$f(S,C) = \sum_{\ell=1}^{k} \sum_{\mathbf{x} \in S^{(\ell)}} \Delta(\mathbf{x}, \mathbf{c}^{(\ell)})$$

## 2. 核心規則：Nearest-Center Rule (最近中心法則)

解釋：

這條規則定義了「在給定 $k$ 個 centers ($C$) 的情況下，如何形成 $k$ 個叢集」。

一個資料點 $\mathbf{x}$ 屬於叢集 $S^{(\ell)}$，當且僅當 $\mathbf{x}$ 距離 $\mathbf{c}^{(\ell)}$ 的距離是所有 $k$ 個 center 中最近的（或之一）。

數學定義：

$$\mathbf{x} \in S^{(\ell)} \;\; \text{iff} \;\; \Delta(\mathbf{x}, \mathbf{c}^{(\ell)}) = \min_{1 \le j \le k} \Delta(\mathbf{x}, \mathbf{c}^{(j)})$$

- **平手 (Tie-breaking):** 如果 $\mathbf{x}$ 到 $\mathbf{c}^{(i)}$ 和 $\mathbf{c}^{(j)}$ 的距離一樣近（且都是最小），則可以任意指派 $\mathbf{x}$ 到 $S^{(i)}$ 或 $S^{(j)}$。但在 Lloyd's procedure 中，為了確保收斂，通常會維持 $\mathbf{x}$ 原本的歸屬，除非有**嚴格更近**的 center 出現。
    
- **注意：** Center $\mathbf{c}^{(\ell)}$ 不一定需要是 $S$ 中的原始資料點。
    
## 3. 求解演算法：Lloyd's Procedure

解釋：

由於 k-means problem 是 NP-hard，我們通常不找「全域最佳解」，而是使用 Lloyd's procedure 來找到一個「區域最佳解」(local minimum)。

這是一個迭代演算法，它在兩個步驟之間交替進行：

1. Initialize (初始化):
    
    從 $S$ 中隨機選取 $k$ 個點，作為初始的 centers $C = \langle \mathbf{c}^{(1)}, \ldots, \mathbf{c}^{(k)} \rangle$。
    
2. Assign points to clusters (指派點)：
    
    (固定 Centers)
    
    遍歷 $S$ 中的每一個點 $\mathbf{x}$，並根據 Nearest-Center Rule 將其指派到最近的 center $\mathbf{c}^{(\ell)}$ 所屬的叢集 $S^{(\ell)}$。
    
3. Recompute centers as centroids (重算中心)：
    
    (固定分群)
    
    對於 $k$ 個叢集中的每一個 $S^{(\ell)}$，重新計算其 centroid (質心或均值)，並將其設為新的 center $\mathbf{c}^{(\ell)}$。
    
4. Repeat (重複)：
    
    重複步驟 2 和 3，直到叢集指派不再發生變化（或 centers 的位置不再變動），此時演算法收斂。
    
## 4. 關鍵數學性質 (演算法的基礎)

Lloyd's procedure 之所以有效，是因為它交替地滿足了兩個最佳化條件：

**A. 給定叢集 $S^{(\ell)}$，最佳 Center 為 Centroid (Theorem 33.1)**

- **解釋：** 如果我們已經分好一群 $S^{(\ell)}$，什麼樣的 center $\mathbf{c}^{(\ell)}$ 能使其內部距離和 $\sum_{\mathbf{x} \in S^{(\ell)}} \Delta(\mathbf{x}, \mathbf{c}^{(\ell)})$ 最小？
    
- **答案：** 該群的 **centroid (均值)**。
    
- Centroid 公式：
    
    $$\mathbf{c}^{(\ell)} = \frac{1}{|S^{(\ell)}|} \sum_{\mathbf{x} \in S^{(\ell)}} \mathbf{x}$$
    
    (此公式即 Lloyd's procedure 的「步驟 3：重算中心」)
    

**B. 給定 Centers $C$，最佳叢集由 Nearest-Center Rule 決定 (Theorem 33.2)**

- **解釋：** 如果我們已經選定 $k$ 個 centers $C$，要如何分群 $S^{(1)}, \ldots, S^{(k)}$ 才能使總成本 $f(S,C)$ 最小？
    
- **答案：** 使用 **Nearest-Center Rule**。
    
- 原因： 總成本 $f(S,C)$ 是每個點 $\mathbf{x}$ 貢獻的總和。要使總和最小，只需確保每個 $\mathbf{x}$ 都被分配到離它最近的 center，使其貢獻的成本 $\Delta(\mathbf{x}, \mathbf{c}^{(\ell)})$ 為最小。
    
    (此規則即 Lloyd's procedure 的「步驟 2：指派點」)
    
## 5. 應用範例

**範例 1：地理資料分群 (如美國城市)**

- **$S$ (資料點):** $n$ 個城市的座標（例如 $n=49$）。
    
- **$d$ (維度):** $d=2$（緯度, 經度）。
    
- **$k$ (叢集數):** 自行設定，例如 $k=4$。
    
- **結果：** 演算法會找到 4 個「區域中心」(centroids)，並將 49 個城市分成 4 個地理上最緊湊的群組。
    

**範例 2：影像壓縮 (Vector Quantization)**

- **$S$ (資料點):** 影像中的**每一個像素**（例如 $700 \times 500 = 350,000$ 個點）。
    
- **$d$ (維度):** $d=3$，即每個像素的 (R, G, B) 顏色值。
    
- **$k$ (叢集數):** 我們希望壓縮後剩下的顏色總數，例如 $k=16$ 或 $k=256$。
    
- **演算法：**
    
    1. K-means 會在 3 維的 (R,G,B) 顏色空間中執行。
        
    2. 演算法找到 $k$ 個 (R,G,B) 向量，這 $k$ 個向量就是「**代表色**」(centroids)。
        
- **壓縮過程：**
    
    1. 儲存一個 $k$ 色「調色盤 (palette)」(包含 $k$ 個 centroids 的 (R,G,B) 值)。
        
    2. 原始影像中的每一個像素 $\mathbf{x}$，不再儲存其 24-bit 的 (R,G,B) 值，而是儲存其**最近的 centroid 的索引 (index)**。
        
- **結果：** 如果 $k=16$，每個像素只需要 4 bits ($2^4=16$) 來儲存索引，大幅降低儲存空間（從 24 bits 降至 4 bits）。
    
- **$k$ 值的取捨：**
    
    - **$k$ 越小 (如 $k=4$)：** 壓縮率非常高，檔案非常小，但顏色失真嚴重 (品質差)。
        
    - **$k$ 越大 (如 $k=256$)：** 壓縮率較低，檔案較大，但顏色更接近原始影像 (品質好)。

### 計算方式

- **資料點 $S$：**
    
    - $P_1 = (1, 2)$
        
    - $P_2 = (2, 1)$
        
    - $P_3 = (3, 2)$
        
    - $P_4 = (6, 7)$
        
    - $P_5 = (7, 8)$
        
    - $P_6 = (8, 6)$
        
- **$k=2$**
    
- **初始 Centers (第 0 輪)：**
    
    - $\mathbf{c}^{(1)} = P_1 = (1, 2)$
        
    - $\mathbf{c}^{(2)} = P_6 = (8, 6)$
        
- **距離公式：** $\Delta(\mathbf{x}, \mathbf{c}) = (x_1 - c_1)^2 + (x_2 - c_2)^2$
#### 完整計算步驟

#### 步驟 1：(A) Assign points to clusters (指派點到叢集)

我們必須計算**每一個點**到**每一個 center** 的（平方）距離，然後看哪個比較近。

|**資料點 x**|**計算 Δ(x,c(1)) (到 c(1)=(1,2) 的距離)**|**計算 Δ(x,c(2)) (到 c(2)=(8,6) 的距離)**|**哪個最近？**|**分配到叢集**|
|---|---|---|---|---|
|**$P_1 = (1, 2)$**|$(1-1)^2 + (2-2)^2 = 0$|$(1-8)^2 + (2-6)^2 = 49 + 16 = 65$|$0 < 65$|**$S^{(1)}$**|
|**$P_2 = (2, 1)$**|$(2-1)^2 + (1-2)^2 = 1 + 1 = 2$|$(2-8)^2 + (1-6)^2 = 36 + 25 = 61$|$2 < 61$|**$S^{(1)}$**|
|**$P_3 = (3, 2)$**|$(3-1)^2 + (2-2)^2 = 4 + 0 = 4$|$(3-8)^2 + (2-6)^2 = 25 + 16 = 41$|$4 < 41$|**$S^{(1)}$**|
|**$P_4 = (6, 7)$**|$(6-1)^2 + (7-2)^2 = 25 + 25 = 50$|$(6-8)^2 + (7-6)^2 = 4 + 1 = 5$|$50 > 5$|**$S^{(2)}$**|
|**$P_5 = (7, 8)$**|$(7-1)^2 + (8-2)^2 = 36 + 36 = 72$|$(7-8)^2 + (8-6)^2 = 1 + 4 = 5$|$72 > 5$|**$S^{(2)}$**|
|**$P_6 = (8, 6)$**|$(8-1)^2 + (6-2)^2 = 49 + 16 = 65$|$(8-8)^2 + (6-6)^2 = 0$|$65 > 0$|**$S^{(2)}$**|

(A) 的結果：

經過指派後，我們的兩個新叢集是：

- **$S^{(1)} = \{ P_1, P_2, P_3 \}$**
    
- **$S^{(2)} = \{ P_4, P_5, P_6 \}$**
    

---

#### 步驟 2：(B) Recompute centers as centroids (重算 centers)

現在我們根據上面分好的群組，來計算它們各自的「平均位置 (centroid)」，這就是我們的**新 centers**。

**計算新的 $\mathbf{c}^{(1)}$ (來自 $S^{(1)}$)：**

- $S^{(1)}$ 包含 $P_1(1, 2), P_2(2, 1), P_3(3, 2)$
    
- X 座標的平均： $\frac{1 + 2 + 3}{3} = \frac{6}{3} = 2$
    
- Y 座標的平均： $\frac{2 + 1 + 2}{3} = \frac{5}{3} \approx 1.67$
    

**計算新的 $\mathbf{c}^{(2)}$ (來自 $S^{(2)}$)：**

- $S^{(2)}$ 包含 $P_4(6, 7), P_5(7, 8), P_6(8, 6)$
    
- X 座標的平均： $\frac{6 + 7 + 8}{3} = \frac{21}{3} = 7$
    
- Y 座標的平均： $\frac{7 + 8 + 6}{3} = \frac{21}{3} = 7$
    

**(B) 的結果：**

- **新的 $\mathbf{c}^{(1)} = (2, 5/3)$** (或 $(2, 1.67)$)
    
- **新的 $\mathbf{c}^{(2)} = (7, 7)$**
    
### 總結

**第一輪迭代 (Iteration 1) 完成！**

- 我們從 $\mathbf{c}^{(1)}=(1, 2), \mathbf{c}^{(2)}=(8, 6)$ 開始。
    
- 經過「指派」和「重算」兩個步驟後，
    
- 我們的 centers **移動到了** 新位置：$\mathbf{c}^{(1)}=(2, 5/3), \mathbf{c}^{(2)}=(7, 7)$。
    
演算法不會在這裡停止。它會進入第二輪迭代 (Iteration 2)：

1. **Assign (指派)：** 使用這兩個**新 centers** $(2, 5/3)$ 和 $(7, 7)$，重新計算所有 6 個點的歸屬。
    
2. **Recompute (重算)：** 根據第二輪的指派結果，再次計算新的 centroids。
    
3. ... 一直重複，直到 centers 的位置不再有明顯變動（或點的歸屬不再改變），這時演算法就「收斂」了。
