## 1. Hardware-based Mutex Implementation

現代 Mutex 通常依賴 **Atomic Hardware Instructions** 來保證操作的不可分割性。

### A. Test-and-Set (TAS)

#### Instruction Definition

```
boolean test_and_set(boolean *target) {
    boolean rv = *target;  // 1. Store original value
    *target = TRUE;        // 2. Set new value to TRUE
    return rv;             // Executed Atomically
}
```

#### Application: Simple Spinlock (Algorithm 1)

這是最基礎的 Mutex 實作，採用 **Busy Waiting**。

```
// Shared: boolean lock = FALSE;
while (TRUE) {
    // Acquire Lock
    while (test_and_set(&lock))
        ; /* busy wait */

    /* Critical Section */

    // Release Lock
    lock = FALSE; 

    /* Remainder Section */
}
```

#### Proof of Correctness

> [!NOTE] Mutual Exclusion
> 
> [Satisfied]
> 
> test_and_set 的原子性保證只有第一個讀到 FALSE 並將其設為 TRUE 的 Process 能進入。

> [!NOTE] Bounded Waiting
> 
> [Not Satisfied]
> 
> 硬體不保證等待佇列的順序，多個 Process 競爭時可能導致 Starvation。

---

### B. Compare-and-Swap (CAS)

#### Instruction Definition

```
int compare_and_swap(int *value, int expected, int new_value) {
    int temp = *value;
    if (*value == expected)
        *value = new_value;
    return temp; // Executed Atomically
}
```

#### Application: Simple Spinlock

邏輯與 TAS 相同，僅指令不同。


```
// Shared: int lock = 0;
while (TRUE) {
    // Attempt to change lock from 0 to 1
    while (compare_and_swap(&lock, 0, 1) != 0)
        ; /* busy wait */

    /* Critical Section */

    lock = 0; 
    /* Remainder Section */
}
```

#### Proof of Correctness

> [!NOTE] Analysis
> 
> 與簡單 TAS 鎖相同：滿足 Mutual Exclusion 與 Progress，但不滿足 Bounded Waiting。

---

### C. Bounded-waiting Mutex (Algorithm 2)

為了修正上述硬體鎖缺乏 **Bounded Waiting** 的問題，引入 `waiting[]` 陣列來建立排隊機制。

#### Code Implementation

```
// Shared: boolean lock = FALSE; boolean waiting[n] = {FALSE};
do {
    waiting[i] = true; // Declare intent
    key = true;
    
    // Wait until lock is free AND it's my turn
    while (waiting[i] && key)
        key = test_and_set(&lock);

    waiting[i] = FALSE; // Acquire lock
    
    /* Critical Section */

    // Find next waiting process (Round-Robin)
    j = (i + 1) % n;
    while ((j != i) && !waiting[j])
        j = (j + 1) % n;

    if (j == i)
        lock = FALSE;       // No one waiting, release lock
    else
        waiting[j] = FALSE; // Hand over permission to Pj (No lock release needed)
        
    /* Remainder Section */
} while (true);
```

#### Proof of Correctness

> [!SUCCESS] Bounded Waiting
> 
> [Satisfied]
> 
> 離開 CS 的 Process 會依序掃描 waiting 陣列，明確指定下一個進入者 (Hand-off)，確保 FIFO 性質，消除 Starvation。

---

## 2. Mutex Locks (High-Level Tool)

**Mutex** (Mutual Exclusion) 是 OS 提供給開發者最簡單的同步工具。

### 定義與特性

- **State:** 包含一個 Boolean 變數 `available`。
    
- **Atomic Operations:**
    
    - `acquire()`: 取得鎖，若不可用則等待。
        
    - `release()`: 釋放鎖。
        
- **Spinlock:** 由於通常使用 Busy-waiting 實作，故常被稱為 Spinlock。適用於 **Short Duration** 的 CS，因為可以避免 Context Switch 的開銷。
    
### Code Structure

```
acquire() {
    while (!available)
        ; /* busy wait */
    available = false;
}

release() {
    available = true;
}

// Usage
do {
    acquire();
    /* Critical Section */
    release();
    /* Remainder Section */
} while (true);
```

---
## 3. Semaphores (Robust Tool)

**Semaphore** 是一種比 Mutex 更健壯的同步工具，可用於互斥或資源計數。

