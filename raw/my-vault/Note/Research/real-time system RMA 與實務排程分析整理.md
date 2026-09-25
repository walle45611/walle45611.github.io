## 一、RMA 基本模型與測試

### 1. 基本假設

經典 Rate Monotonic Analysis（RMA）假設：

- 任務集合：$\tau_i$，皆為 periodic task
    
    - WCET（執行時間）：$C_i$
        
    - 週期：$P_i$
        
    - Deadline：$D_i = P_i$
        
- 單一處理器、固定優先權
    
    - 優先權依 period 決定：週期越短 → 優先權越高（Rate Monotonic, RM）
        
- 任務彼此獨立
    
    - 沒有共享資源、沒有阻塞
        
- 理想環境
    
    - 無中斷、任務不會自行 suspend
        
    - context switch overhead 先視為 0
        

後面所有「加入 context switching / interrupt / blocking / 同步協定」都是在這個 baseline 上的修正。


### 2. 利用率上界測試（Utilization Bound, UB）

每個任務的利用率：

$$  
U_i = \frac{C_i}{P_i}, \qquad  
U = \sum_{i=1}^n U_i  
$$

Liu & Layland 上界：

$$  
U \le U(n) = n\left(2^{1/n} - 1\right)  
$$

- 若 $U \le U(n)$  
    ⇒ 在 RMA 假設下 **必定 schedulable**（充分條件）
    
- 若 $U > U(n)$  
    ⇒ 測試「不保證」，實際上可能排得進去，也可能不行
    

特例：

- $n = 1$：$U(1) = 1$  
    ⇒ 單一任務：$C_1 \le P_1$ 是必要且充分條件
    
- $n \to \infty$：$U(n) \to \ln 2 \approx 0.693$
    

---

### 3. Completion-time / Response-time 測試（Theorem 3）

對第 $i$ 個任務 $\tau_i$（依 period 由小到大排序）：

- 檢查所有比它優先權高的任務 $\tau_j,; j < i$
    
- 目標：求 worst-case response time $W_i$
    

無 blocking 的迭代公式：

$$  
W_i^{(k+1)} =  
C_i +  
\sum_{j < i}  
\left\lceil \frac{W_i^{(k)}}{P_j} \right\rceil C_j  
$$

流程：

1. 設 $W_i^{(0)} = C_i$（或 $0$）
    
2. 反覆代入，直到：
    
    - 收斂：$W_i^{(k+1)} = W_i^{(k)} = W_i$  
        檢查 $W_i \le D_i$ ⇒ $\tau_i$ schedulable
        
    - 或某次迭代 $W_i^{(k)} > D_i$ ⇒ 立即判定 $\tau_i$ 不可排
        

這個測試比 UB 精確，是實務上最常用的分析方法。

---

### 4. Schedulability Point / Theorem 2（processor demand）

概念：在一組「關鍵時間點」$t \in R_i$ 上，比較：

- 在 $[0, t]$ 內所有必須完成的工作量（processor demand）
    
- 是否小於等於 $t$
    

若對所有 $t \in R_i$ 都滿足：

$$  
\text{demand}(t) \le t  
$$

則 $\tau_i$ schedulable。

實務重點：

- UB 太保守時，可退到 Theorem 2 / Theorem 3
    
- 通常考試與手算優先用 Theorem 3（response time iteration）
    

---

## 二、Context Switching、Interrupt 與 Blocking 建模

### 1. Context Switching Overhead

在 preemptive、priority-based scheduling 下，每個 task 每個 period 通常會有兩次排程動作：

- 週期開始時被排入
    
- 週期結束時讓出 CPU
    

若每次排程動作的成本為 $s$，則可把「有效執行時間」改成：

$$  
U_i = \frac{C_i + 2s}{P_i}  
$$

也就是在分析 schedulability 時，直接把 context switch overhead 加到 $C_i$ 裡面。

---

### 2. Cyclic Executive 與 Run-time Scheduling

- Cyclic executive：
    
    - 事先排好一個 major frame，所有任務被切成固定 slot 順序執行，非搶先
        
    - run-time scheduling overhead 幾乎為 0
        
