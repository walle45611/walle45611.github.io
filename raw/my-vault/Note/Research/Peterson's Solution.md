
## 演算法 1：使用 turn 變數 (Strict Alternation)

此方法嘗試使用一個共享的「輪流」變數來強制互斥。

### 1. 程式碼邏輯 (Process $P_i$)

- **共享變數**：`int turn;` (初始值為 $i$ 或 $j$)
    
- **邏輯**：

```
do {
    // Entry Section
    while (turn != i); // 忙碌等待 (Busy waiting)
    
    /* critical section */
    
    // Exit Section
    turn = j; // 將權限交給對方
    
    /* remainder section */

} while (true);
```

### 2. 正確性分析

| **條件**               | **結果**   | **證明 / 說明**                                                                                                                                                                                                                          |
| -------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Mutual Exclusion** | ✅ **滿足** | 由於 `turn` 在任何時間點只能儲存 $i$ 或 $j$ 其中一個值，因此最多只有一個處理程序能夠通過 while 迴圈進入 CS。                                                                                                                                                                 |
| **Progress**         | ❌ **違反** | 如果 $P_i$ 想進入 CS 但此時 $turn == j$，即使 $P_j$ 正在剩餘區域 (Remainder Section) 執行而不打算進入 CS，$P_i$ 也必須被迫等待。這違反了前進性（由不想進入 CS 的行程阻擋了想進入的行程），更簡單的來說就是會有嚴格的輪流限制也就是 $P_i$ 進入 CS 之後把 $turn==j$ 之後 $P_j$ 其實並無意願，但是這時候在 remainder section 的 $P_i$ 又想要進入了。 |
| **Bounded Waiting**  | ✅ **滿足** | 雖然滿足，但因為前進性已違反，此條件在此處討論意義不大。                                                                                                                                                                                                         |

---

## 演算法 2：使用 flag 旗標

此方法嘗試讓處理程序「宣布」它們進入臨界區域的意願。

### 1. 程式碼邏輯 (Process $P_i$)

- **共享變數**：`boolean flag[2];` (初始值皆為 `false`)
    
- **邏輯**：

```
do {
    // Entry Section
    flag[i] = true;    // 宣示意外：我想進入
    while (flag[j]);   // 檢查對方是否也想進入
    
    /* critical section */
    
    // Exit Section
    flag[i] = false;   // 離開 CS，取消意願
    
    /* remainder section */

} while (true);
```

### 2. 正確性分析

| **條件**               | **結果**   | **證明 / 說明**                                                                                                     |
| -------------------- | -------- | --------------------------------------------------------------------------------------------------------------- |
| **Mutual Exclusion** | ✅ **滿足** | 若 $P_i$ 進入 CS，則 $flag[j]$ 必為 `false`。若兩者同時嘗試進入，其中一個會先看到對方的旗標為 `true` 而被擋住。                                      |
| **Progress**         | ❌ **違反** | **Deadlock** 風險。如果 $P_i$ 和 $P_j$ 幾乎同時執行到 `flag[i] = true` 和 `flag[j] = true`，接著兩者都會卡在 `while` 迴圈中互相等待，無人能進入 CS。 |
| **Bounded Waiting**  | ❌ **違反** | 如果產生死結，則兩者都無限期等待。                                                                                               |

---

## 演算法 3：Peterson's Solution (彼得森解法)

Peterson's Solution 結合了上述兩種方法的優點：

1. 使用 `flag` 宣示意願。
    
2. 使用 `turn` 作為發生爭用時的決策者 (tie-breaker)。
    

### 1. 程式碼邏輯 (Process $P_i$)

- **共享變數**：`int turn;` 和 `boolean flag[2];` (初始值：`flag` 皆為 `false`)
    
- **邏輯**：

