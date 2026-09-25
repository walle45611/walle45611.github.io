---
title: "陣列記憶體位址計算"
slug: array-memory-address
topic_section: data-structures
description: "陣列記憶體位址計算的重點整理。"
date: 2026-09-24
blog: true
vault_source: "Note/Research/Array 記憶體位址計算全攻略.md"
---

### 1. 基礎觀念：一維陣列 (1-Dim Array)

- **公式**：$Loc(A[i]) = L_0 + (i - l) \times d$
    

---

### 2. 進階觀念：二維陣列 (2-Dim Array)

- **宣告**：$A[l_1 \ldots u_1, \ l_2 \ldots u_2]$
    
- **列數 (Rows)**：$R = u_1 - l_1 + 1$
    
- **行數 (Cols)**：$C = u_2 - l_2 + 1$
    

#### 通用公式

- Row-major (以列為主 - C/C++)：
    
    $$Loc(A[i,j]) = L_0 + [(i - l_1) \times C + (j - l_2)] \times d$$
    
- Column-major (以行為主 - Fortran)：
    
    $$Loc(A[i,j]) = L_0 + [(j - l_2) \times R + (i - l_1)] \times d$$
    

---

### 3. 四大常考題型攻略 (含範例詳解)

#### 🔥 型一：給定所有參數，求位址

**題目**：陣列 $A=[-3\ldots8,\,-1\ldots14]$，起始位址 $L_0=100$，元素大小 $d=4$。

**1. Row-major (求 $A[5,7]$)**

- **先算行數 (Cols)**：$C = 14 - (-1) + 1 = 16$
    
- **代入公式**：
    
    $$ \begin{aligned} Loc(5,7) &= 100 + \big((5 - (-3)) \times 16 + (7 - (-1))\big) \times 4 \\ &= 100 + \big(8 \times 16 + 8\big) \times 4 \\ &= 100 + (128 + 8) \times 4 \\ &= 100 + 544 = \boxed{644} \end{aligned}$$
    

**2. Column-major (求 $A[3,12]$)**

- **先算列數 (Rows)**：$R = 8 - (-3) + 1 = 12$
    
- **代入公式**：
    
    $$ \begin{aligned} Loc(3,12) &= 100 + \big((12 - (-1)) \times 12 + (3 - (-3))\big) \times 4 \\ &= 100 + \big(13 \times 12 + 6\big) \times 4 \\ &= 100 + (156 + 6) \times 4 \\ &= 100 + 162 \times 4 = \boxed{748} \end{aligned}$$
    

---

#### 🔥 型二：給 2 個元素位址，判斷 Row/Col Major

**題目**：$A(3,2)=1110$，$A(2,3)=1115$，且 $d=1$。求儲存方式與 $A(5,4)$。

**1. 判斷方式 (相對位移法)**

- $\Delta i = 2-3 = -1$
    
- $\Delta j = 3-2 = 1$
    
- $\Delta Addr = 1115 - 1110 = 5$
    
- **測試 Row-major**：$5 = (-1 \times C + 1) \times 1 \Rightarrow C=-4$ (不合理，刪去)
    
- **測試 Col-major**：$5 = (1 \times R + (-1)) \times 1 \Rightarrow R=6$ (**合理！**)
    
- **結論**：Column-major，且總列數 $R=6$。
    

**2. 求 $A(5,4)$ (利用平移)**

- 從 $A(3,2)=1110$ 出發到 $(5,4)$。
    
- 公式：$Loc = 1110 + [(4-2)\times 6 + (5-3)] \times 1$
    
- 計算：$1110 + [2 \times 6 + 2] = 1110 + 14 = \boxed{1124}$
    

---

#### 🔥 型三：已知 2 點位址，求維度與第三點

**題目**：$A[1\ldots m, 1\ldots n]$，$d=1$，已知 $A(3,3)=121$，$A(6,4)=159$。求 $A(10,7)$。

**1. 找出維度 (Row 還是 Col?)**

- $\Delta i = 3, \Delta j = 1, \Delta Addr = 38$
    
