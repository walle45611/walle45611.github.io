---
blog: true
blog_title: Approximation Algorithm：近似比與證明
blog_date: 2026-11-12
blog_url: https://blog.walle4561.com/articles/posts/approximation-algorithm/
blog_tag: DSA
blog_topic: 演算法
---

## Decision Problem、NP 與 NP-complete

Decision Problem問「是否存在符合條件的解？」，答案是 Yes / No；Optimization Problem則要找出最小值、最大值或對應的最佳解。

例如 TSP 的Decision Version問「是否存在成本不超過 $K$ 的 Tour？」；Optimization Version則問「成本最低的 Tour 是什麼？」。給定一條成本不超過 $K$ 的 Tour，只能證明答案是 Yes，不需要證明它是最佳解。

**P：** 能在多項式時間內解決的Decision Problem。**NP：** 對答案為 Yes 的實例，存在長度為輸入規模多項式的 Certificate（候選解），且可在多項式時間內驗證；答案為 No 的實例，不能有任何 Certificate 通過驗證。

因此，證明問題屬於 NP，要交代候選解的形式、長度與驗證方法。能快速驗證候選解，並不代表已經知道如何快速找到解；NP 也不代表一定難解，因為 $P \subseteq NP$。

**NP-hard：** 所有 NP 問題都能多項式時間歸約到它。實際證明時，通常選一個已知的 NP-complete 問題 $A$，證明 $A \leq_p B$，便能推出 $B$ 是 NP-hard。

**NP-complete（NPC）：** 同時滿足 $B \in NP$ 與 $B$ 是 NP-hard。這兩件事都要證明，不能只證明可以快速驗證，就說它是 NPC。

$A \leq_p B$ 表示能在多項式時間內，把 $A$ 的實例轉成 $B$ 的實例，且保留 Yes / No 答案。因此，若能快速解決 $B$，就能快速解決 $A$；證明新問題很難時，方向是「已知難題 → 新問題」。

標準定義下，NP 與 NP-complete 分類的是 Decision Problem。因此，下文 WVC、TSP、MAX-CUT 的 Decision Version 是 NP-complete，對應的Optimization Version則是 NP-hard。這不排除Optimization Version存在有效率的近似演算法。

## 近似比要證明什麼

近似演算法要在多項式時間內找到合法解，再證明解的品質相對於最佳值 $OPT$ 有保證。令 $n$ 為輸入編碼長度，$C$ 為演算法輸出的目標值，$C^*=OPT$ 為最佳值；這一段的 $C$ 是數值，不是後面 Vertex Cover 的頂點集合。

**Approximation Ratio 的定義：** 對每個輸入實例，在目標值為正時，若 $\max\left\{\frac{C}{C^*},\frac{C^*}{C}\right\} \leq \rho(n)$，就稱為 $\rho(n)$-Approximation Algorithm。後面也以 $ALG$ 表示演算法輸出的目標值。

==最小化問題中 $C \geq C^*$，因此取 $C/C^*$；最大化問題中 $C \leq C^*$，因此取 $C^*/C$。兩者都採用 $\rho(n) \geq 1$ 的慣例，數值越接近 1，保證越好。$\rho(n)=1$ 代表演算法對所有輸入都能找到最佳解；但近似演算法也可能在某個實例剛好找到最佳解==。

若最佳值為 0，則用下方不涉及除法的不等式描述保證，避免除以 0。$\rho(n)$ 可以依輸入規模改變；以下的 2 與 $8/7$ 則是固定常數近似比。

**最小化問題：** $OPT \leq ALG \leq \rho OPT$。例如 2-Approximation 表示成本最多是最佳成本的兩倍。

**最大化問題：** $OPT/\rho \leq ALG \leq OPT$。例如 2-Approximation 表示結果至少是最佳值的一半，也常稱為 $1/2$-Approximation。

Randomized Approximation 以下討論的是期望值保證。例如 MAX-CUT 的 $\mathbb{E}[ALG] \geq OPT/2$，不代表每次執行都一定達到 $OPT/2$。