- Run-time scheduling（例如 RM）：
    
    - 每次事件都要決定下一個執行的任務，看起來有較高的 scheduling overhead
        

隱藏問題：

- 為了讓所有 task 的 period 整齊塞進 cyclic frame，常被迫縮短 period 或拆小任務
    
- 造成 **utilization 人為放大**，但實際有用工作量沒變
    

結論：run-time scheduling 不一定比較浪費，有時反而因為不需要扭曲 period/phase，整體 utilization 更好。

---

### 3. Priority Inversion 與來源

定義：

> 高優先權任務被低優先權任務擋住無法執行，所造成的延遲。

常見來源：

- Non-rate-monotonic priority assignment  
    優先權設定未遵守 RM，某些低頻任務被給了過高優先權
    
- Non-preemptibility  
    某些區段不可被搶先，高優先權任務被迫在外等待
    
- Interrupts  
    中斷 handler 通常優先權極高，可能長時間搶走 CPU
    
- Not enough priority levels  
    優先權層級不足，高低優先權任務被迫混在一起
    
- FIFO queues  
    純 FIFO 不看 priority，容易讓低優先權工作排在前面
    
- Synchronization  
    共享資源與 lock，讓高優先權任務被低優先權 locker 阻塞
    

在 schedulability 分析中，需要把這些影響「找出來、建模、算出上界」。

---

### 4. 中斷（Interrupt）的建模：兩種做法

令 $\tau_{\text{int}}$ 代表某個週期性中斷（例如 timer）：

- 執行時間：$C_{\text{int}}$
    
- 週期：$P_{\text{int}}$
    
- 利用率：$U_{\text{int}} = C_{\text{int}} / P_{\text{int}}$
    

#### 4.1 把中斷當成獨立 periodic task

- 任務集合變成：$\tau_1, \tau_2, \tau_{\text{int}}, \tau_3, \dots$
    
- 利用率上界測試：
    
    $$  
    \sum_i \frac{C_i}{P_i} + \frac{C_{\text{int}}}{P_{\text{int}}} \le U(n+1)  
    $$
    
- 在 response-time / Theorem 2 裡，也把 $\tau_{\text{int}}$ 當成一個高優先權干擾源
    

#### 4.2 把中斷當成 blocking time（或額外執行時間）

若某任務 $\tau_i$ 在自己一個 period 內，最多只會被中斷 handler 卡住一次，可以把這段時間視為 blocking time：

$$  
B_i = C_{\text{int}}\quad  
(\text{或「最多一次的中斷總長」})  
$$

分析 $\tau_i$ 時：

- UB 版本：
    
    $$  
    \sum_{j<i} \frac{C_j}{P_j} +  
    \frac{C_i + B_i}{P_i}  
    \le U(i)  
    $$
    
- Response-time（Theorem 3）版本：
    
    $$  
    W_i^{(k+1)} =  
    C_i + B_i +  
    \sum_{j<i}  
    \left\lceil \frac{W_i^{(k)}}{P_j} \right\rceil C_j  
    $$
    

投影片中看到：

- 對 $\tau_1$：$(C_1 + C_{\text{int}})/P_1$
    
- 對 $\tau_2$：$C_1/P_1 + (C_2 + C_{\text{int}})/P_2$
    
- 對 $\tau_3$：$\sum C_j/P_j + C_{\text{int}}/P_{\text{int}}$
    

就是在切換兩種觀點：  
有時把中斷視為 blocking $B_i$，有時視為獨立 task $\tau_{\text{int}}$。

---

### 5. Rule of Thumb：三種效應的頻率

- Preemption effects
    
    - 一個 period 內可能被 preempt 很多次
        
    - 對高優先權任務的干擾需透過 response-time 中的 $\sum \lceil W/P_j\rceil C_j$ 處理
        
- Execution effects
    
    - 任務本體 $C_i$ 每個 period 執行一次
        
- Blocking effects
    
    - 每個 blocking source 在一個 period 內對某任務「頂多一次」
        
    - 所以 $B_i$ 通常可以當成固定常數加入，無需乘以 ceiling
        

因此在 response-time 公式中，blocking 是常數 $B_i$，其他高優先權任務則用 ceiling 計數。

---

## 三、同步協定與阻塞時間

