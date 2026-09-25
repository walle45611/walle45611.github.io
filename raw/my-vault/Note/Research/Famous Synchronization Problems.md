[]d
## 1. Bounded-Buffer Problem (Producer-Consumer)

此問題描述生產者 (Producer) 將資料放入有限大小的緩衝區，消費者 (Consumer) 從中取出資料。必須確保：

1. **Mutual Exclusion**: 存取 Buffer 時互斥。
    
2. **Synchronization**: Buffer 滿時 Producer 等待，Buffer 空時 Consumer 等待。
    

### Shared Variables


```
Semaphore mutex = 1; // 保護 Buffer 操作的互斥鎖
Semaphore full = 0;  // 計算 Buffer 中已佔用的數量 (初始為 0)
Semaphore empty = N; // 計算 Buffer 中剩餘空位的數量 (初始為 N)
```

### Producer Process


```
do {
    // produce an item in next_produced
    
    wait(empty); // 1. 確認有空位 (若 empty <= 0 則等待)
    wait(mutex); // 2. 取得互斥鎖
    
    // add the item to the buffer
    
    signal(mutex); // 3. 釋放互斥鎖
    signal(full);  // 4. 增加已佔用數量 (通知 Consumer)
    
} while (TRUE);
```

### Consumer Process

```
do {
    wait(full);  // 1. 確認有資料 (若 full <= 0 則等待)
    wait(mutex); // 2. 取得互斥鎖
    
    // remove an item from buffer to next_consumed
    
    signal(mutex); // 3. 釋放互斥鎖
    signal(empty); // 4. 增加空位數量 (通知 Producer)
    
    // consume the item in next_consumed
    
} while (TRUE);
```

---
## 2. Readers-Writers Problem

此問題允許多個 **Readers** 同時讀取，但在同一時間只允許一個 **Writer** 寫入。根據優先權的不同，分為兩種。

### A. First Readers-Writers Problem (Reader Priority)

**定義：** 除非 Writer 已經取得權限，否則讀者不應讓其他讀者等待。

- **特性：** 只要有一個 Reader 在讀，後續的 Readers 都可以直接進入，無需等待 Writer。
    
- **缺點：** 可能導致 **Writer Starvation** (若讀者源源不絕，寫者永遠搶不到鎖)。
    

#### Shared Variables

```
Semaphore mutex = 1;     // 保護 readcount 的互斥鎖
Semaphore wrt = 1;       // 用於 Writer 和 Reader 之間的互斥 (寫入鎖)
Integer readcount = 0;   // 記錄目前有多少 Reader 正在讀取
```

#### Writer Process

```
do {
    wait(wrt); // 請求寫入權限 (若有 Reader 在讀，會卡住)
    
    // writing is performed
    
    signal(wrt); // 釋放寫入權限
} while (TRUE);
```

#### Reader Process

```
do {
    wait(mutex);       // 1. 保護 readcount
    readcount++;       // Reader 增加
    if (readcount == 1)
        wait(wrt);     // [關鍵]: 第一個 Reader 負責搶 wrt 鎖 (擋住 Writer)
    signal(mutex);

    // reading is performed

    wait(mutex);       // 2. 保護 readcount
    readcount--;       // Reader 離開
    if (readcount == 0)
        signal(wrt);   // [關鍵]: 最後一個 Reader 負責釋放 wrt 鎖 (允許 Writer 進入)
    signal(mutex);
    
} while (TRUE);
```

---

### B. Second Readers-Writers Problem (Writer Priority)

**定義：** 一旦 Writer 準備好寫入，它應該儘快執行。

- **特性：** 當 Writer 在等待時，任何**新**到達的 Reader 都必須被阻擋，不能插隊。
    
- **缺點：** 可能導致 **Reader Starvation** (若寫者源源不絕，讀者可能長時間無法讀取)。
    

#### Shared Variables

為了實現寫者優先，需要額外的計數器與號誌來管理排隊順序。

```
int readcount = 0, writecount = 0; 
Semaphore rwmutex = 1; // [資源鎖] 真正的檔案/資源互斥鎖
Semaphore rsem = 1;    // [順序鎖] 用來阻擋 Reader (當 Writer 存在時)
Semaphore x = 1;       // 保護 readcount
Semaphore y = 1;       // 保護 writecount
Semaphore z = 1;       // [排隊鎖] 防止 Reader 在 Writer 等待時持續進入 rsem 隊列
```

#### Writer Process

Writer 的邏輯是：只要有 Writer 出現 (`writecount > 0`)，就鎖住 `rsem` 不讓 Reader 進來。

```
while (true) {
    // 1. 增加 writecount
    wait(y);
    writecount++;
    if (writecount == 1) 
        wait(rsem);    // [關鍵]: 第一個 Writer 負責鎖住 rsem，阻擋後續 Reader
    signal(y);

    // 2. 執行寫入
    wait(rwmutex);     // 搶資源鎖 (可能需等待當前的 Reader 讀完)
    // perform writing
    signal(rwmutex);

    // 3. 減少 writecount
    wait(y);
    writecount--;
    if (writecount == 0) 
        signal(rsem);  // [關鍵]: 最後一個 Writer 離開，才開放 Reader 進入
    signal(y);
}
```

#### Reader Process

Reader 必須檢查是否有 Writer 在排隊 (`wait(z)` 和 `wait(rsem)`)。

