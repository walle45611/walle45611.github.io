---
title: "Reduction & Problem Types"
slug: reductions-and-problem-types
topic_section: algorithms
description: "My vault 演算法筆記：Reduction & Problem Types。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Reduction & Problem Types.md"
---

### 1. 決策問題 vs. 優化問題 (Decision vs. Optimization)

在計算複雜度理論中，我們通常關注「決策問題」，但實際應用常遇到「優化問題」。

#### 1.1 決策問題 (Decision Problems)

- **定義**：問題的答案只有簡單的 **"Yes" (1)** 或 **"No" (0)**。
    
- **範例**：
    
    - **PRIME**：給定自然數 $x$，$x$ 是質數嗎？
        
    - **COLOR**：給定圖 $G$ 和數值 $k$，能否用 $k$ 種顏色為頂點著色且相鄰頂點顏色不同？
        
    - **MST (Minimum Spanning Tree)**：給定圖 $G$ 和邊界 $K$，是否存在成本 $\le K$ 的生成樹？
        
    - **KNAPSACK**：背包問題。給定容量 $C$ 與目標價值 $V$，是否存在一種裝法使得總價值 $\ge V$ 且不超重？
        

#### 1.2 優化問題轉決策問題 (Converting Optimization to Decision)

- **關係**：
    
    - 每一個優化問題都有一個對應的決策版本，該決策版本的難度**不高於** (no harder than) 優化問題。
        
    - 事實上，它們通常具有**相同**的計算難度 (Computational Difficulty)。
        
- **轉換方法**：
    
    - **從優化解決策 ($A_{opt} \to P_{dec}$)**：若能算出最佳解（如最短路徑長度），只需檢查該值是否 $\le k$ 即可回答決策問題。
        
    - **從決策解優化 ($A_{dec} \to P_{opt}$)**：利用二分搜尋法 (Binary Search) 配合決策演算法。透過不斷詢問「最佳值是否 $\le X$？」來逼近並找出最佳解。
        

### 2. 多項式時間 Reduction (Polynomial-time Reduction)

![01-2. 多項式時間 Reduction (Polynomial-time](/vault-assets/844b533c7e03d7996a2a.png)

#### 2.1 定義與機制

- **符號**：$A \le_p B$ (讀作：Problem A reduces to Problem B)。
    
- **概念**：存在一個**多項式時間演算法 (Reduction Algorithm $f$)**，能將問題 $A$ 的每一個實例 (Instance $\alpha$) 轉換成問題 $B$ 的一個實例 (Instance $\beta$)。
    
- **邏輯條件**：
    
    - 轉換後的答案必須一致：$AlgA(\alpha) = 1 \iff AlgB(f(\alpha)) = 1$。
        
    - 這意味著我們可以使用解決 $B$ 的演算法 ($AlgB$) 來解決 $A$。
        

#### 2.2 難度關係含義

若 $A \le_p B$，則隱含以下重要結論：

1. **$A$ 不會比 $B$ 難 (A is no harder than B)**：只要能解 $B$，就能解 $A$。
    
2. **$B$ 至少跟 $A$ 一樣難 (B is at least as hard as A)**：若 $A$ 本身很難，那 $B$ 一定也很難。
    

#### 2.3 具體實例 (Example)

- **問題 A**：「2 能否整除 $x$？」(Instances $\alpha \in \{1, 2, 3...\}$)。
    
- **問題 B**：「$y$ 能否整除 $x$？」(Instances $\beta \in \{(1,1), (1,2)...\}$)。
    
- **Reduction 過程 $f$**：
    
    - 建構轉換函數 $f(\alpha) = (\alpha, 2)$。這可以在 $O(1)$ 時間內完成。
        
    - 如此一來，解決通用的除法問題 B，就能解決特定的除以 2 問題 A。
        

### 3. Reduction 的應用 (Applications of Reduction)

Reduction 主要有三種用途：

1. **設計演算法 (Designing Algorithms)**：
    
    - 若已知如何解 $B$，且 $A \le_p B$，則可直接套用 $B$ 的解法來解 $A$。
        
2. **分類問題 (Classifying Problems)**：
    
    - 若 $A \le_p B$ 且 $B \le_p A$，則 $A$ 與 $B$ 屬於同一個複雜度等級 (Same hardness level)。
        
3. **證明極限 (Proving Limits) - 最重要的應用**：
    
    - **目的**：證明某問題 $B$ 是難解的 (Intractable)，而非為了設計演算法。
        
    - **邏輯**：
        
        - 已知：問題 $A$ 是難的 (例如 $A$ 是 NPC)。
            
        - 證明：$A \le_p B$ (將難題 $A$ **Reduction** 為 $B$)。
            
        - 結論：因為 $B$ 能解決 $A$，所以 $B$ 至少跟 $A$ 一樣難，故 $B$ 也是 NP-Hard。