- **若 Row-major**：$38 = (3n + 1) \Rightarrow 3n=37 \Rightarrow n=12.33$ (非整數，剔除)
    
- **若 Col-major**：$38 = (1m + 3) \Rightarrow m=35$ (**合理！**)
    
- **結論**：Column-major，且 $m=35$。
    

**2. 求 $A(10,7)$**

- 從 $A(3,3)=121$ 平移到 $(10,7)$。
    
- $\Delta i' = 7, \Delta j' = 4$
    
- $Loc = 121 + (4 \times 35 + 7) \times 1 = 121 + 147 = \boxed{268}$
    

---

#### 🔥 型四：給 3 個元素位址，解全套 (B, n, m, d)

**題目**：$A(2,3)=18, A(3,2)=28, A(1,1)=2$。求 $A(4,5)$。

1. 假設 Row-major 並列式

通式：$Addr = B + ((i-1)n + (j-1))d$

- $A(2,3) \to 18 = 2 + (n + 2)d$ (因為 $B=A(1,1)=2$) ...①
    
- $A(3,2) \to 28 = 2 + (2n + 1)d$ ...②
    

**2. 解聯立**

- 整理 ①：$nd + 2d = 16$
    
- 整理 ②：$2nd + d = 26$
    
- ② - ① $\times 2$ (消去 $nd$)：
    
    $(2nd+d) - (2nd+4d) = 26 - 32 \Rightarrow -3d = -6 \Rightarrow \boxed{d=2}$
    
- 代回 ①：$2n + 4 = 16 \Rightarrow 2n=12 \Rightarrow \boxed{n=6}$
    
- 檢查：$n, d$ 皆為正整數，假設成立。(若是分數則改設 Column-major)
    

**3. 求 $A(4,5)$**

- $Loc = 2 + ((4-1)\times 6 + (5-1)) \times 2$
    
- $Loc = 2 + (18 + 4) \times 2 = 2 + 44 = \boxed{46}$
    

---

### 4. 高維度陣列 (3D, 4D, N-Dim)

#### 3D Row-Major 實戰例題

**題目**：宣告 $A[1\dots5, 2\dots4, 0\dots3]$，Row-major，$L_0=1000, d=2$。求 $A[3, 3, 2]$。

**1. 準備參數**

- **各維度大小 ($S_k$)**：
    
    - $S_1 = 5-1+1 = 5$
        
    - $S_2 = 4-2+1 = 3$
        
    - $S_3 = 3-0+1 = 4$
        
- **索引偏移 ($idx$)**：
    
    - $i_{off} = 3-1 = 2$
        
    - $j_{off} = 3-2 = 1$
        
    - $k_{off} = 2-0 = 2$
        

2. 方法 A：公式法

$$Loc = L_0 + [(i_{off})S_2 S_3 + (j_{off})S_3 + k_{off}] \times d$$

$$Loc = 1000 + [2 \times (3 \times 4) + 1 \times 4 + 2] \times 2$$

$$Loc = 1000 + [24 + 4 + 2] \times 2 = 1000 + 60 = \boxed{1060}$$

3. 方法 B：霍納法則 (括號法) - 推薦考試用

$$Loc = L_0 + \Big( \big( (i_{off})S_2 + j_{off} \big) S_3 + k_{off} \Big) \times d$$

$$Loc = 1000 + \Big( \big( 2 \times 3 + 1 \big) \times 4 + 2 \Big) \times 2$$

$$Loc = 1000 + (7 \times 4 + 2) \times 2 = 1000 + 30 \times 2 = \boxed{1060}$$

---

### 5. N 維陣列通用公式

宣告：$A[n_1, n_2, \ldots, n_k]$

Row-major 通用公式：

$$Loc = L_0 + \left( \sum_{x=1}^{k} (i_x - l_x) \cdot (\prod_{y=x+1}^{k} S_y) \right) \cdot d$$

N 維霍納法則：

$$Offset = (\dots((i_1-l_1)S_2 + (i_2-l_2))S_3 + \dots)S_n + (i_n-l_n)$$

(口訣：上一層結果乘以下一層 Size，再加上下一層偏移)
