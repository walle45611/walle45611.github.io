---
title: "常見 NP 完全問題"
slug: common-np-complete-problems
topic_section: algorithms
description: "My vault 演算法筆記：常見 NP 完全問題。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/NP-C常見的問題.md"
---

#### 命題分析

##### 1) If $A \le_p B$, then $A$ is no harder than $B$

**Verdict: True（對）**

- **解釋：** 因為能解 $B \implies$ 透過歸約就能解 $A$。
    
- $B$ 的解法邏輯上「包含」了 $A$ 的解法（加上一點多項式轉換的時間成本）。
    

##### 2) If $A \le_p B$ and $B \le_p C$, then $A \le_p C$

**Verdict: True（對）**

- **解釋：** 這是歸約的 **傳遞性（Transitivity）**。
    
- 先把 $A$ 轉成 $B$，再把 $B$ 轉成 $C$。兩個多項式時間的轉換合起來，總時間仍是多項式時間。
    

##### 3) NP-Complete problems can be reduced to each other in polynomial time

**Verdict: True（對）**

- **解釋：** 若 $X$ 與 $Y$ 都是 NP-Complete：
    
    1. $X$ 是 NP-hard，根據定義，這表示所有 $L \in NP$ 都可以歸約到 $X$ ($L \le_p X$)。
        
    2. 因為 $Y \in NP$，所以自然得出 $Y \le_p X$。
        
    3. 同理，因為 $X \in NP$ 且 $Y$ 是 NP-hard，所以 $X \le_p Y$。
        
- **結論：** 它們彼此可以互相多項式歸約。
    

##### 4) If $A \le_p B$ and $B$ is NPC, then $A$ is NPC too

**Verdict: False（錯）**

- **解釋：** 方向不對。$A \le_p B$ 只表示 $B$ 的難度 $\ge A$，不能推出 $A$ 就一定很難（$A$ 可能很簡單）。
    
- **反例：**
    
    - $A =$ **2-SAT**（屬於 P，很容易解）
        
    - $B =$ **3-SAT**（屬於 NP-Complete）
        
    - 確實存在 $2\text{-SAT} \le_p 3\text{-SAT}$（簡單問題當然可以歸約到難題），但這不會讓 2-SAT 變成 NP-Complete（除非 $P=NP$）。
        

##### 5) If $A \le_p B$ and $A$ is NPC, then $B$ is NPC too

**Verdict: False（錯）**

- **解釋：** 少了一個必要條件。
    
    - 由於 $A$ 是 NP-hard 且 $A \le_p B$，這確實意味著 $B$ 也是 **NP-hard**（這部分是對的，因為 $B$ 比 $A$ 難）。
        
    - 但要成為 **NP-Complete (NPC)**，還必須滿足條件：$B \in NP$。題目沒給這個前提，所以不能保證。
        
- **反例：**
    
    - $A =$ **SAT**（NP-Complete）
        
    - $B =$ **Halting Problem**（停機問題，不可判定，甚至不在 NP 裡）
        
    - SAT 可以多項式歸約到 Halting Problem，但 Halting Problem 不是 NPC（因為它連 NP 都不是）。
        
- **補充：** 如果題目修正為「若 $A$ 是 NPC 且 $A \le_p B$ 並且 **$B \in NP$**，則 $B$ 是 NPC」，那就會是 **True**。

![01-5) If $A le p B$ and $A$ is NPC, the](/vault-assets/b571aaa2ae64df17ee9e.png)