NP-complete 證明是在說明問題的計算難度；近似比證明是在說明演算法找到的解有多好。兩者是不同的證明。

## 一、利用 Reduction 證明Decision Version是 NP-complete

這類證明要完成兩件事：證明目標問題屬於 NP，再從已知的 NPC 問題歸約到目標問題。

### 證明 Weighted Vertex Cover 的Decision Version是 NP-complete

Vertex Cover 要選擇頂點集合 $C \subseteq V$，使每條邊至少有一個端點在 $C$ 中。一般版本最小化 $|C|$；加權版本最小化 $\sum_{v \in C} w(v)$，以下權重皆為非負整數。Decision Version另外給定上限 $B$，問是否存在總權重不超過 $B$ 的 Vertex Cover。

**定理：** Weighted Vertex Cover（加權頂點覆蓋問題）的 Decision Version 是 NP-Complete。

**證明（Proof）：**

(1) WVC $\in$ NP

給定一個頂點集合 $C$，我們可以在多項式時間內驗證每一條邊是否都被覆蓋，以及所有被選中頂點的權重總和是否滿足： $\sum_{v \in C} w(v) \leq B$ 因此，WVC 屬於 NP。

(2) WVC 是 NP-hard

我們從已知的 NP-Complete 問題 Vertex Cover（VC）進行歸約。

給定一個 VC 的問題實例 $(G,k)$，我們將它轉換成 WVC 的問題實例 $(G,w,B)$，其中將所有頂點的權重設為 $w(v)=1$，並設定 $B=k$。

因此： $|C| \leq k \iff \sum_{v \in C} w(v) \leq B$ 這表示原本的 VC 存在符合條件的解，若且唯若轉換後的 WVC 也存在符合條件的解，也就是兩個問題實例具有相同的 YES/NO 答案。

由於這個轉換可以在多項式時間內完成，因此： $\mathrm{VC} \leq_p \mathrm{WVC}$ 所以 WVC 是 NP-hard。

**結論：** 因為 WVC 同時屬於 NP 且是 NP-hard，因此： $\boxed{\text{Weighted Vertex Cover is NP-Complete.}}$

### 證明 Metric TSP 的Decision Version是 NP-complete

Metric TSP 以下指無向、非負邊成本、滿足 Triangle Inequality 的 Complete Graph，亦即 $c(u,v) \leq c(u,z)+c(z,v)$。Decision Version問是否存在成本不超過 $K$ 的 Tour。

給定任意 HC Instance，透過 Polynomial-time Reduction 將其轉換成 Complete Graph。原有邊的 Cost 設為 1，新增邊的 Cost 設為 2，並設定 $K=n$。

由於所有邊的 Cost 都介於 1 和 2 之間，因此滿足 Triangle Inequality，符合 Metric TSP 的條件。

若 HC 有解，則存在一條 Cost = $n$ 的 TSP Tour；若 HC 無解，則任何 TSP Tour 都必須使用至少一條 Cost = 2 的邊，因此總成本大於 $n$。

所以： $\mathrm{HC} = \text{Yes} \iff \text{Metric TSP} = \text{Yes}$ 因此 Metric TSP 是 NP-hard。給定一條 Tour，可以在多項式時間內檢查是否恰好拜訪每個頂點一次並回到起點，以及總成本是否不超過 $K$，因此Decision Version也屬於 NP。所以 Metric TSP 的Decision Version是 NP-complete。

## 補充：利用 Reduction 證明 General TSP 的不可近似性

這段不是 NPC 證明，也不是設計近似演算法，而是假設存在固定常數近似比演算法，再利用它解決 HC，推出 $P=NP$。

General TSP 不要求 Triangle Inequality（三角不等式）。令 $n=|V|$，HC（Hamiltonian Cycle）問是否存在恰好拜訪每個頂點一次、最後回到起點的環。

