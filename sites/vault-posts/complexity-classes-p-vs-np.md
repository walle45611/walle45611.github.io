---
title: "複雜度類別：P 與 NP"
slug: complexity-classes-p-vs-np
topic_section: algorithms
description: "My vault 演算法筆記：複雜度類別：P 與 NP。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Complexity Classes and P vs NP.md"
---

## 1. 基礎複雜度類別 (P 與 NP)

- **複雜度類別 (Complexity Class)**
    
    - 依據資源（如時間、記憶體）消耗來分類問題的集合。
        
    - 此處專注於**多項式時間 (Polynomial Time)** 的討論。
        
- **Class P (Polynomial Time)**
    
    - **定義**：能夠在多項式時間 $O(n^k)$ 內被**解決 (Solved)** 的問題。
        
    - **特徵**：被視為「易解的」或「有效率的」(Tractable/Efficient)。
        
- **Class NP (Non-deterministic Polynomial Time)**
    
    - **定義**：能夠在多項式時間 $O(n^k)$ 內被**驗證 (Verified)** 的問題。
        
    - **關係**：$P \subseteq NP$。
        
    - **邏輯**：如果一個問題能很快被算出來 (P)，那答案當然也能很快被檢查 (NP)。
        

## 2. 進階複雜度定義 (NP-Hard 與 NP-Complete)

- **什麼是「難」問題 (Hard Problem)**
    
    - 目前沒有已知的多項式時間演算法可以解決（例如：K-colorability, Knapsack problem）。
        
    - **多項式時間歸約 (Polynomial-time reduction)**：這是判斷難度的核心方法。
        
        - 觀念：「已知問題 A 很難。若能證明問題 B 至少跟 A 一樣難 (A $\le_p$ B)，那問題 B 也很難。」
            
- **Class NP-Hard**
    
    - **定義**：至少跟 NP 中最難的問題一樣難的問題 (At least as hard as the hardest problems in NP)。
        
    - **注意**：NP-Hard 的問題**不一定**要屬於 NP（它可能連驗證都很難，甚至不可判定）。
        
- **Class NP-Complete (NPC)**
    
    - **定義**：必須同時滿足兩個條件：
        
        1. 屬於 NP (Is in NP)。
            
        2. 屬於 NP-Hard (Is NP-Hard)。
            
    - **地位**：它是 NP 類別中**最難**的問題群。
        
    - **重要性**：只要能找到一個 NPC 問題的高效解法（P解法），就等於解決了所有 NP 問題（即證明 $P=NP$）。
        

## 3. P 與 NP 的關係 ($P \stackrel{?}{=} NP$)
![01-3. P 與 NP 的關係 ($P stackrel{ }{=} NP$](/vault-assets/2fefb61551fd234a86a0.png)
- **已知事實**：$P \subseteq NP$ (P 是 NP 的子集)。
    
- **未解之謎**：是否 $NP \subseteq P$？ (也就是 $P = NP$ 嗎？)
    
    - 這是價值一百萬美元的千禧年大獎難題。
        
- **普遍信念**：大多數電腦科學家相信 **$P \neq NP$**。
    
    - 這意味著，「驗證答案」雖然容易，但不代表「尋找答案」也同樣容易。
        
- **集合關係圖 (Venn Diagrams)**：
    
    - **若 $P \neq NP$**：P 是 NP 內的一個小圓圈，NPC 與 P 完全不重疊（這是主流觀點）。
        
    - **若 $P = NP$**：P、NP、NPC 三者邊界消失，全部重疊在一起。
        

## 4. 不可判定性 (Undecidability) 與 停機問題

這部分跳脫了「解得快慢」，討論的是更根本的「能不能解」。

- **停機問題 (Halting Problem)**
    
    - **問題描述**：給定一個程式 $P$ 和輸入 $x$，能否判斷這個程式會「執行結束 (Halt)」還是「無限迴圈 (Run forever)」？
        
    - **結論**：這是 **不可判定 (Undecidable)** 的。不存在一個通用的演算法能解決所有情況。
        
- **證明方法 (反證法 + 對角線法)**
    
    1. 假設有一個機器 $H(p, x)$ 可以準確判斷程式 $p$ 在輸入 $x$ 下是否會停機。
        
    2. 創造一個新程式 $g(p)$ 利用 $H$ 的結果來運作：
        
        - 如果 $H$ 說 $p$ 會停，$g$ 就故意跑無窮迴圈。
            
        - 如果 $H$ 說 $p$ 不會停，$g$ 就停下來。
            
    3. 當把 $g$ 餵給 $g$ 自己執行時 ($g(g)$)，會產生矛盾：
        
        - 若它停，依定義它就不該停；若它不停，依定義它就該停。
            
    4. 因此假設錯誤，這樣的全能機器 $H$ 不存在。
        

## 5. 複雜度層級總結 (The Hierarchy)

![02-5. 複雜度層級總結 (The Hierarchy)](/vault-assets/1b35dd275e67d5bd9238.png)
根據計算資源與可判定性，由內而外（由簡入難）的同心圓層級關係：

1. **P** (Polynomial Time)：最內層，多項式時間可解。
    
2. **NP** (Nondeterministic Polynomial Time)：多項式時間可驗證，包含 P。
    
3. **PSPACE** (Polynomial Space)：多項式記憶體空間可解，包含 NP。
    
4. **EXPTIME** (Exponential Time)：指數時間可解，包含 PSPACE。
    
5. **Decidable** (Recursive)：可判定問題，有演算法能在有限時間給出答案。
    
6. **Recognizable** (Recursively Enumerable)：可識別問題，若答案為 Yes 可停機，若為 No 可能無窮迴圈。
    
7. **Undecidable**：最外層，無法用演算法解決的問題（如 Halting Problem）。