```
while (true) {
    // 1. 進場檢查 (Entry Section)
    wait(z);           // 避免多個 Readers 在 Writer 等待時擠在 rsem 上
    wait(rsem);        // 若有 Writer (writecount > 0)，Reader 會卡在這裡
    wait(x);           // 保護 readcount
    
    readcount++;
    if (readcount == 1) 
        wait(rwmutex); // 第一個 Reader 負責搶資源鎖
    
    signal(x);
    signal(rsem);      // 釋放 rsem 讓下一個 Reader 或 Writer 有機會
    signal(z);

    // 2. 讀取 (Reading Section)
    // reading is performed

    // 3. 離場 (Exit Section)
    wait(x);
    readcount--;
    if (readcount == 0) 
        signal(rwmutex); // 最後一個 Reader 釋放資源鎖
    signal(x);
}
```

### C. Comparison Summary

|**Feature**|**First R-W Problem**|**Second R-W Problem**|
|---|---|---|
|**Priority**|**Reader Priority** (讀者優先)|**Writer Priority** (寫者優先)|
|**Concept**|只要有讀者在讀，新讀者可直接加入|一旦寫者在等，新讀者必須等待|
|**Starvation**|**Writer Starvation**|**Reader Starvation**|
|**Complexity**|簡單 (需 `mutex`, `wrt`)|複雜 (需 5 個 Semaphores)|
|**Throughput**|高 (Reader 可高度並發)|較低 (頻繁切換)|

---

## 3. Dining-Philosophers Problem

五位哲學家圍繞圓桌，每人之間有一根筷子 (共 5 根)。哲學家必須同時取得左右兩邊的筷子才能進食。
### A. Semaphore Solution

這是最直觀的解法，但存在風險。

**Shared Variables:**

```
Semaphore chopstick[5]; // 初始值皆為 1
```

**Philosopher $i$ Process:**

```
while (true) {
    wait(chopstick[i]);             // 1. 拿左邊筷子
    wait(chopstick[(i + 1) % 5]);   // 2. 拿右邊筷子

    // eat

    signal(chopstick[i]);           // 3. 放下左邊筷子
    signal(chopstick[(i + 1) % 5]); // 4. 放下右邊筷子

    // think
}
```

> [!WARNING] Deadlock Risk
> 
> 若所有哲學家同時拿起左邊的筷子 (wait(chopstick[i]) 成功)，則所有人都在等待右邊的筷子，會造成 Deadlock。

### B. Monitor Solution (Deadlock-free)

利用高階同步工具 Monitor 來解決死結問題。只有當左右鄰居都沒有在進食時，哲學家才進入 EATING 狀態。

**Monitor Structure:**

```
monitor DP {
    enum { THINKING, HUNGRY, EATING } state[5];
    condition self[5]; // 用於延遲哲學家 (當筷子不可用時)

    // Initialization
    void structural_init() {
        for (int i = 0; i < 5; i++) 
            state[i] = THINKING;
    }

    // Attempt to pickup chopsticks
    void pickup(int i) {
        state[i] = HUNGRY;
        test(i); // 嘗試進食
        if (state[i] != EATING)
            self[i].wait(); // 若無法進食，則等待
    }

    // Put down chopsticks
    void putdown(int i) {
        state[i] = THINKING;
        // 檢查左右鄰居是否因為我在吃而卡住，若是則喚醒他們
        test((i + 4) % 5); // Check Left Neighbor
        test((i + 1) % 5); // Check Right Neighbor
    }

    // Test function (Check logic)
    void test(int i) {
        if ((state[(i + 4) % 5] != EATING) && // 左鄰居沒在吃
            (state[i] == HUNGRY) &&           // 自己想吃
            (state[(i + 1) % 5] != EATING)) { // 右鄰居沒在吃
            
            state[i] = EATING;
            self[i].signal(); // 喚醒自己 (若之前在 wait) 或無操作
        }
    }
}
```

---

## 4. The Sleeping Barber Problem

理髮師在沒有顧客時睡覺；顧客到達時喚醒理髮師。若理髮師在忙且有空位，顧客等待；若無空位，顧客離開。

### Shared Variables

```
Semaphore Customers = 0; // 等待理髮的顧客數 (用來喚醒理髮師)
Semaphore Barber = 0;    // 理髮師的狀態 (用來讓顧客等待理髮完成)
Semaphore mutex = 1;     // 保護 waiting 變數
int waiting = 0;         // 等待室中實際坐著的顧客數量
int N = 5;               // 等待室的椅子總數
```

### Barber Process

```
while (true) {
    wait(Customers);    // 1. 睡覺，直到有顧客喚醒 (Customers > 0)
    
    wait(mutex);        // 2. 修改 waiting 變數
    waiting--;          // 顧客離開等待室去理髮
    signal(mutex);
    
    signal(Barber);     // 3. 理髮師準備好了 (喚醒一位卡在 wait(Barber) 的顧客)
    
    /* barber is cutting hair */
}
```

### Customer Process

```
while (true) {
    wait(mutex);        // 1. 進入店內，取得互斥鎖檢查座位
    
    if (waiting < N) {  // 還有空位
        waiting++;      // 坐下
        
        signal(Customers); // 2. 喚醒理髮師 (或是增加排隊人數)
        signal(mutex);     // 釋放互斥鎖 (讓其他人可以進來檢查)
        
        wait(Barber);      // 3. 坐在椅子上發呆，直到理髮師叫我 (Barber Signal)
        
        // get haircut
        
    } else {
        signal(mutex);  // 沒位子，釋放鎖
        // leave shop
    }
}
```