### 1. 評估同步協定的三個指標

1. Bounded Priority Inversion  
    能否為 priority inversion 給出明確上界（也就是 $B_i$ 可被計算）
    
2. Blocked at Most Once  
    每個任務在存取共享資源時，是否最多只會被 block 一次
    
3. Deadlock Avoidance  
    協定本身是否能避免 deadlock
    

---

### 2. 常見協定特性

1. **Nonpreemptible Critical Sections**
    
    - 作法：critical section（CS）內禁止 preemption
        
    - 特性：
        
        - Bounded priority inversion：Yes  
            ⇒ $B_i$ 可由 CS 長度總和估計
            
        - Blocked at most once：Yes¹（假設 CS 內不自行 suspend）
            
        - Deadlock avoidance：Yes¹（假設沒有錯誤的鎖序）
            
    - 缺點：CS 不能太長，否則整體延遲變大
        
2. **Highest Locker’s Priority**
    
    - 作法：持有 lock 的任務暫時提升到該資源使用者中的最高優先權
        
    - 特性：
        
        - Bounded priority inversion：Yes
            
        - Blocked at most once：Yes¹
            
        - Deadlock avoidance：Yes¹
            
3. **BIP（Basic Inheritance Protocol）**
    
    - 作法：低優先權 holder 繼承被它擋住的高優先權任務的 priority
        
    - 特性：
        
        - Bounded priority inversion：Yes
            
        - Blocked at most once：No（不同資源可能多次被 block）
            
        - Deadlock avoidance：No（缺少資源排序）
            
4. **PCP（Priority Ceiling Protocol）**
    
    - 作法：每個資源設一個 priority ceiling，限制鎖取得順序
        
    - 特性：
        
        - Bounded priority inversion：Yes
            
        - Blocked at most once：Yes²（假設沒有亂 suspend）
            
        - Deadlock avoidance：Yes（協定本身避免循環等待）
            
    - 對分析的好處：
        
        - 可以算出每個任務的最大 blocking time $B_i$
            
        - 然後直接把 $C_i$ 改成 $C_i + B_i$ 套進 UB / Theorem 2 / Theorem 3
            

---

## 四、RMA 典型算例模板

### 範例 1：UB 測試直接通過（最基本）

任務集合：

- $\tau_1 : C_1 = 10,; P_1 = 50$
    
- $\tau_2 : C_2 = 15,; P_2 = 100$
    
- $\tau_3 : C_3 = 20,; P_3 = 200$
    

利用率：

$$  
U_1 = \frac{10}{50} = 0.2,\quad  
U_2 = \frac{15}{100} = 0.15,\quad  
U_3 = \frac{20}{200} = 0.1  
$$

總利用率：

$$  
U = 0.2 + 0.15 + 0.1 = 0.45  
$$

三個任務的上界：

$$  
U(3) = 3(2^{1/3}-1) \approx 0.78  
$$

因為 $U = 0.45 \le U(3)$  
⇒ 在標準 RMA 假設下必定 schedulable，不必再做 Theorem 2/3。

---

### 範例 2：UB 失敗，但用 Theorem 3 證明可排（無 blocking）

任務集合（投影片例子）：

- $\tau_1 : C_1 = 40,; P_1 = 100$
    
- $\tau_2 : C_2 = 40,; P_2 = 150$
    
- $\tau_3 : C_3 = 100,; P_3 = 350$
    

利用率：

$$  
U_1 = 0.4,\quad  
U_2 \approx 0.267,\quad  
U_3 \approx 0.286,\quad  
U \approx 0.952  
$$

$U(3) \approx 0.78$，因此 $U > U(3)$，UB 不通過。

改用 Theorem 3 對最低優先權的 $\tau_3$：

$$  
W_3^{(k+1)} =  
C_3

- \left\lceil \frac{W_3^{(k)}}{P_1} \right\rceil C_1
    
- \left\lceil \frac{W_3^{(k)}}{P_2} \right\rceil C_2  
    $$
    

迭代：

- $W_3(0) = 0$
    
- $W_3(1) = 100$
    
- $W_3(2) = 180$
    
- $W_3(3) = 260$
    
- $W_3(4) = 300$
    
- $W_3(5) = 300$（收斂）
    

