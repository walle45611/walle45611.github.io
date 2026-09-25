
由 **Leslie Lamport** 於 1974 年提出，這是一個具有里程碑意義的演算法。它證明了在沒有硬體原子指令 (Atomic Instructions) 的支援下，僅使用最基礎的 **安全暫存器 (Safe Registers)** 讀寫操作，就能解決 $N$ 個 Process 的互斥問題。

## 1. Core Concept
演算法的核心思想源自麵包店的取號機制，並嚴格遵循 **先到先服務 (FCFS)** 原則。

> [!TIP] The Bakery Analogy (麵包店類比)
> 
> 顧客 (Process) 進店時，先在門口抽一張號碼牌。
> 
> - **規則 1**：號碼最小的人先接受服務。
>     
> - **規則 2**：若兩個人抽到一樣的號碼（因為同時抽），則依照身分證字號 (Process ID) 較小的人優先。
>     

### Lexicographical Order (優先權比較規則)

系統使用 (ticket #, process ID) 的來決定優先順序。

定義 $(a, b) < (c, d)$ 為真，若：

1. $a < c$ (號碼牌較小者優先)
    
2. 或 $a == c$ 且 $b < d$ (號碼相同時，ID 較小者優先)
    
---

## 2. Shared Data Structures

假設系統中有 $N$ 個 Processes。

```c
// 1. 意願陣列 (Boolean)
// 用於保護「取號碼」這個動作的原子性
// choosing[i] = true 表示 P_i 正處於「門口區段 (Doorway)」，正在計算號碼中
boolean choosing[N]=false; // Initialized to false

// 2. 號碼牌陣列 (Integer)
// number[i] = 0 表示 P_i 不想進入 Critical Section
// number[i] > 0 表示 P_i 的排隊號碼
int number[N]; // Initialized to 0
```

---

## 3. Algorithm Implementation

以下為 Process $i$ 的完整邏輯：

```c
// Process i 的結構
do {
	// 1. Doorway Section (取號區)
    choosing[i] = true;             // 鎖門：告訴大家「我要開始算號碼了」
    
    // 取號：查看目前所有人號碼的最大值，並 +1
    number[i] = find_max(number) + 1; 
    
    choosing[i] = false;            // 開門：號碼取好了，開放讀取
    
    // 2. Waiting Section (檢查區)
    for (j = 0; j < N; j++) {
        // [Step A]: 等待 Process j 取完號碼 (解決 Race Condition)
        while (choosing[j]) { /* busy wait */ }
        
        // [Step B]: 優先權比較
        // 條件：(P_j 有號碼) AND (P_j 的優先權 > 我的優先權)
        while (number[j] != 0 && 
              ((number[j] < number[i]) || (number[j] == number[i] && j < i)) );
             /* busy wait */
    }
    
    // 3. Critical Section
    // Access shared resources...
    
    // 4. Exit Section (離場)
    number[i] = 0; // 把號碼丟掉，表示我離開了
    
    // 5. Remainder Section

} while (true);
```

---

## 4. Correctness Proof

### A. Mutual Exclusion

**目標：** 證明不可能有兩個 Process ($P_i, P_j$) 同時在 Critical Section (CS) 中。

**證明邏輯 (Proof by Contradiction)：**

1. **假設衝突**：假設 $P_i$ 和 $P_j$ 同時進入了 CS。
    
2. **進入條件**：
    
    - $P_i$ 能進入 CS，表示它通過了對 $P_j$ 的檢查。這意味著：$(n_i, i) < (n_j, j)$。
        
    - $P_j$ 能進入 CS，表示它通過了對 $P_i$ 的檢查。這意味著：$(n_j, j) < (n_i, i)$。
        
3. **矛盾**：根據詞典順序的定義，不可能同時滿足 $A < B$ 且 $B < A$。
    
4. **結論**：假設不成立，故同一時間只有一個 Process 能在 CS 中。
    

> [!NOTE] 關鍵機制：choosing 旗標的作用
> 
> 為何需要 while (choosing[j])？
> 
> 如果沒有這個檢查，$P_i$ 可能會在 $P_j$ 正在寫入 number[j]（更新號碼）的過程中讀取它。此時 $P_i$ 可能讀到舊值 0，誤以為 $P_j$ 沒有排隊而直接進入 CS，但實際上 $P_j$ 剛剛算出了一個更小的號碼並進入 CS。
> 
> choosing 保證了可見性： 當 $P_i$ 進行比較時，它看到的 $P_j$ 號碼一定是穩定且有效的。

### B. Progress (No Deadlock)

**目標：** 證明若有人想進 CS，系統不會卡死（Deadlock），一定會有人進去。

**證明邏輯：**

1. **尋找最小值**：在任何時刻，所有想進入 CS 的 Process 集合中，根據詞典順序 `(number, ID)`，一定存在一個**全域最小值 (Global Minimum)**。
    
2. **贏家出線**：擁有這個最小值的 Process (設為 $P_{min}$)，在檢查其他所有 Process 時，發現自己的優先權最高 (自己 < 別人)。
    
3. **無阻礙**：因此，$P_{min}$ 不會被任何 `while` 迴圈卡住，它必然能順利進入 CS。
    
4. **結論**：系統永遠不會發生所有人都卡在 `while` 迴圈互相等待的情況。
    

### C. Bounded Waiting (有限等待 / Fairness)

**目標：** 證明不會有 Process 餓死 (Starvation)，且遵循先到先服務 (FCFS)。

**證明邏輯：**

1. **號碼固定**：一旦 $P_i$ 取得號碼 $n_i$，在它完成服務前，這個號碼不會變。
    
2. **後來者讓步**：任何比 $P_i$ 晚到的 $P_k$，取得的號碼 $n_k$ 必然 $\ge n_i + 1$。所以 $P_k$ 必須排在 $P_i$ 後面。
    
3. **等待有上限**：$P_i$ 只需要等待那些「號碼比它小」的 Process。因為 $P_i$ 的號碼是固定的，比它小的號碼數量是有限的 (最多 $N-1$ 個)。
    
4. **結論**：當前面有限的人做完後，$P_i$ 就會成為全域最小值並進入 CS。
    

---

## 5. Theoretical Foundation

### Safe Registers vs. Atomic Registers

Lamport 的證明指出，Bakery Algorithm 不需要強大的硬體支援。

- **Atomic Registers (原子暫存器)**：讀寫操作瞬間完成，不可分割。
    
- **Safe Registers (安全暫存器)**：這是最弱的條件。只要讀取操作與寫入操作**沒有重疊**，讀取就能拿到正確的值。若發生重疊（例如 $P_i$ 讀的時候 $P_j$ 正在寫），讀取到的值可以是任意值。
    
    - LBA 透過 `choosing` 變數巧妙地避開了讀寫重疊造成的錯誤判斷，證明了僅需 Safe Registers 即可運作。
        

---

## 6. Analysis & Limitations

儘管證明了正確性，但在現代電腦架構下，此演算法仍有實務上的困難。

|**缺點**|**說明**|
|---|---|
|**Unbounded Registers**|**(最大致命傷)** 號碼牌 `number` 只增不減。在長時間運行的系統中，數值會無限上升導致 **Integer Overflow**，進而破壞比較邏輯與互斥性。|
|**High Complexity**|時間複雜度為 $O(N)$。每個 Process 都要讀取其他所有 $N-1$ 個人的狀態，擴展性差。|
|**Busy Waiting**|使用 Spinlock (自旋鎖)，等待時持續佔用 CPU，浪費電力與資源。|
|**Cache Thrashing**|在多核心系統中，頻繁讀取共享變數 `choosing` 和 `number` 會導致大量的 Cache Coherence 流量，降低效能。|

---

## 7. Modern Variants

1. **Black-White Bakery Algorithm (BWB)**:
    
    - 解決了 **Unbounded Registers** 問題。引入「顏色位元 (Color bit)」與有限大小的暫存器，在保持 FIFO 公平性的同時防止溢位。
        
2. **Hardware Assistance**:
    
    - 現代系統多採用硬體指令 (如 `CAS`, `TAS`) 實作 Mutex，提供 $O(1)$ 的效率並支援 Sleep/Wakeup 機制。