假設 General TSP 存在 Polynomial-time $\rho$-Approximation Algorithm。給定任意 HC Instance，將其轉換成 Complete Graph，原有邊 Cost 設為 1，新增邊 Cost 設為 $M>\rho n$。

若 HC 有解，則 $OPT=n$，近似演算法輸出成本小於 $M$；若 HC 無解，則任何 Tour 的成本至少為 $M$。

因此可以在 Polynomial Time 內判斷 HC，得到 $HC\in P$。由於 HC 是 NP-complete，推出 $P=NP$，與假設的 $P\neq NP$ 矛盾。

故除非 $P=NP$，General TSP 不存在多項式時間的固定常數近似比演算法。

## 二、證明確定性演算法是 2-Approximation

先說明如何在多項式時間內產生合法解，再證明最小化問題滿足 $ALG \leq 2OPT$，或最大化問題滿足 $ALG \geq OPT/2$。這裡的保證適用於每次執行。

### 證明 Weighted Vertex Cover 的 LP Rounding 是 2-Approximation

使用 ILP（整數線性規劃）建模：令 $x_v=1$ 表示選取頂點 $v$，$x_v=0$ 表示不選。目標為 $\min \sum_{v \in V} w(v)x_v$，限制為每條邊 $(u,v) \in E$ 都滿足 $x_u+x_v \geq 1$，且 $x_v \in \{0,1\}$。

**ILP、LP 與 Relaxation：** ILP 要求變數是整數，這裡更限定為 0 或 1，表示「選或不選」。LP（線性規劃）則允許變數取連續值；LP Relaxation 是放寬原本 ILP 的整數限制，保留線性的目標函數與限制式。

這裡的 LP 模型仍最小化 $\sum_{v \in V}w(v)x_v$，每條邊仍須滿足 $x_u+x_v \geq 1$，但把 $x_v \in \{0,1\}$ 改成 $0 \leq x_v \leq 1$。因此 $x_v$ 可以是 0.5 等小數，LP 解不能直接當成選取頂點的結果，還需要 Rounding。

LP 可以在輸入編碼長度的多項式時間內求解。因為放寬後每個 ILP 可行解也都是 LP 可行解，且這是最小化問題，所以 $OPT_{LP} \leq OPT_{ILP}=OPT$。也就是 LP 最佳值提供原問題的 Lower Bound。

解出 LP 最佳解 $x^*$ 後，選取所有 $x_v^* \geq 1/2$ 的頂點，得到 $C=\{v \in V \mid x_v^* \geq 1/2\}$。

**合法性：** 每條邊都滿足 $x_u^*+x_v^* \geq 1$，因此兩端點不可能同時小於 $1/2$，至少一個端點會被選入 $C$。

**近似比：** 因為權重非負，且選中的頂點皆滿足 $x_v^* \geq 1/2$，所以 $OPT_{LP}=\sum_{v \in V}w(v)x_v^* \geq \sum_{v \in C}w(v)x_v^* \geq \frac12\sum_{v \in C}w(v)=\frac12 w(C)$。

因此 $w(C) \leq 2OPT_{LP} \leq 2OPT$，證明 LP Rounding 是 Weighted Vertex Cover 的 2-Approximation。

**Approximation Algorithm（Pseudocode）：**

```text
APPROX-WEIGHTED-VERTEX-COVER(G = (V, E), w)
    x* ← Solve-LP(
        minimize Σ(v ∈ V) w(v) · x[v],
        subject to x[u] + x[v] ≥ 1 for every (u, v) ∈ E,
                   0 ≤ x[v] ≤ 1 for every v ∈ V
    )
    C ← ∅
    for each vertex v ∈ V
        if x*[v] ≥ 1/2
            C ← C ∪ {v}
    return C
```

### 證明 Metric TSP 的 MST 演算法是 2-Approximation

給定任意 Metric Complete Graph，先使用 Prim 或 Kruskal 找出 Minimum Spanning Tree（MST）$T^*$。這裡不需要先做 HC Reduction，邊成本也不必限定為 1 或 2。