```
while (true) {
    // Entry Section
    flag[i] = true;          // 1. 我想進入
    turn = j;                // 2. 禮讓給對方 (若對方也想進，對方優先)
    
    // 檢查條件：對方想進 (flag[j]) 且 輪到對方 (turn == j)
    while (flag[j] && turn == j); 
    
    /* CRITICAL SECTION */
    
    // Exit Section
    flag[i] = false;         // 我不想進了
    
    /* REMAINDER SECTION */
}
```

### 2. 正確性分析

Peterson's Solution 滿足所有臨界區域設計的三個必要條件：

#### A. Mutual Exclusion: ✅ 滿足

- **證明**：若 $P_i$ 和 $P_j$ 同時位於 CS，則必須同時滿足 `flag[i] == true`, `flag[j] == true`。
    
- 但要通過 `while` 迴圈，`turn` 必須分別不等於 $j$ (對 $P_i$ 而言) 和不等於 $i$ (對 $P_j$ 而言)。
    
- 然而 `turn` 變數在同一時刻只能是 $i$ 或 $j$，不可能同時滿足 `turn == i` 和 `turn == j`。因此互斥成立。
    

#### B. Progress: ✅ 滿足

- **證明**：若 $P_i$ 想進入 CS，它只會被 $P_j$ 阻擋。阻擋條件是 $P_j$ 也想進 `(flag[j] == true)` 且 $P_j$ 擁有優先權 `(turn == j)`。
    
- 如果 $P_j$ 不想進 (在 R.S.)，則 `flag[j]` 為 `false`，$P_i$ 直接進入。
    
- 如果 $P_j$ 也想進，當它執行 `turn = i` 時 (禮讓)，如果 $P_i$ 已經在等待，則 `turn` 變為 $i$， $P_i$ 的等待條件被打破，得以進入。這保證了不會發生死結。
    

#### C. Bounded Waiting: ✅ 滿足

- **證明**：如果 $P_i$ 正在等待，代表 `flag[j] == true` 且 `turn == j`。
    
- $P_j$ 進入 CS 並完成後，會將 `flag[j]` 設為 `false`，這時 $P_i$ 可以進入。
    
- 即便 $P_j$ 立刻再次想進入，它會執行 `turn = i`。因為 $P_j$ 是最後一個寫入 `turn` 的人，這次 `turn` 會變成 $i$，導致 $P_j$ 必須等待，$P_i$ 優先進入。
    
- 因此 $P_i$ 最多等待 $P_j$ 進入一次，即滿足有限等待。
	
- 更簡單的說就是因為最後移行的 `turn=j` 或是 `turn=i` 的緣故，所以結束過後會把這個執行權利交給對方，也就是說在下一次他們會等一次而已。 
    

---

## ⚠️ 現代架構下的限制 (Modern Architecture Limitations)

> [!WARNING] Peterson's Solution 在現代電腦上可能失效
> 
> 儘管 Peterson's Solution 在理論上正確，但在現代處理器與編譯器上不保證能正確運作。

### 原因：重新排序 (Reordering)

- **問題**：現代處理器 (Processors) 和編譯器 (Compilers) 為了效能最佳化，可能會對**沒有資料相依性 (Data Dependency)** 的讀寫操作進行指令重新排序。
    
- **後果**：在 Peterson's Solution 中，Entry Section 的兩個操作：
    
    1. `flag[i] = true;`
        
    2. `turn = j;`
        
    
    - 這兩者對處理器來說沒有直接相依，可能會被對調執行順序。若發生重排，可能導致兩個程序都尚未設定旗標就先檢查對方，進而同時進入 CS，導致 **互斥性 (Mutual Exclusion) 被違反**。

### 解決方案

純軟體解法（如 Peterson's）在現代系統已不夠可靠，通常需搭配硬體支援或高階 API：

- **硬體指令**：`TestAndSet`, `CompareAndSwap (CAS)`, `Memory Barriers (Fences)`.
    
- **高階軟體 API**：`Mutex Locks`, `Semaphores`, `Monitors`.