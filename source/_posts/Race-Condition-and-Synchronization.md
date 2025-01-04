---
title: Race Condition and Synchronization in Process Management
date: 2025-01-04
categories: [Operating System]
tags: [Concurrency, Critical Section, Priority Inversion]
---

![圖 1. : Blog Image](https://imgur.com/UkVANiR.png)

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

| **特性** | **Spinlock** | **Mutex Lock** |
| --- | --- | --- |
| **等待行為** | 忙等待（Busy Waiting），不斷輪詢鎖的狀態。 | 阻塞等待（Blocking Wait），進入睡眠直到被喚醒。 |
| **性能** | 適合短時間臨界區段，避免上下文切換的開銷。 | 適合長時間臨界區段，避免浪費 CPU 資源。 |
| **使用場景** | 多用於內核或高性能場景，臨界區段時間極短時。 | 多用於應用層或臨界區段時間較長的場景。 |
| **資源消耗** | 高（因為需要輪詢，占用 CPU）。 | 低（CPU 可用於其他任務）。 |
| **上下文切換開銷** | 無上下文切換開銷（因為不進入睡眠）。 | 有上下文切換開銷（因為進入睡眠後需被喚醒）。 |
| **公平性** | 可能導致飢餓問題（某些執行緒可能一直輪詢不到）。 | 相對公平（內部可能採用排隊機制）。 |

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

### Priority Inversion

- 高優先級 process 被低優先級 process block 並延遲執行。
- 解決方案：**Priority-Inheritance Protocol**，讓低優先級 process 繼承高優先級，快速完成臨界區操作。