將 MST 每條邊複製一次，得到連通且所有頂點 Degree 都是偶數的多重圖，因此可以找到 Eulerian Circuit。這個環使用每條複製後的邊恰好一次，總成本為 $2w(T^*)$，但可能重複拜訪頂點。

接著做 Shortcut：跳過已拜訪過的頂點，直接前往下一個尚未拜訪的頂點，最後回到起點，得到合法的 TSP Tour $H$。使用的是原本 Complete Graph 中的邊。

例如 $A \to B \to A \to C \to A \to D \to A$ 可縮短為 $A \to B \to C \to D \to A$。因為 Triangle Inequality，例如 $c(B,C) \leq c(B,A)+c(A,C)$，所以 Shortcut 不會增加成本，得到 $w(H) \leq 2w(T^*)$。

令 $H^*$ 為最佳 TSP Tour。從 $H^*$ 刪除任意一條邊，就得到一棵 Spanning Tree；MST 不會比這棵樹更貴，因此 $w(T^*) \leq w(H^*)=OPT$。

合併可得 $w(H) \leq 2w(T^*) \leq 2w(H^*)=2OPT$。各步驟都能在多項式時間內完成，因此這是 2-Approximation。

**兩個證明要分開：** HC Reduction 的 1、2 邊成本用來證明Decision Version是 NP-complete；MST、Eulerian Circuit 與 Shortcut 用來證明Optimization Version存在 2-Approximation。

**Approximation Algorithm（Pseudocode）：**

```text
APPROX-METRIC-TSP(G = (V, E), c)
    T ← MST(G, c)                         // 使用 Prim 或 Kruskal
    D ← 將 T 的每條邊複製一次，形成多重圖
    W ← EULERIAN-CIRCUIT(D)
    H ← 空串列
    visited ← ∅
    for each vertex v in W, in traversal order
        if v ∉ visited
            append v to H                 // 跳過重複頂點
            visited ← visited ∪ {v}
    append H[1] to H                      // 回到起點
    return H
```

### 證明 MAX-CUT 的 Greedy Algorithm 是 2-Approximation

依任意順序處理頂點 $v_1,\ldots,v_n$。處理 $v_i$ 時，令 $a_i$ 為它連到 $A$ 中已分配頂點的邊數，$b_i$ 為它連到 $B$ 中已分配頂點的邊數；尚未處理的鄰居不計入。

如果 $a_i \geq b_i$，就把 $v_i$ 放進 $B$，否則放進 $A$。如此新增的 Cut Edges 為 $\max(a_i,b_i) \geq (a_i+b_i)/2$。

每條邊會在「兩端點中較晚被處理的頂點」被處理時，恰好計算一次。較早處理端點時，另一端尚未分配，所以不計；較晚處理端點時，另一端已分配，所以計入。因此 $\sum_{i=1}^{n}(a_i+b_i)=m$。

例如三角形依序處理 $v_1,v_2,v_3$，連到已處理頂點的邊數分別為 0、1、2，因此總和是 $0+1+2=3=m$。$a_i+b_i$ 不是頂點的全部 Degree，而是連到先前已處理頂點的邊數。

因此 $ALG=\sum_i\max(a_i,b_i) \geq \frac12\sum_i(a_i+b_i)=m/2 \geq OPT/2$，證明 Greedy 是 2-Approximation。這裡每次執行都有保證，與 Randomized 的期望值保證不同。

**Approximation Algorithm（Pseudocode）：**

```text
GREEDY-MAX-CUT(G = (V, E))
    A ← ∅
    B ← ∅
    for each vertex v ∈ V, in a fixed order
        a ← number of neighbors of v already in A
        b ← number of neighbors of v already in B
        if a ≥ b
            B ← B ∪ {v}
        else
            A ← A ∪ {v}
    return (A, B)
```

### 無權重 Vertex Cover 的 2-Approximation 與 Greedy 注意事項

