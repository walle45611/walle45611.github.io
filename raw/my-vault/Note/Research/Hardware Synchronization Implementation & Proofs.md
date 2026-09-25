## 1. Memory Barriers

### Proof of Failure by Reordering

若缺乏 Memory Barriers，Processor 可能對**無 Data Dependency** 的指令進行 Reordering (指令重排)。

**Scenario Assumption:**

- **Initial State:** `flag = false`, `data = 0`
    
- **Thread 1 (Producer):** 寫入資料，設定旗標。
    
- **Thread 2 (Consumer):** 等待旗標，讀取資料。
    

**Sequential Consistency (預期邏輯):**

```
// Thread 1          // Thread 2
data = 100;          while (!flag);
flag = true;         print(data); // Expected: 100
```

Trace (Failure Case):

由於 data 和 flag 之間無 Data Dependency，Compiler 或 CPU 將 Thread 1 進行 Reordering：

1. **T1:** `flag = true;` (Store Buffer 提交，對 T2 可見)
    
2. **T2:** `while (!flag)` 條件通過 (讀到 true)。
    
3. **T2:** `print(data);` **[Error]** 此時 `data` 尚未寫入，印出 `0` (Stale Value)。
    
4. **T1:** `data = 100;` (現在才執行寫入)
    

### Correct Implementation

```
// Thread 1
data = 100;
__sync_synchronize(); // Memory Barrier: 強制 data 寫入先於 flag 寫入
flag = true;

// Thread 2
while (!flag);
__sync_synchronize(); // Memory Barrier: 確保 flag 讀取後才讀取 data
print(data);
```

---

## 2. Test-and-Set (TAS)

### Instruction Definition

```
boolean test_and_set(boolean *target) {
    boolean rv = *target;
    *target = true;
    return rv; // Executed Atomically
}
```

### A. Simple TAS Mutex ALGO1

**Code:**

```
do {
    while (test_and_set(&lock)); // Busy Waiting
        
    /* Critical Section */
    
    lock = false;
    
    /* Remainder Section */
} while (true);
```

**Proof of Correctness:**

> [!NOTE] Mutual Exclusion
> 
> [Satisfied]
> 
> 若 lock 為 FALSE，test_and_set 會 Atomically 將其設為 TRUE 並回傳 FALSE，只有這一個 Process 能跳出迴圈。其他 Process 會讀到 TRUE 而持續 Busy Waiting。

> [!NOTE] Bounded Waiting
> 
> [Not Satisfied]
> 
> 當 lock 被重置為 FALSE 時，如果有多個 Processes 在 while 迴圈中競爭，Hardware 並不保證公平性。運氣好的 Process 可能連續贏得 TAS，導致其他 Process 發生 Starvation。

---

### B. Bounded-waiting TAS ALGO2

利用 `waiting[]` array 配合 `test_and_set` 實現公平性。

**Code:**

```
// Shared: boolean lock = false;  boolean waiting[n] = {false};
do {
    waiting[i] = true;
    key = true;
    
    // 等待直到 lock 釋放 且 輪到我 (waiting[i] == true)
    while (waiting[i] && key)
        key = test_and_set(&lock); 
    
    waiting[i] = false; // 取得進入權 (Acquired lock or permission)
    
    /* Critical Section */
    
    // 尋找下一個等待者 (Round-Robin)
    j = (i + 1) % n;
    while ((j != i) && !waiting[j])
        j = (j + 1) % n;
    
    if (j == i)
        lock = false;       // 無人等待，釋放 lock
    else
        waiting[j] = false; // 指定 Pj 進入 (Handoff permission)
        
    /* Remainder Section */
} while (true);
```

**Proof of Correctness:**

> [!NOTE] Mutual Exclusion
> 
> [Satisfied]
> 
> Process $P_i$ 進入 Critical Section 只有兩條路徑：
> 
> 1. `test_and_set` 回傳 `false` (直接搶到 lock)。
>     
> 2. waiting[i] 被設為 false (由前一個離開者指定)。
>     
>     兩者都保證同一時間只有一人進入。
>     

> [!NOTE] Progress
> 
> [Satisfied]
> 
> 離開 Critical Section 的 Process 會掃描 waiting[]。若有人在等，必定會指定一人進入；若無人在等，則釋放 lock 讓後續 Process 競爭。不會發生 Deadlock。

