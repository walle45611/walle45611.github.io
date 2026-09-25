
### 1. 漢米爾頓路徑與迴路存在定理 (General Graphs)

這些定理主要給出圖形擁有 HP 或 HC 的**充分條件** (Sufficient Conditions)。

| **類型**       | **條件描述 (若滿足此條件)**                              | **結果**    | **備註**              |
| ------------ | ---------------------------------------------- | --------- | ------------------- |
| **HP (路徑)**  | 任意**不相關** (non-adjacent) 兩點 degree 和 $\ge n-1$ | 有 **HP**  | Ore's Theorem 變體    |
| **HP (路徑)**  | 每個頂點 degree $\ge \frac{n-1}{2}$                | 有 **HP**  | Dirac's Theorem 變體  |
| **HC (迴路)**  | 任意**不相關** (non-adjacent) 兩點 degree 和 $\ge n$   | 有 **HC**  | **Ore's Theorem**   |
| **HC (迴路)**  | 每個頂點 degree $\ge \frac{n}{2}$                  | 有 **HC**  | **Dirac's Theorem** |
| **HC (迴路)**  | 邊數 $E \ge \binom{n-1}{2} + 2$                  |           |                     |
| **Directed** | 有向完全圖 $K_n^*$ (Tournament Graph)               | 必有 **HP** | Rédei's Theorem     |

---

### 2. 完全圖 ($K_n$) 的性質

針對擁有 $n$ 個頂點的完全圖 ($K_n$) 的計數與性質。

|**項目**|**數量 / 公式**|**說明**|
|---|---|---|
|**相異 HC 數量**|$\frac{(n-1)!}{2}$|圓排列除以 2 (無向圖)|
|**互斥 HC 數量**|$\lfloor \frac{n-1}{2} \rfloor$|Edge-disjoint Hamiltonian Cycles|
|**HP 數量**|$n!$|筆記記載為 $n!$ (通常指有向或起點終點視為不同)，無向通常為 $n!/2$|
|**互斥 HP 數量**|$\lfloor \frac{n}{2} \rfloor$|Edge-disjoint Hamiltonian Paths|

---

### 3. 雙分圖 (Bipartite Graph) 與完全雙分圖 ($K_{m,n}$)

針對頂點集分為 $A, B$ 兩邊 ($|A|=m, |B|=n$) 的圖形。

| **主題**             | **性質 / 條件**                | **公式 / 結論**                                                                             |
| ------------------ | -------------------------- | --------------------------------------------------------------------------------------- |
| **基本定義**           | 判斷標準                       | 不含**奇數長度的 Cycle** (Odd Cycle)                                                           |
| **邊數限制**           | Simple Bipartite Max Edges | $e \le \lfloor \frac{n}{2} \rfloor \lceil \frac{n}{2} \rceil$ (即 $e \le \frac{n^2}{4}$) |
| **$K_{m,n}$ 規格**   | 頂點與邊數                      | $\|V\|=m+n$ 和 $\|E\|=m \cdot n$                                                         |
| **$K_{m,n}$ 有 HC** | 存在條件                       | 當且僅當 $m = n$                                                                            |
| **$K_{m,n}$ 有 HP** | 存在條件                       | 當且僅當 $                                                                                  |
| **$K_{n,n}$ 計數**   | 相異 HC 數量                   | $\frac{(n-1) n!}{2}$                                                                    |
| **$K_{m,n}$ 計數**   | 相異 HP 數量                   | $m! n!$ (當 $\|m-n\| = 0 \text{ or }1$                                                   |
| **一般雙分圖**          | 存在 HC 的必要條件                | 兩邊人數必須相等 ($\|A\|=\|B\|$)                                                                |
| **一般雙分圖**          | 存在 HP 的必要條件                | 兩邊人數差距不能超過 1 ($\|\|A\|-\|B\|\| \le 1$)                                                  |
