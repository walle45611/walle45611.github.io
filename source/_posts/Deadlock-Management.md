---
title: 作業系統-死結管理
tags:
  - Operating System
  - Deadlock
  - Resource Management
categories: System Administration
mathjax: true
abbrlink: 3cd8
date: 2025-01-06 00:00:00
---

## 前情提要

這篇文章主要探討了 死結管理（Deadlock Management） 的核心概念與解決方法。今天的內容涵蓋了死結的定義、發生條件，以及處理死結的不同方法，包括 預防（Prevention）、避免（Avoidance） 和 檢測與恢復（Detection and Recovery）。此外，還詳細介紹了 Banker’s Algorithm 和 資源分配圖（Resource-Allocation Graph） 的應用，以及如何選擇適當策略來處理系統中的死結問題，希望透過這篇文章幫助讀者全面了解死結管理的理論基礎與實務應用。

<!--more-->

![圖 1. : Blog Image](https://imgur.com/VbrcmEY.png)

## 系統模型 (System Model)

- Resource types $R_1,R_2,R_3,...,R_m$ ,CPU cycles,memory space…
- Each resource type $R_i$ has $W_i$ instances.


### 資源類型與實例

執行緒在使用資源前必須請求資源，並且在使用後必須釋放資源。執行緒可以請求所需的任意多個資源來完成指定任務。然而，執行緒請求的資源數量不能超過系統中可用的資源總數。例如：如果系統只有一個網路介面，執行緒不能請求兩個網路介面。

在正常操作模式下，執行緒可以按照以下流程使用資源：

1. **請求（Request）**
    執行緒請求資源。如果請求無法立即被滿足（例如：某個互斥鎖當前被其他執行緒持有），那麼請求執行緒必須等待直到獲得該資源。
2. **使用（Use）**
    執行緒可以操作該資源（例如：如果資源是一個互斥鎖，則執行緒可以訪問其臨界區域）。
3. **釋放（Release）**
    執行緒釋放資源。

---

## 死結的特徵 (Deadlock Characterization)

- 定義：當所有執行緒因資源等待而無法繼續執行，系統進入死結狀態。

## **死結發生的四個必要條件**

1. **互斥 (Mutual Exclusion)**  
   資源一次只能被一個執行緒持有。
2. **持有並等待 (Hold and Wait)**  
   執行緒持有部分資源並等待其他資源。
3. **不可搶占 (No Preemption)**  
   資源只能由持有它的執行緒自願釋放。
4. **環形等待 (Circular Wait)**  
   存在一組執行緒相互等待彼此持有的資源。

![圖 2. : Deadlock](https://imgur.com/qo5gxaX.png){ width=80% }

- 如果每個資源類型只有一個實例，那麼圖中的環就表示已發生死結。
- 如果每個資源類型有多個實例，圖中的環不一定意味著死結。

![圖 3. : Deadlock](https://imgur.com/7Jka2IQ.png){ width=80% }

- 如果資源分配圖中沒有環，那麼系統就不會處於死結狀態
- 如果有環，那麼系統可能會或可能不會處於死結狀態

## Methods for Handling Deadlocks

### Deadlock Prevention

1. **Mutual Exclusion (互斥)：**
    - 某些資源（如打印機）是不可共享的。
    - 共享資源（如只讀文件）不會導致死結，因為它們可以被多個 process 同時訪問。
    - *重點：**互斥條件無法破壞，因為某些資源本質上不可共享。
2. **Hold and Wait (持有並等待)：**
    - **解決方法：**
        - 協議 1： process 在執行前請求所有需要的資源，若無法獲得，則等待。
        - 協議 2： process 只能在未持有任何資源時，才可以請求資源。
    - **缺點：**
        - 資源利用率低（分配但未使用）。
        - 可能導致飢餓問題（某些 process 無限等待資源）。
3. **No Preemption (不可搶占)：**
    - **解決方法：**
        - 若 process 請求的資源無法分配，則所有已分配的資源將被收回（Preempted），該 process 進入等待狀態。
        - 應用：適用於可以保存並恢復狀態的資源，如 CPU 寄存器和記憶體空間。
4. **Circular Wait (環形等待)：**
    - 給每個資源分配一個唯一的編號，並要求 process 按照資源編號的升序請求資源。
    - 如果一個 process 已經持有一個資源，則只能請求比該資源編號大的其他資源。

    ![圖 4. : Circular Wait](https://imgur.com/waVqieK.png)

    - **process 可以初次請求任意數量的資源 $R_i$**：
        - process P1 最初可以請求任何資源 Ri。
        - 一旦 process 請求到資源 Ri，其後的資源請求必須遵守一個順序規則。
    - **之後的資源請求必須滿足** $F(R_j) > F(R_i)$**：**
        - $F(R_i)$ 是資源 $R_i$ 的唯一整數編號。
        - 若 process $P_1$ 已經持有資源 $R_2$，則只能請求編號比 $F(R_2)$ 大的資源（如 $R_3,R_4$）。
    - **若需要請求編號較小的資源，必須釋放較大的資源**：
        - 如果 P1 想請求 R1，但已持有 R2，則需要先釋放 R2，再重新請求 R1。
    - **Case 1：F(R3) = 10**
        - P1 持有 R2（F(R2) = 5）。
        - P1 請求 R3 時滿足條件 F(Rj) > F(Ri)，所以可以成功請求 R3。
    - **Case 2：F(R1) = 3**
        - P1 持有 R2（F(R2) = 5）。
        - P1 請求 R1 時不滿足條件 F(Rj) > F(Ri)，因此無法直接請求。
        - 必須先釋放 R2，然後重新請求 R1。

- 那麼 1~3 沒有辦法有效解決，所以會朝著有沒有安全的順序或是思路解決這個問題。

---

### Deadlock Avoidance

#### 核心特徵

1. **process 需提前聲明資源需求**。
2. 系統動態檢查狀態以避免進入 Unsafe State。

#### Safe State 和 Unsafe State

- **Safe State**  
   系統能找到安全執行序列（Safe Sequence）。
- **Unsafe State**  
   無法保證所有執行緒完成執行。

---

### Banker’s Algorithm

- **適用於多個資源實例**
- **銀行家演算法的效率低於資源分配圖方案**
- 每個程序必須事先聲明其最大資源需求量
- 當某個程序請求資源時，可能需要等待
- 當某個程序獲得所有所需資源後，必須在有限的時間內釋放這些資源
- **動態資源分配：** 在 process 提出資源請求時，檢查請求是否可以安全地被滿足。
- **避免死結：** 保證系統在任何時間點都處於「安全狀態」，從而避免死結的發生。

#### Banker’s Algorithm 資料結構

1. **Available**：目前可用資源數量。
2. **Max**： process 宣告的最大資源需求。
3. **Allocation**：已分配給 process 的資源數量。
4. **Need**：尚需的資源數量，$Need[i][j] = Max[i][j] - Allocation[i][j]$。

#### Banker’s Algorithm 演算法流程

1. **檢查請求合法性**
   - 若 $Request[i][j] > Need[i][j]$，拒絕請求。
   - 若 $Request[i][j] > Available[j]$，執行緒進入等待。
2. **試探分配**
   - 假設資源分配，更新狀態（Resource-Request Algorithm）：
     - $Available[j] -= Request[i][j]$
     - $Allocation[i][j] += Request[i][j]$
     - $Need[i][j] -= Request[i][j]$
3. **安全性檢查 (Safety Algorithm)**
   - 初始化：
     - $Work = Available$
     - $Finish[i] = false$。
   - 檢查條件：
     - 找到滿足 $Finish[i] = false$ 且 $Need[i] \leq Work$ 的 process  $T_i$。
   - 模擬完成：
     - 更新 $Work$ 為 $Work + Allocation[i]$，設置 $Finish[i] = true$。
   - 若所有 $Finish[i] = true$，則系統安全。

---

### Resource-Allocation Graph

- A directed edge $T_i \rightarrow R_j$ is called a **request edge**.
- A directed edge $R_j \rightarrow Ti$ is called an **assignment edge**.
- 範例
    1. A system is composed of four processes, {P1, P2, P3, P4}, and three types of serially reusable resources, {R1, R2, R3}. The number of units of the resources are <3,2,2>.
  - P1 hold 1 unit of R1 and request 1 unit of R2.
  - P2 hold 2 unit of R2 and request 1 unit each of R1 and R3.
  - P3 hold 1 unit of R1 and request 1 unit of R2.
  - P4 hold 2 unit of R3 and request 1 unit of R1.

    Show the resource-allocation graph to represent this system state.

    Can deadlock occur? If so, what processes are deadlocked in this state? If not, give a safe sequence for this state.

    ![圖 5. : Resource-Allocation Graph](https://imgur.com/gmOIGtC.png){ width=50% }

#### Resource-Allocation Graph Algorithm

- Claim Edge
  - **Claim Edge** 用於表示一個 process 可能需要某資源的潛在需求，但該資源目前尚未被請求。
  - **圖中表示方式：** 使用虛線 (dashed line)。
  - 範例：假設 process  T1 可能需要資源 R1，在資源分配圖中會以 T1→R1 的虛線表示。

1. **僅適用於單一實例資源：**
    - 若資源有多個實例（如多台磁碟機），**Claim Edge** 不能很好地表示。
    - 此時需使用 **Banker's Algorithm** 進行分析。
2. **需要靜態定義需求：**
    - process 必須在執行開始前，明確宣告所有潛在需求，否則無法建立 **Claim Edge**。

---

### 比較 Banker’s Algorithm 和 Resource-Allocation Graph Algorithm

| **比較項目** | **Banker's Algorithm** | **Resource-Allocation Graph** |
| ------------ | ---------------------- | ----------------------------- |
| **用途**     | 多實例資源死結避免     | 單實例資源死結避免            |
| **檢測方法** | Safety Algorithm       | Cycle-Detection Algorithm     |
| **效率**     | $O(m \cdot n^2)$       | $O(n^2)$                      |
| **應用場景** | 動態、多類型資源       | 單類型、靜態資源              |

---

## Deadlock Detection

### wait-for graph

![圖 6. : wait-for graph](https://imgur.com/yTPxwCP.png){ width=70% }

- 就是把 resources 移除掉，且只能使用在 **signal instance**
- 如果有 cycle 存在那麼就是有 deadlock

### Detection Algorithm

#### Detection Algorithm 資料結構

1. **Work (向量)**：
    - 代表系統中目前的可用資源。
    - 初始值設定為 `Available`。
2. **Finish (向量)**：
    - Boolean 陣列，表示某個 process 是否能完成（`true` 表示可以完成，`false` 表示無法完成）。
    - 初始化為 `false`，如果 $Allocation[i] == 0$（ process 未持有任何資源），則設為 `true`。
3. **Allocation (矩陣)**：
    - 表示當前每個 process 已經獲得的資源數量。
4. **Request (矩陣)**：
    - 表示每個 process 尚需要的額外資源。

#### Detection Algorithm 演算法流程

1. 初始化：
   - $Work = Available$。
   - $Finish[i] = false$。
2. 查找可執行 process ：
   - 找到 $Request[i] \leq Work$ 的 process 。
3. 模擬執行：
   - 將 process  $T_i$ 的資源釋放至 $Work$。
4. 檢查 Deadlock 狀態：
   - 若存在 $Finish[i] = false$，則系統處於有 Deadlock 狀態。

---

## Recover from Deadlock

1. **過程與執行緒終止（Process and Thread Termination）**

   - **Abort all deadlocked processes:**
     - 終止所有處於死結狀態的執行緒。
   - **Abort one process at a time:**
     - 逐步終止執行緒，直到消除死結循環。

2. **部分終止方法（Partial Termination Method）**
    - 終止帶來**最小成本**的執行緒：
        - 考量以下因素決定終止順序：
            1. 執行緒的**優先級**。
            2. 執行緒已運行的時間，與完成所需的剩餘時間。
            3. 執行緒已使用的資源數量。
            4. 執行緒完成任務所需的資源數量。
            5. 需終止的執行緒數量。
            6. 執行緒是**互動式（interactive）還是批次式（batch）**。

- **Resource Preemption**：
  - **Selecting a Victim:** 選擇影響最小的執行緒進行資源回收。
  - **Rollback:** 回滾執行緒至安全狀態，重新啟動執行。
  - **Starvation:**
    - 相同執行緒可能多次被選為犧牲者。應在成本考量中包含**回滾次數**