> [!NOTE] Bounded Waiting
> 
> [Satisfied]
> 
> 離開 Critical Section 的 Process 依序 ($i+1, i+2...$) 掃描陣列。任何等待中的 Process $P_j$ 最多只需要等待 $n-1$ 次讓位操作，就能被輪到 (設定 waiting[j] = false)。

---

## 3. Compare-and-Swap (CAS)

### Instruction Definition

```
int compare_and_swap(int *value, int expected, int new_value) {
    int temp = *value;
    if (*value == expected)
        *value = new_value;
    return temp; // Executed Atomically
}
```

### A. CAS Mutex (Simple)

邏輯同 Simple TAS，僅指令不同。

```
// lock 0: unlocked, 1: locked
while (compare_and_swap(&lock, 0, 1) != 0); // Busy Waiting
/* Critical Section */
lock = 0;
```

**Proof:**

- **Mutual Exclusion:** `[Satisfied]`
    
- **Bounded Waiting:** `[Not Satisfied]` (存在 Starvation 風險)
    

### B. Bounded-waiting CAS ALGO2

此演算法利用 `waiting[]` 配合 `compare_and_swap` 實現公平性。

**Code:**

```
// Shared: int lock = 0; boolean waiting[n] = {false};
// Local: int key;
do {
    waiting[i] = true;
    key = 1;

    // 嘗試取得 lock。
    // 若 CAS 成功 (lock 0->1)，回傳 0 -> key=0 -> 跳出迴圈
    // 若 CAS 失敗 (lock 原本是 1)，回傳 1 -> key=1 -> 繼續等待
    while (waiting[i] && key == 1)
        key = compare_and_swap(&lock, 0, 1);

    waiting[i] = false; // 取得進入權

    /* Critical Section */

    // 尋找下一個等待者 (Round-Robin)
    j = (i + 1) % n;
    while ((j != i) && !waiting[j])
        j = (j + 1) % n;

    if (j == i)
        lock = 0;           // 無人等待，釋放 lock
    else
        waiting[j] = false; // 指定 Pj 進入 (Handoff permission)

    /* Remainder Section */
} while (true);
```

**Proof:**

> [!NOTE] Mutual Exclusion
> 
> [Satisfied]
> 
> Process $P_i$ 進入 Critical Section 只有兩條路徑：
> 
> 1. `compare_and_swap` 回傳 `0` (成功將 lock 從 0 改為 1)。
>     
> 2. `waiting[i]` 被設為 `false` (由前一個離開者指定)。
>     

> [!NOTE] Bounded Waiting
> 
> [Satisfied]
> 
> 透過 waiting[] 陣列與 Round-Robin 掃描機制，保證任何等待中的 Process 最多等待 $n-1$ 次即可進入。

### C. Atomic Integer Implementation

**Code (Atomic Increment):**

```
void atomic_increment(int *v) {
    int temp;
    do {
        temp = *v; // Snapshot
        // 若 *v 仍等於 temp 則更新；否則重試 (Retry)
    } while (temp != compare_and_swap(v, temp, temp + 1));
}
```

### D. Proof of Limitation (Atomic Variables)

Atomic Variables 無法解決 **Compound Operations** (複合操作) 的 Race Condition。

**Scenario: Bounded Buffer**

C

```
// 錯誤實作 (Flawed Implementation)
if (atomic_count < BUFFER_SIZE) { // Line A: Check
    buffer[atomic_count] = item;  // Line B: Write
    atomic_increment(&atomic_count); // Line C: Update
}
```

**Trace (Failure Case):**

1. **P1** 執行 Line A: `count` 為 4 (Size 為 5)，檢查通過。
    
2. (Context Switch)
    
3. **P2** 執行 Line A: `count` 仍為 4，檢查通過。
    
4. **P2** 執行 Line B: 寫入 `buffer[4]` 並 Increment，`count` 變為 5。
    
5. (Context Switch 回到 P1)
    
6. **P1** 執行 Line B: 寫入 `buffer[5]` -> **[Error]** 發生 Buffer Overflow 或 Index Out of Bounds。
    

> [!NOTE] Conclusion
> 
> 雖然 count 本身是 Atomic Variable，但 "Check Value" 與 "Write Buffer" 這兩個步驟之間缺乏 Atomicity。此類邏輯仍需使用 Lock 或 Semaphore 來保護整個 Block。