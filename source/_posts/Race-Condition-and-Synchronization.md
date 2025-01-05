---
title: Race Condition and Synchronization in Process Management
date: 2025-01-04
categories: [Operating System]
tags: [Concurrency, Critical Section, Priority Inversion]
---

![圖 1. : Blog Image](https://imgur.com/UkVANiR.png)

<!--more-->

- Where several processes access and manipulate the same data concurrently, and the outcome of the execution depends on the particular order in which the access takes place, is called a race condition.
  - 當多個 process 同時訪問和操作相同的數據，而執行結果取決於訪問操作的特定順序時，這種情況稱為競爭條件（race condition）。
- 重點單字：`process synchronization` 和 `coordination`

## The Critical-Section Problem (臨界區問題)

![圖 2. : Critical-Section](https://imgur.com/DMJmmyv.png)

### 解決該問題的三個主要條件

- **Mutual Exclusion**
  - 保證同一時間只有一個 process 能進入 `critical section`，避免資料不一致。
- **Progress**
  - 確保後選 process 能夠進入 `critical section`。
  - 換句話說，如果沒有 process 在執行 `critical section`，而有些 process 想進入，只有那些不在 `remainder section` 的 process 可以參與決定下一個進入的 process，且不能有飢餓狀態（無限等待）。
- **Bounded Waiting**
  - 確保每個 process 不會被無限期地等待，系統保證在有限次數的輪候後獲得進入權限，避免某個 process 被「餓死」，確保公平性。

### Preemptive Kernels and Nonpreemptive Kernels

- Preemptive 排程演算法會導致 race condition 的風險。
- Nonpreemptive 排程因為 process 不會被中斷，所以不會有 race condition。

### Peterson’s Solution

![圖 3. : Peterson’s Solution](https://imgur.com/vtaZse1.png)

- 基於軟體的解決方案。
- 現代 compiler 的重新排序可能導致此方法失效。
- 適用於兩個 process。
- 基於硬體假設的 `atomic operations` 和共享內存。

#### 流程

1. 將 `id` 傳入，並標記自己想進入臨界區。
2. 設定 `turn = other`，讓另一個 process 有機會。
3. 執行臨界區並退出。

```cpp
#include <bits/stdc++.h>
using namespace std;

atomic<bool> flag[2] = {false, false};
atomic<int> turn = 0;

void critical_section(int id) {
  int other = 1 - id;
  flag[id] = true;
  turn = other;
  while (flag[other] && turn == other);

  // 臨界區段
  cout << "Thread " << id << " is in the critical section." << endl;
  this_thread::sleep_for(chrono::milliseconds(100));

  flag[id] = false;
}

int main() {
  thread t1(critical_section, 0);
  thread t2(critical_section, 1);
  t1.join();
  t2.join();
  return 0;
}
```

---

### Hardware Instructions

#### 核心概念：原子性操作（Atomic Operation）

- **Test-and-Set**
  - 測試某變數值並將其設為特定值。
- **Compare-and-Swap**
  - 比較某變數的當前值與指定值，相等則替換，否則不執行。

### Mutex Lock（互斥鎖）

![圖 4. : Mutex Lock 1](https://imgur.com/oXER5vp.png)

![圖 5. : Mutex Lock 2](https://imgur.com/ARKsZva.png)

- 其實就是先上鎖在解鎖就這樣很簡單
- available = true 可用，反之

```cpp
#include <bits/stdc++.h>
using namespace std;

mutex mtx;

void critical_section(int id) {
  for (int i = 0; i < 5; ++i) {
    mtx.lock();
    cout << "Thread " << id << " is in critical section." << endl;
    mtx.unlock();
    this_thread::sleep_for(chrono::milliseconds(50));
  }
}

int main() {
  vector<thread> threads;
  for (int i = 0; i < 3; ++i) {
    threads.push_back(thread(critical_section, i));
  }
  for (auto &th : threads) {
    th.join();
  }
  return 0;
}
```

---

### Spinlock（自旋鎖）

```cpp
#include <bits/stdc++.h>
using namespace std;

class Spinlock {
private:
  atomic_flag lock_flag = ATOMIC_FLAG_INIT;

public:
  void lock() {
    while (lock_flag.test_and_set(memory_order_acquire));
  }
  void unlock() {
    lock_flag.clear(memory_order_release);
  }
};

int main() {
  Spinlock spinlock;
  // 臨界區邏輯同樣適用
  return 0;
}
```

---

### Semaphore

- Counting semaphore 可在無限制範圍內變化。
- Binary semaphore 數值僅能在 `0` 和 `1` 之間，模擬類似 mutex locks。
- 是一個整數變數，除了初始化，只能透過兩個 atomic operations 操作，wait() 和 signal() 操作
- wait() 稱為 P 因為來自荷蘭語 proberen 測試
- signal() 稱為 V 也是因為荷蘭語 verhogen 增加

![圖 6. : Semaphore wait](https://imgur.com/RuxzPo1.png)
![圖 7. : Semaphore signal](https://imgur.com/E56z8Kf.png)

```cpp
#include <semaphore>
#include <bits/stdc++.h>
using namespace std;

binary_semaphore sem(1);

void critical_section(int id) {
  sem.acquire();
  cout << "Thread " << id << " is in critical section." << endl;
  this_thread::sleep_for(chrono::milliseconds(100));
  sem.release();
}

int main() {
  thread t1(critical_section, 1);
  thread t2(critical_section, 2);
  t1.join();
  t2.join();
  return 0;
}
```

---

| **特性**           | **Spinlock**                                     | **Mutex Lock**                                  |
| ------------------ | ------------------------------------------------ | ----------------------------------------------- |
| **等待行為**       | 忙等待（Busy Waiting），不斷輪詢鎖的狀態。       | 阻塞等待（Blocking Wait），進入睡眠直到被喚醒。 |
| **性能**           | 適合短時間臨界區段，避免上下文切換的開銷。       | 適合長時間臨界區段，避免浪費 CPU 資源。         |
| **使用場景**       | 多用於內核或高性能場景，臨界區段時間極短時。     | 多用於應用層或臨界區段時間較長的場景。          |
| **資源消耗**       | 高（因為需要輪詢，占用 CPU）。                   | 低（CPU 可用於其他任務）。                      |
| **上下文切換開銷** | 無上下文切換開銷（因為不進入睡眠）。             | 有上下文切換開銷（因為進入睡眠後需被喚醒）。    |
| **公平性**         | 可能導致飢餓問題（某些執行緒可能一直輪詢不到）。 | 相對公平（內部可能採用排隊機制）。              |

---

## Monitors

- 高階同步機制，封裝共享變量及其操作，避免信號量與鎖的錯誤使用。
- 包含共享變量、函數和條件變量。

```cpp
#include <iostream>
#include <queue>
#include <thread>
#include <mutex>
#include <condition_variable>

using namespace std;

class Monitor {
private:
  queue<int> buffer;
  const unsigned int max_size;
  mutex mtx;
  condition_variable not_full, not_empty;

public:
  Monitor(unsigned int size) : max_size(size) {}

  void produce(int item) {
    unique_lock<mutex> lock(mtx);
    not_full.wait(lock, [&] { return buffer.size() < max_size; });
    buffer.push(item);
    not_empty.notify_one();
  }

  void consume() {
    unique_lock<mutex> lock(mtx);
    not_empty.wait(lock, [&] { return !buffer.empty(); });
    buffer.pop();
    not_full.notify_one();
  }
};
```

---

## Liveness

- liveness 是一組屬性，這些屬性確保系統中的 process 在其執行生命週期能告持續取得前進
- 如果臨界區無限等待，那麼就是一個失去 liveness failure 的範例
  - 失去 liveness 的原因
    - Indefinite waiting
      - 如果一個 process 無限期等待其他process釋放資源，則它的活性失效。
      - 這違反了臨界區段的進度條件（progress）和有限等待條件（bounded-waiting）。
    - infinite loop
    - busy-waiting

```cpp
#include <bits/stdc++.h>

using namespace std;

atomic<bool> lock_flag(false);

void busy_wait_example(int id){
  while(lock_flag){}

  lock_flag = true;
  cout << "Thread " << id << " is in critical section." << endl;
  this_thread::sleep_for(chrono::milliseconds(1000));
  lock_flag = false;
}

int main(){
  thread t1(busy_wait_example,1);
  thread t2(busy_wait_example,2);

  t1.join();
  t2.join();
}
```

### Priority Inversion

- 高優先級 process 被低優先級 process block 並延遲執行。
- 解決方案：**Priority-Inheritance Protocol**，讓低優先級 process 繼承高優先級，快速完成臨界區操作。

---

## Synchronization Examples

### Producer-consumer problem 或 Bounded-buffer problem

![圖 6. : Producer-consumer problem 2](https://imgur.com/2Io3aEB.png)

**有界緩衝區問題**（Bounded-Buffer Problem）是由**生產者**和**消費者**共同存取同一個**緩衝區**（buffer）所引發的同步問題。由於生產者負責向緩衝區中添加資料，而消費者負責從緩衝區中取出資料，這種「共享資源」的情境可能導致以下問題：

1. **緩衝區競爭條件（Race Condition）**：

    若生產者和消費者同時訪問緩衝區，可能導致資料損壞或讀寫錯誤。

2. **緩衝區溢出（Overflow）**：

    生產者在緩衝區已滿的情況下繼續生產，可能導致資料丟失或程式崩潰。

3. **緩衝區匱乏（Underflow）**：

    消費者在緩衝區空的情況下繼續消費，可能導致取出的資料無效。

為了解決這些問題，**使用了三個 semaphore** 進行同步和資源管理：

![圖 7. : semaphore](https://imgur.com/zfbspoe.png)

1. **`mutex` 信號**：

    用於確保生產者和消費者不能同時訪問緩衝區，避免競爭條件。

2. **`empty` 信號**：

    用於追蹤緩衝區中剩餘的空位數量，確保生產者在空間不足時停止操作。

3. **`full` 信號**：

    用於追蹤緩衝區中已填充的資料數量，確保消費者在無資料可用時停止操作。

- 範例
  
  ![圖 8.1 : 範例](https://imgur.com/LUk8O10.png)
  ![圖 8.2 : 範例](https://imgur.com/my4R14j.png)
  - 當 t1 是一個 Consumer 發現 wait(full), full == 0 的時候發現所以會 waiting
  - 當 t2 是一個 Producer 的時候 wait(empty), empty ⇒ n - 1 and wait(mutex) mutex ⇒ 0，所以 Entering, when Producer Exiting 經過 Signal 所以 full += 1 和 mutex += 1
  - t4 因為 full 是 1 了所以 Wakeup
  - t5 就是 Consumer 消費完成
  
### Readers-Writers problem

**讀者-寫者問題**描述了一個場景：多個 process 需要同時存取共享資料，這些 process 分為兩類：

1. **讀者（Readers）**：只讀取資料，不會修改。
2. **寫者（Writers）**：可以讀取和修改資料。

**關鍵問題**：

- 允許多個讀者同時讀取資料，因為讀操作不會相互影響。
- 但只有一個寫者可以同時訪問資料，因為寫操作可能會改變資料，並且不能與其他寫者或讀者同時進行。

**引發的問題：**

- 允許**多個讀者同時讀取**。
- 保證**只有一個寫者可以修改資料**，並在修改時禁止任何讀者訪問。
- 防止**饑餓問題**（Starvation），即：
  - 不讓讀者無限等待寫者完成。
  - 也不讓寫者無限等待讀者完成。

**解決問題：**

- 使用 semaphore
  - rw_mutex：確保 writer 的互斥
  - mutex：用來保護 read_count
  - read_count：追蹤當前有多少個 reader 正在讀取
- 流程
  - reader：
    - 當第一個讀者到來時，鎖定 `rw_mutex`，阻止寫者進入。
    - 當最後一個讀者完成時，釋放 `rw_mutex`，允許寫者進入。

        ```cpp
        while(true){
         wait(mutex);            // 獲取互斥鎖，保護 read_count
         read_count++;
         if (read_count == 1)    // 如果是第一個讀者
             wait(rw_mutex);     // 鎖定寫者的訪問權限
         signal(mutex);          // 釋放互斥鎖
         
         // 讀取操作
         
         wait(mutex);            // 獲取互斥鎖，保護 read_count
         read_count--;
         if (read_count == 0)    // 如果是最後一個讀者
             signal(rw_mutex);   // 釋放寫者的訪問權限
         signal(mutex);          // 釋放互斥鎖
        }
        ```

  - writer：
    - 獨占 `rw_mutex`，阻止任何其他讀者或寫者。

        ```cpp
        while(true){
         wait(rw_mutex);         // 鎖定寫者的訪問權限
         
         // 寫入操作
         
         signal(rw_mutex);       // 釋放寫者的訪問權限
        }
        ```

### The Dining-Philosophers Problem

#### **問題描述**

哲學家進餐問題描述了一個同步情境：

1. 有 **N 位哲學家** 坐在一張圓桌旁，桌上有一碗米飯和 **N 根筷子**。
2. 每位哲學家有兩種狀態：**思考** 和 **吃飯**。
3. 哲學家需要兩根筷子才能進食（每人左右各一根）。
4. 哲學家在完成進食後，會放下兩根筷子並繼續思考。

**問題**：

- 確保哲學家在吃飯和放下筷子的過程中不發生死鎖（Deadlock）。
- 防止哲學家因無法同時取得兩根筷子而陷入飢餓（Starvation）。

#### **基本解法**

使用 **信號量**（Semaphore）來模擬筷子：

1. 每根筷子用一個信號量表示，初始值設為 `1`，表示筷子空閒可用。
2. 哲學家執行流程：
    - **取筷子**：使用 `wait()` 嘗試獲取左手和右手筷子。
    - **進食**：吃完後放下筷子。
    - **放筷子**：使用 `signal()` 釋放左右筷子。

```cpp
while (true) {
    wait(chopstick[i]);               // 嘗試取左筷子
    wait(chopstick[(i+1) % N]);       // 嘗試取右筷子
    // 吃飯
    signal(chopstick[i]);             // 放下左筷子
    signal(chopstick[(i+1) % N]);     // 放下右筷子
    // 思考
}
```

#### **潛在問題**

1. **死鎖（Deadlock）**：
    - 如果每位哲學家同時取走左手筷子，右手筷子將無法被其他人取用，導致所有哲學家都陷入等待狀態。
2. **飢餓（Starvation）**：
    - 哲學家可能因為其他人反覆佔用筷子，長時間無法進食。

#### **解決方案**

##### 1. 限制同時進餐的哲學家數量

- **限制最多只有 4 位哲學家同時進餐**（假設有 5 位哲學家）。
- 通過額外的信號量來控制進餐的哲學家數量，防止所有人同時競爭。

##### 2. 批量檢查筷子的可用性

- **哲學家只有在左右兩根筷子同時可用時才能取筷子**。
- 避免一次取左筷子後無法取得右筷子的情況。

##### 3. 非對稱分配策略（Asymmetric Solution）

- 規定哲學家按不同的順序取筷子：
  - **奇數編號哲學家**：先取左筷子，再取右筷子。
  - **偶數編號哲學家**：先取右筷子，再取左筷子。
- 此方法打破了所有人同時取相同筷子的對稱性。