比較 deadline：

$$  
W_3 = 300 \le D_3 = P_3 = 350  
$$

⇒ 雖然 UB 失敗，但此任務集合實際上是 schedulable。

---

### 範例 3：有 periodic interrupt，直接當成一個 task

加入一個 timer interrupt：

- $\tau_1 : C_1 = 20,; P_1 = 100$
    
- $\tau_2 : C_2 = 40,; P_2 = 150$
    
- $\tau_{\text{int}} : C_{\text{int}} = 60,; P_{\text{int}} = 200$
    
- $\tau_3 : C_3 = 20,; P_3 = 350$
    

利用率：

$$  
U_1 = 0.2,\quad  
U_2 \approx 0.267,\quad  
U_{\text{int}} = 0.3,\quad  
U_3 \approx 0.057  
$$

$$  
U_{\text{total}} \approx 0.824  
$$

四個任務的上界：

$$  
U(4) = 4(2^{1/4}-1) \approx 0.757  
$$

$U_{\text{total}} > U(4)$  
⇒ 若把 interrupt 當成獨立 task，UB test 不通過，需要改用其他模型。

---

### 範例 4：同一個 interrupt，改用 blocking time + Theorem 3

只看三個一般任務：

- $\tau_1 : C_1 = 20,; P_1 = 100$
    
- $\tau_2 : C_2 = 40,; P_2 = 150$
    
- $\tau_3 : C_3 = 20,; P_3 = 350$
    

interrupt：

- $\tau_{\text{int}} : C_{\text{int}} = 60,; P_{\text{int}} = 200$
    

假設每個 period 最多被 interrupt 卡一次：

- 對 $\tau_1$：$B_1 = C_{\text{int}} = 60$
    
    $$  
    \frac{C_1 + B_1}{P_1}  
    = \frac{20+60}{100}  
    = 0.8 \le 1  
    $$
    
- 對 $\tau_2$：$B_2 = 60$
    
    $$  
    \frac{C_1}{P_1} + \frac{C_2 + B_2}{P_2}  
    = \frac{20}{100} + \frac{100}{150}  
    \approx 0.867 > U(2) \approx 0.828  
    $$
    

UB 再次失敗，改用含 blocking 的 Theorem 3：

$$  
W_2^{(k+1)} =  
C_2 + B_2 +  
\left\lceil \frac{W_2^{(k)}}{P_1} \right\rceil C_1  
$$

代入：

- $W_2(0) = 0$
    
- $W_2(1) = 40 + 60 + 0 = 100$
    
- $W_2(2) = 40 + 60 + 20 = 120$
    
- $W_2(3) = 40 + 60 + 40 = 140$
    
- $W_2(4) = 140$（收斂）
    

比較 deadline：

$$  
W_2 = 140 < D_2 = P_2 = 150  
$$

⇒ 在「interrupt 當 blocking」模型下，$\tau_2$ 仍 schedulable。

---

### 範例 5：interrupt 太頻繁導致不可排

延續範例 4，把 interrupt 改成「每 60 單位一次」：

- $\tau_1 : C_1 = 20,P_1 = 100$
    
- $\tau_2 : C_2 = 40,P_2 = 150$
    
- $\tau_3 : C_3 = 20,P_3 = 350$
    
- $\tau_{\text{int}} : C_{\text{int}} = 60,; P_{\text{int}} = 60$
    

直接當 task：

$$  
U_1 = 0.2,\quad  
U_2 \approx 0.267,\quad  
U_{\text{int}} = 1.0,\quad  
U_3 \approx 0.057  
$$

$$  
U_{\text{total}} \approx 1.524 > 1  
$$

單處理器必定不可排。

若當 blocking 來看：

- 在 $[0,100]$ 期間，$\tau_1$ 最多被 interrupt 打斷兩次  
    ⇒ $B_1 \approx 2C_{\text{int}} = 120$
    

檢查：

$$  
C_1 + B_1 = 20 + 120 = 140 > P_1 = 100  
$$

光看 $\tau_1$ 就塞不進其 period，必定 miss deadline。  
說明：中斷太長、太頻繁時，不論用 task 或 blocking 模型，結論都會是 unschedulable。