### A. Operations

必須透過兩個原子操作存取：

1. **wait(S)** (亦稱 `P()`): 若 $S \le 0$ 則等待，否則 $S$ 減 1。
    
2. **signal(S)** (亦稱 `V()`): $S$ 加 1。
    
### B. Implementation Strategies

#### 1. Busy-waiting (Binary Semaphore) Implementation

簡單但浪費 CPU (Spinlock)。

```
wait(S) {
    while (S <= 0); // busy wait
    S--;
}

signal(S) {
    S++;
}
```

#### 2. Counting Semaphore Implementation

利用 Binary Semaphores 來模擬 Counting Semaphore。

- **概念**：統計有多少 Process 在 wait 當中。如果 $-N$ 代表有 $N$ 個 processes 卡在 wait 中。
    
- **Global Variables:**
    
    - `int C`: 用於記錄資源數量的 Counter (Initial value = initial count)。
        
    - `Binary_semaphore S1 = 1`: 用於保護 `C` 的 **Mutex** (Mutual Exclusion)。
        
    - `Binary_semaphore S2 = 0`: 用於 **Blocking** (當資源不足時卡住 Process)。
        

**Code Implementation:**

```
// Wait Operation (P)
Wait(c) {
    Wait(S1);             // Protect C
    C--;
    if (C < 0) {
        Signal(S1);       // [Critical]: Must release mutex before blocking!
        Wait(S2);         // Block self in S2 queue
    } else {
        Signal(S1);       // Release mutex
    }
}

// Signal Operation (V)
Signal(c) {
    Wait(S1);             // Protect C
    C++;
    if (C <= 0) {
        Signal(S2);       // Wake up one process from S2 queue
    }
    Signal(S1);           // Release mutex
}
```

> [!NOTE] Key Concept
> 
> - **S1 (Mutex):** 確保一次只有一個 Process 能修改整數 `C`。
>     
> - **S2 (Delay Queue):** 當 `C < 0` 時，Process 必須在 `S2` 上等待 (`Wait(S2)`)。注意在進入 `Wait(S2)` 睡眠之前，**必須先釋放 S1 (`Signal(S1)`)**，否則會造成 Deadlock（因為持有鎖睡覺，其他人無法進入 Signal 操作來喚醒你）。
>     

#### 3. Non-busy Waiting Implementation (Block/Wakeup)

為了避免長時間 Spinning 浪費 CPU，標準 OS 實作會將等待的 Process 放入 Waiting Queue。

- **Structure:** 每個 Semaphore 包含一個 `value` 和一個 `list` (PCB List)。
    
- **Wait with Block:**
    
    ```
    wait(S) {
        S->value--;
        if (S->value < 0) {
            add this process to S->list;
            block(); // Suspend process, distinct from busy wait
        }
    }
    ```
    
- **Signal with Wakeup:**
    
    ```
    signal(S) {
        S->value++;
        if (S->value <= 0) { // Means someone is waiting
            remove a process P from S->list;
            wakeup(P); // Resume process
        }
    }
    ```
    
####  4. Semaphore Implementation Strategies (Construction)

根據 Critical Section 的保護機制與等待策略，Semaphore 的實作可分為四種方法。

**分類:**

|**Protection Mechanism \ Waiting Strategy**|**Non-busy waiting (Block/Wakeup)**|**Busy-waiting (Spinlock)**|
|---|---|---|
|**Disable Interrupt**|**[Algorithm 1]** (適合 Uniprocessor)|**[Algorithm 3]**|
|**HW Instructions (TAS/CAS)**|**[Algorithm 2]** (適合 Multiprocessor)|**[Algorithm 4]**|

##### A. Non-busy Waiting Semaphores (Block/Wakeup)

此類實作旨在避免 CPU 空轉。當資源不足時，Process 會將自己加入 Waiting Queue 並呼叫 `sleep()`。

###### Algorithm 1: Disable Interrupt

利用「關閉中斷」來確保修改 Semaphore 數值與 List 操作的原子性。

- **Applicability:** 僅適用於 **Uniprocessor** 系統。在 Multiprocessor 系統中，關閉一個 CPU 的中斷無法阻止其他 CPU 存取共享變數。