Prim 和 Kruskal 都是 Greedy Algorithm，而且能找到真正的 MST 最佳解。使用 Greedy 並不表示只得到近似解，也不表示任何 Greedy 策略都有相同的近似比。

一般無權重 Vertex Cover 的經典 2-Approximation，是每次選一條尚未覆蓋的邊 $(u,v)$，把兩個端點都加入 $C$，再移除所有與它們相連的邊，直到沒有邊剩下。

假設總共選了 $k$ 條邊，這些邊沒有共同端點，形成 Matching。任何 Vertex Cover 都至少要為每條 Matching Edge 選一個端點，所以 $OPT \geq k$；演算法選了 $2k$ 個頂點，因此 $|C|=2k \leq 2OPT$。

「每次選擇覆蓋最多尚未覆蓋邊的頂點」是另一種策略，不能直接保證 2-Approximation。上述選兩端點的證明適用於無權重版本；加權版本可使用前面的 LP Rounding。

**Approximation Algorithm（Pseudocode）：**

```text
APPROX-VERTEX-COVER(G = (V, E))
    C ← ∅
    E_remaining ← E
    while E_remaining ≠ ∅
        choose any edge (u, v) ∈ E_remaining
        C ← C ∪ {u, v}
        remove every edge incident to u or v from E_remaining
    return C
```

## 三、證明 Randomized Approximation 的期望近似比

先計算每個事件成功的機率，利用期望值的線性性求出 $\mathbb{E}[ALG]$，再與 $OPT$ 比較。以下證明的是期望值保證。

### MAX-CUT：期望 2-Approximation

給定無自環的無向、無權重圖 $G=(V,E)$，令 $n=|V|$ 為頂點數、$m=|E|$ 為邊數。MAX-CUT 要將頂點分成不相交的集合 $A$、$B$，使跨越兩個集合的邊數最多；這些邊稱為 Cut Edges。

讓每個頂點獨立以 $1/2$ 的機率加入 $A$ 或 $B$。對任意邊 $(u,v)$，兩端點有 $(A,A)$、$(A,B)$、$(B,A)$、$(B,B)$ 四種等機率分配，其中兩種是 Cut Edge，因此被 Cut 的機率是 $1/2$。

令 $X_e$ 在邊 $e$ 被 Cut 時為 1，否則為 0。利用 Linearity of Expectation（期望值的線性性），可得 $\mathbb{E}[ALG]=\sum_{e \in E}\mathbb{E}[X_e]=m/2$；這一步不需要不同邊的 $X_e$ 彼此獨立。

因為 $OPT \leq m$，所以 $\mathbb{E}[ALG]=m/2 \geq OPT/2$。因此這是期望近似比為 2 的隨機演算法，而非每次執行都保證至少 Cut 一半的邊。

**Approximation Algorithm（Pseudocode）：**

```text
RANDOMIZED-MAX-CUT(G = (V, E))
    A ← ∅
    B ← ∅
    for each vertex v ∈ V
        independently sample r uniformly from {0, 1}
        if r = 0
            A ← A ∪ {v}
        else
            B ← B ∪ {v}
    return (A, B)
```

### MAX-3-CNF-SAT：期望 $8/7$-Approximation

若 MAX-3-CNF-SAT 的每個 Clause 恰含三個不同變數的 Literal，獨立隨機指定每個變數的 True / False，則每個 Clause 滿足的機率為 $1-(1/2)^3=7/8$。令 $m$ 為 Clause 數，便有 $\mathbb{E}[ALG]=7m/8 \geq 7OPT/8$，得到期望近似比 $8/7$。這裡的 $m$ 是 Clause 數，MAX-CUT 的 $m$ 則是 Edge 數。

**Approximation Algorithm（Pseudocode）：**

```text
RANDOMIZED-MAX-3-CNF-SAT(F)
    for each variable x appearing in F
        independently assign x ← True or False, each with probability 1/2
    return the assignment
```