```
// Wait(S)
wait(S) {
    Disable_Interrupt();  // [Critical]: Start Atomicity
    S->value--;
    if (S->value < 0) {
        add process P to S->list;
        Enable_Interrupt(); // Must enable before sleeping
        sleep();
    } else {
        Enable_Interrupt();
    }
}

// Signal(S)
signal(S) {
    Disable_Interrupt(); // [Critical]: Start Atomicity
    S->value++;
    if (S->value <= 0) {
        remove P from S->list;
        wakeup(P);
    }
    Enable_Interrupt();
}
```

###### Algorithm 2: Hardware Instructions (TAS/CAS)

利用硬體指令 (如 `test_and_set` 或 `compare_and_swap`) 實作一個 Spinlock，用來保護 Semaphore 內部的 `value` 和 `list`。

- **Applicability:** 適用於 **Multiprocessor** 系統。
    
- **Note:** 雖然 Semaphore 本身是 Non-busy waiting (Process 會 sleep)，但在進入 Critical Section 修改 `S->value` 的極短時間內，使用的是 Busy-waiting (Spinlock) 來保護數據結構。
    
```
// Concept using Spinlock (implemented by TAS/CAS)
wait(S) {
    // 1. Entry Section (Busy-waiting here for protection)
    acquire_spinlock(&S->lock); 

    // 2. Critical Section (Semaphore Logic)
    S->value--;
    if (S->value < 0) {
        add self to S->list;
        release_spinlock(&S->lock); // Release lock before sleep
        sleep();
    } else {
        release_spinlock(&S->lock);
    }
}

signal(S) {
    acquire_spinlock(&S->lock);
    S->value++;
    if (S->value <= 0) {
        remove P from S->list;
        wakeup(P);
    }
    release_spinlock(&S->lock);
}
```

> [!NOTE] Observation
> 
> 即便是在 Non-busy waiting 的設計中，Entry Code (取得內部鎖的過程) 仍含有 Busy-waiting。這是為了保護 Semaphore 結構本身的完整性。

##### B. Busy Waiting Semaphores (Spinlocks)

此類實作中，當資源不足時，Process 會在迴圈中持續檢查，直到資源可用。

###### Algorithm 3: Disable Interrupt

利用關閉中斷來保護「檢查與修改」的過程。

```
wait(S) {
    Disable_Interrupt();
    while (S <= 0) {
        Enable_Interrupt(); // 允許中斷，避免系統死鎖
        // delay or NOP
        Disable_Interrupt(); // 再次關閉以檢查條件
    }
    S--;
    Enable_Interrupt();
}

signal(S) {
    Disable_Interrupt();
    S++;
    Enable_Interrupt();
}
```

###### Algorithm 4: Hardware Instructions (TAS/CAS)

利用 TAS/CAS 保護檢查邏輯。這是最標準的 Spinlock 實作方式。

- **Transformation:** 將 Algorithm 3 中的 `Disable/Enable Interrupt` 替換為 `TAS/CAS` 的 Entry/Exit Section。
    
```
// Wait Operation (P)
Wait(S) {
    while (true) {
        acquire_lock(); // 1. 取得鎖 (保護 S)
        if (S > 0) {
            S--;
            release_lock(); // 2a. 成功扣除，釋放鎖
            break;          // 離開迴圈 (Enter CS)
        }
        release_lock(); // 2b. [關鍵]: S 不足，必須釋放鎖！
                        // 否則 Signal 無法取得鎖來增加 S，會造成 Deadlock。
    }
}

// Signal Operation (V)
Signal(S) {
    acquire_lock(); // 1. 取得鎖 (保護 S)
    S++;
    release_lock(); // 2. 釋放鎖
}
```
##### Comparison Summary

|**Method**|**Protection**|**Wait Strategy**|**Pros**|**Cons**|
|---|---|---|---|---|
|**Algo 1**|Disable Int|Sleep|簡單|僅限單核心 (Uniprocessor)|
|**Algo 2**|HW Instr.|Sleep|適用多核心，節省 CPU|實作複雜，Entry section 仍有極短暫 busy-wait|
|**Algo 3**|Disable Int|Spin|-|僅限單核心，浪費 CPU|
|**Algo 4**|HW Instr.|Spin|適用多核心，Context Switch 開銷低|浪費 CPU (若等待時間長)|