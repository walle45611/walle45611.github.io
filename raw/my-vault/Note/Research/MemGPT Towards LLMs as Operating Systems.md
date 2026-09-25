[[Assets/Note/Research/Papers/MemGPT - Towards LLMs as Operating Systems/MemGPT - Towards LLMs as Operating Systems.pdf|論文 PDF]]

## 初讀摘要

- MemGPT 借用作業系統記憶體階層的概念，讓有限 context window 的模型管理更大的外部資訊。
- 系統在工作上下文與外部儲存之間調用、保存和檢索內容，藉由工具控制哪些資訊當下進入模型。
- 在長文件分析與跨多次對話中展示持續使用記憶的能力；它增加的是可管理的資訊範圍，並非一次把所有資料放入模型上下文。

摘要依據：PDF 摘要與導論。


＠# MemGPT: Towards LLMs as Operating Systems

這篇論文的核心主張是：**不要只把 LLM 的 context window 做大，而是讓 LLM 像作業系統一樣管理有限的記憶體。**

MemGPT 把 LLM 的 context window 類比成作業系統裡有限的 RAM，把外部資料庫、長期記憶、文件庫類比成 disk / external storage。模型不是一次看完所有資料，而是透過 function calls 主動保存、搜尋、取回、更新外部記憶，讓固定 context window 的 LLM 看起來像有更大的有效上下文。

最短版：

> MemGPT 是一個 OS-inspired memory management system for LLMs。它把 `main context` 當成有限的 main memory，把 `external context` 當成外部儲存，並透過 `Queue Manager`、`Function Executor`、`working context`、`FIFO queue`、`recall storage`、`archival storage`、`memory pressure warning`、`recursive summary` 和 `function chaining`，讓 LLM 可以像作業系統管理記憶體一樣管理自己的上下文。

## 1. 這篇論文想解決什麼問題？

LLM 的根本限制是 **context window 有限**。

模型每次推論時，只能看到被放進 context window 的 tokens。超出 context window 的內容，模型當下就看不到。這會造成兩類問題。

第一類是 **長期對話問題**。如果一個聊天代理要跟使用者互動數天、數週、數月，對話歷史一定會超過 context window。早期對話被擠出去後，模型就可能忘記使用者過去說過的生日、偏好、人際關係、任務狀態，甚至忘記某些資訊已經更新。

例如：

- 使用者以前說生日是 2 月 7 日。
- 使用者以前說男朋友叫 James。
- 後來使用者說已經和 James 分手。
- 如果模型只靠短期 context，它之後可能還把 James 當成現任男友。

第二類是 **長文件 / 多文件分析問題**。法律文件、財報、醫療紀錄、Wikipedia 文件集合，很容易超過模型一次能看的長度。即使有些模型支援 128k tokens，也不代表真實任務都塞得進去，更不代表模型能有效使用所有 token。

![[Assets/Note/Research/Papers/MemGPT - Towards LLMs as Operating Systems/memgpt-context-window-comparison.png|600]]

這張圖說明不同模型的 context window 大小差很多，但再大的 context window 仍然是有限資源。論文的重點不是否定長 context，而是指出「只靠變長」不夠。

原因有三個：

- Transformer self-attention 的計算與記憶體成本會隨 context length 增加而快速上升。
- 長 context 不等於有效 context。像 Lost in the Middle 這類研究指出，模型對中間位置資訊的利用能力可能比較差。
- 訓練與推理超長 context 模型成本高，而且邊際效益不一定穩定。

所以作者提出另一個方向：

> 與其一直把 context window 做大，不如讓 LLM 學會管理有限 context window。

這就是 MemGPT 的出發點。

## 2. 核心類比：LLM context window = RAM

MemGPT 的核心類比來自作業系統的 **virtual memory**。

在傳統作業系統中，程式不一定真的擁有很大的實體記憶體 RAM。作業系統會透過 virtual memory，讓程式感覺自己有更大的記憶體空間。當 RAM 不夠時，不常用的資料會被放到 disk；等程式需要時，再把資料 page in 回 RAM。

也就是：

- RAM 是快但有限的 main memory。
- Disk 是慢但容量大的 external storage。
- OS 負責決定哪些資料留在 RAM，哪些資料移出去，何時再取回。

MemGPT 把這個概念搬到 LLM：

- `main context` = LLM 當下真正看得到的 context window，類似 RAM。
- `external context` = LLM 當下看不到，但可透過工具讀寫的外部記憶，類似 disk / database。
- `function calls` = LLM 操作外部記憶體的系統呼叫。
- `Queue Manager` / `Function Executor` = 管理 context、記憶、工具執行的系統層。

所以 MemGPT 不是改造 Transformer 架構，也不是單純訓練一個更長 context 的模型，而是在 LLM 外面加一層記憶體管理系統。

## 3. MemGPT 的記憶體階層

![[Assets/Note/Research/Papers/MemGPT - Towards LLMs as Operating Systems/memgpt-memory-hierarchy.png|700]]

MemGPT 把記憶體分成兩大層：

- `Main Context`
- `External Context`

這個分層非常重要，因為它決定了「模型現在能直接使用什麼」以及「模型需要透過工具取回什麼」。

## 4. Main Context：模型當下真的看得到的內容

`Main Context` 是實際會被送進 LLM 的 prompt tokens。只有在 main context 裡的內容，模型推論時才真正看得到。

如果某個資訊存在外部資料庫，但沒有被取回 main context，模型當下不能直接使用它。這一點很關鍵，因為很多人講「LLM 有外部記憶」時容易講得太抽象。對模型而言，外部記憶不是魔法；外部記憶必須被工具搜尋、取回、放進 context，模型才看得到。

MemGPT 的 main context 不是一整坨文字，而是分成三個區塊：

- `System Instructions`
- `Working Context`
- `FIFO Queue`

### 4.1 System Instructions

`System Instructions` 可以理解成 MemGPT 的 system prompt。它是唯讀、靜態的，告訴 LLM 如何操作 MemGPT 系統。

![[Assets/Note/Research/Papers/MemGPT - Towards LLMs as Operating Systems/memgpt-function-executor.png|650]]

這張圖中的重點是：system instructions 包含兩大部分。

- 記憶體階層的詳細說明，以及每個 memory tier 的用途。
- function schema，讓模型知道可以呼叫哪些函式來讀寫或修改記憶。

換句話說，system instructions 不是普通聊天 prompt，而是 MemGPT 的操作手冊。它會告訴模型：

- main context 是什麼。
- external context 是什麼。
- working context 要怎麼用。
- FIFO queue 是什麼。
- recall storage 和 archival storage 的差別。
- context 快滿時要怎麼保存重要資訊。
- 可以呼叫哪些 function。
- function arguments 長什麼樣。
- function call 出錯後應該如何修正。

所以 system instructions 的角色是：

> 教 LLM 扮演一個會管理記憶體的 agent。

### 4.2 Working Context

`Working Context` 是 main context 裡固定大小、可讀寫的文字區塊。

它在 main context 裡，所以模型每次推論都看得到。它只能透過 MemGPT function calls 修改，而不是讓模型在自然語言回答裡隨便改。

Working context 通常保存「當前任務或長期互動中最重要、必須持續可見的狀態」。

例如：

- 使用者生日是 2 月 7 日。
- 使用者偏好繁體中文。
- 使用者正在準備研究所考試。
- James 以前是男朋友，但現在是前男友。
- 目前任務是整理某份文件。
- agent 的 persona 或行為設定。

它可以想成模型桌面上的重要便條紙。它不是完整聊天紀錄，也不是大型資料庫，而是當前最應該一直留在 context 裡的狀態。

Figure 4 的例子說明了 working context 的可更新性：
![[Assets/Note/Research/Papers/MemGPT - Towards LLMs as Operating Systems/memgpt-working-context-update-example.png|600]]

這表示 MemGPT 不只是會「記住」，也會「更新記憶」。對長期對話代理來說，這比單純保存舊資訊更重要，因為使用者狀態會改變；如果記憶不能演化，記憶反而會變成錯誤來源。

### 4.3 FIFO Queue

`FIFO Queue` 是 main context 裡的近期訊息紀錄。FIFO 是 First In, First Out，先進先出。

==它保存最近的事件流==，包括：

- user messages
- assistant responses
- system messages
- memory pressure warnings
- function call inputs
- function call outputs

當新訊息進來時，Queue Manager 會把訊息 append 到 FIFO queue。接著 MemGPT 會把：

```text
system instructions + working context + FIFO queue
```

串接成完整 prompt，送進 LLM processor。

FIFO queue 的特點是：

> 它是近期上下文，但不是永久保存。

當對話越來越長，FIFO queue 會越來越滿。context window 快滿時，舊訊息會被 evict，也就是移出 main context。

但是 MemGPT 不會直接把舊訊息完全丟掉。FIFO queue 的第一個位置會放一個 `recursive summary`，用來保存已經被移出 queue 的訊息摘要。

recursive summary 的更新邏輯是：

```text
舊摘要 + 新被移出的訊息 -> 新摘要
```

這樣做的好處是，舊訊息離開 main context 後，系統仍然保留一個壓縮後的歷史摘要。

但要注意：

> recursive summary 是有損的。它保留大意，但可能遺失細節。

因此完整舊訊息還是會保存到 `recall storage`。

## 5. External Context：模型當下看不到，但可以透過工具讀寫的記憶

`External Context` 是 LLM 當下看不到，但可以透過 function call 讀取或寫入的外部記憶體。

MemGPT 的 external context 主要包含：

- `Recall Storage`
- `Archival Storage`

兩者都在 main context 外面，但用途不同。

### 5.1 Recall Storage

`Recall Storage` 是完整歷史對話紀錄資料庫，比較像 chat log。

Queue Manager 會把使用者輸入和 LLM 輸出寫入 recall storage。所以即使某些舊訊息被 FIFO queue 擠出 main context，它們仍然存在 recall storage 裡。

之後如果 LLM 需要找以前對話裡的資訊，可以透過 function call 搜尋 recall storage，把相關訊息取回 main context。

例如過去對話裡有一句：

```text
James and I first met at Six Flags.
```

這句話後來離開 FIFO queue 了。之後使用者問：

```text
Where did I first meet James?
```

MemGPT 可以搜尋 recall storage，找回這段舊對話，再把它放回 main context，最後回答 `Six Flags`。

所以 recall storage 的用途是：

> 保存完整對話歷史，讓模型之後可以回憶過去互動。

它偏向原始歷史紀錄。

### 5.2 Archival Storage

`Archival Storage` 是長期知識庫或外部文件庫，比較像整理後的長期記憶或大型資料庫。

它可以存：

- 使用者重要資訊
- 整理後的長期記憶
- 文件內容
- Wikipedia passages
- 外部知識庫
- 任意長度文字物件

在 document QA 實驗裡，作者把 Wikipedia 文件放進 archival storage。實作上使用 PostgreSQL 作為 archival memory storage，並用 pgvector 支援向量搜尋，用 HNSW index 加速搜尋。

大致流程是：

```text
Wikipedia documents / passages
-> embeddings
-> PostgreSQL + pgvector
-> HNSW index
-> archival_storage.search()
-> paginated search results
-> main context
-> LLM answer
```

所以 archival storage 的用途是：

> 保存大量長期資料，讓 LLM 在需要時可以主動檢索。

它偏向整理後的長期知識或文件資料。

### 5.3 Recall Storage 和 Archival Storage 的差別

可以這樣記：

| 類型 | 用途 | 類比 |
|---|---|---|
| Recall Storage | 完整歷史對話紀錄 | raw chat log |
| Archival Storage | 長期知識庫 / 文件庫 | database / wiki / document store |

兩者都在 external context 裡。模型當下看不到，必須透過 function call 把相關片段取回 main context。

## 6. Queue Manager：管理 FIFO queue 和 context overflow

`Queue Manager` 負責管理 FIFO queue、recall storage，以及 context overflow。

它主要做四件事：

- 新訊息進來時，把訊息加入 FIFO queue。
- 把使用者訊息和 LLM 輸出寫入 recall storage。
- 把 system instructions、working context、FIFO queue 串接成 main context，送給 LLM 推論。
- 在 context 快滿或已滿時處理 warning、flush、summary 和 eviction。

這裡有兩個重要門檻：

- `warning token count`
- `flush token count`

### 6.1 70%：Memory Pressure Warning

當 prompt tokens 超過 warning token count，例如 context window 的 70%，Queue Manager 會插入一則 `memory pressure warning` 到 FIFO queue。

這個 warning 的意思是：

> context 快滿了，重要資訊要趕快保存。

這時候只是警告，不會直接清 queue。

LLM 收到警告後，可以透過 MemGPT functions，把 FIFO queue 中的重要資訊保存到：

- working context
- archival storage

例如：

- 使用者生日
- 使用者偏好
- 長期任務目標
- 重要事件
- persona 更新
- 重要文件摘要

重點是：

> 70% 是提醒 LLM 做記憶保存，不是自動 flush。

### 6.2 100%：Flush Token Count

當 prompt tokens 超過 flush token count，例如 context window 的 100%，Queue Manager 會真正執行 flush queue。

這時候它會從 FIFO queue 中 evict 一部分舊訊息，以釋放 context window 空間。論文中舉例可能會 evict 約 50% 的 context window 內容。

注意：

> 50% 不是觸發門檻。正確理解是：100% 觸發 flush，flush 時可能移出約 50% 的舊訊息。

flush 時會發生幾件事：

- 移出一部分 FIFO queue 舊訊息。
- 用 existing recursive summary 加上 evicted messages 產生新的 recursive summary。
- 完整被移出的訊息仍保存到 recall storage。
- 被移出的訊息不再 in-context，所以 LLM 當下看不到。
- 未來如果需要，可以透過 function call 從 recall storage 搜尋回來。

所以這個機制等於：

> context 裡保留摘要，外部保存完整紀錄。

這就是 OS memory hierarchy 的精神：

- RAM 空間有限。
- 不常用資料移出去。
- 保留必要摘要或索引。
- 需要時再從外部存取。

## 7. Function Executor：執行 LLM 產生的 function call

`Function Executor` 負責執行 LLM 產生的 function call。

在普通 LLM 中，模型輸出通常只是文字。但在 MemGPT 中，LLM 的 completion tokens 可以被解析成 function call。

![[Assets/Note/Research/Papers/MemGPT - Towards LLMs as Operating Systems/memgpt-archival-search-nobel-example.png|600]]
Function Executor 會解析 LLM 的輸出，檢查 function arguments 是否有效。如果有效，就執行函式。

執行結果會回傳給 LLM，放入 main context。如果出錯，例如 context 已滿、argument 格式錯、查詢沒有結果，錯誤也會回傳給 LLM，讓模型可以修正下一步。

這形成一個 feedback loop：

```text
LLM 產生 function call
-> Function Executor 執行
-> 結果或錯誤回傳 main context
-> LLM 根據回饋繼續推理
```

這就是 MemGPT 能做多步驟操作的原因。

## 8. Inference Cycle：每一輪推論怎麼跑？

> During each inference cycle, LLM processor takes main context (concatenated into a single string) as input, and generates an output string.

每一次推論週期中，LLM processor 會把 main context 當成輸入。這個 main context 會被串接成一個單一字串，然後 LLM 產生輸出字串。

完整 inference cycle 是：
1. 外部事件進來。
2. Queue Manager 把事件轉成訊息，加入 FIFO queue。
3. Queue Manager 組合 main context。
4. LLM processor 推論。
5. LLM 產生 output string。
6. MemGPT parser 檢查 output。
7. 如果是一般回答，就回覆使用者。
8. 如果是 function call，就交給 Function Executor 執行。
9. 執行結果放回 main context。
10. 如果需要，繼續下一輪 inference。

## 9. Events、Interrupts 和 Function Chaining

events trigger LLM inference。
- user message
- system message
- memory warning
- 使用者登入
- 使用者上傳文件
- 定時事件
- scheduled interrupt

這些事件會被 parser 轉成文字訊息，放進 main context，再送給 LLM。

另外，MemGPT 支援 `function chaining`。

很多任務不是一次 function call 就能完成。例如：

```text
search recall storage
-> 看搜尋結果
-> 發現結果不夠
-> 查下一頁
-> 找到相關內容
-> 放回 main context
-> 回答使用者
```

為了支援這種多步驟操作，MemGPT 使用一個特殊 flag：

```text
request_heartbeat=true
```

如果 function call 裡有這個 flag，代表函式執行完之後不要停，立刻再跑下一輪 LLM inference。

如果沒有這個 flag，就是 yield，系統暫停，等下一個外部事件再繼續。

所以：

- 有 heartbeat：function 執行後繼續推論，可以串多個 function calls。
- 沒有 heartbeat：暫停，回到等待狀態。

這對 Nested KV 很重要，因為 Nested KV 需要連續查：

```text
search(A) -> 得到 B
search(B) -> 得到 C
search(C) -> 得到 D
回答 D
```

## 10. MemGPT 和 RAG 的關係

MemGPT 的 document QA 實驗用到 embedding、cosine similarity、top-K retrieval、PostgreSQL、pgvector，所以看起來很像 RAG。

> Baseline 的 retriever-reader setup 比較像傳統 RAG。

RAG 通常是：

```text
使用者問問題
-> 外部 retriever 找 top-K documents
-> 把 top-K documents 塞進 prompt
-> LLM 根據文件回答
```

也就是：

```text
Retrieve -> Augment -> Generate
```

MemGPT 也有 retrieval，但重點不只是 retrieval，而是：

> LLM 可以自己透過 function call 控制 retrieval。

MemGPT 的流程是：

- 文件集合放在 archival storage。
- LLM 根據任務決定要不要 search。
- LLM 決定 query 是什麼。
- 搜尋結果分頁回傳 main context。
- LLM 可以決定要不要查下一頁。
- LLM 可以連續多次 function call。
- 最後回答。

普通 RAG 多半是外部系統先幫 LLM 找好資料。MemGPT 是 LLM 自己透過工具管理和搜尋外部記憶體。

## 11. 實驗一：Conversational Agents

作者第一類實驗是長期對話代理，想驗證 MemGPT 在長期聊天中是否有幫助。

作者設定兩個評估方向：

- `Consistency`
- `Engagement`

### 11.1 Consistency：對話一致性

Consistency 是指模型能不能記住過去對話中的事實、偏好、事件，並在後續對話中保持一致。

例如：

- 以前使用者說生日是 2 月 7 日，後來問生日，模型能不能答對。
- 以前 James 是男朋友，後來使用者說分手了，模型能不能更新成 James 是前男友。

為了測 consistency，作者設計了 `Deep Memory Retrieval Task, DMR`。

### 11.2 DMR(Deep Memory Retrieval) Task 怎麼做？

作者使用 `MSC dataset, Multi-Session Chat dataset`。

這個資料集本來有多 session 聊天，每組對話有兩個 persona，並且跨多個 sessions 保持一致人設。每組有 5 個 sessions，每個 session 大概十幾則訊息。

作者為了測 consistency，新增 `session 6`。

Session 6 不是普通聊天，而是一個 question-answer pair。問題必須能從前 5 個 sessions 的內容中找到答案。

例如前 5 個 sessions 有一句：

```text
James and I first met at Six Flags.
```

Session 6 可能問：

```text
Where did I first meet James?
```

Gold response 是：

```text
Six Flags
```

### 11.3 DMR 問題怎麼產生？

使用另一個 LLM 來產生 QA pairs。這個 LLM 根據前 5 個 sessions 生成問題。

- 問題明確指向過去對話。
- 答案範圍要很窄。
- 答案必須能從過去 sessions 找到。
- 不能是很主觀或開放式問題。

這樣可以測模型是否真的能回憶過去細節。

### 11.4 Baseline 和 MemGPT 怎麼比較？

Baseline 是普通 fixed-context LLM。

但為了公平，baseline 不是完全沒有過去記憶。作者給 baseline 看前 5 個 sessions 的 `lossy summarization`，也就是有損摘要。

摘要可以保留大方向，但會遺失細節。

MemGPT 則有完整對話歷史存在 recall storage。但 MemGPT 也不能一次把全部歷史塞進 context，它必須透過 paginated search queries 去 recall storage 搜尋，把相關內容取回 main context。

所以對比是：

| 系統 | 能看到什麼 |
|---|---|
| Baseline | 前 5 個 sessions 的有損摘要 |
| MemGPT | 可搜尋的完整歷史，但要透過工具分頁取回 |

這個設計是在測：

- 只靠摘要是否夠？
- 如果有完整可搜尋記憶，是否更好？

結果 Table 2 顯示，加上 MemGPT 後 accuracy 和 ROUGE-L 都明顯提升。

### 11.5 DMR 的評估指標

作者用三種評估：

- `Accuracy`
- `ROUGE-L recall`
- `LLM judge`

Accuracy 是看模型回答是否與 gold response 一致。文中提到用 LLM judge 判斷 generated response 是否和 gold response 一致，因為回答有時候不是完全同一個字串，但語意正確。

ROUGE-L recall 則是文字相似度 / 涵蓋度指標。因為模型回答通常比 gold response 更長。

例如 gold response 是：

```text
Six Flags
```

模型可能回答：

```text
You first met James at Six Flags.
```

這其實是對的，但字串不是完全一樣。所以作者用 ROUGE-L recall，看模型回答有沒有包含標準答案的重要內容。

### 11.6 Engagement：互動投入感 / 個人化程度

Engagement 是指模型能不能利用長期記憶，讓對話更自然、更個人化。

這裡不是測「答不答對」，而是測它能不能寫出比較好的 conversation opener。

普通 opener：

```text
Hi, how are you?
```

有記憶的 opener：

```text
You mentioned yesterday that you had an exam coming up. How did your preparation go?
```

第二種比較像真正記得你的助理。

### 11.7 Conversation Opener Task

這個任務讓 agent 根據過去對話，產生下一個 session 的第一句開場。

作者用 `SIM-1`、`SIM-3`、`SIM-H` 評估 opener。可以先理解成 similarity / relevance 指標，用來比較模型生成的 opener 和 persona labels 或 human-written opener 的相關程度。

Table 3 顯示，MemGPT 產生的 opener 和人類寫的 opener 接近，有些指標甚至超過 human baseline。

但要注意：

> 這不是直接測「像不像真人」，而是測 MemGPT 是否能利用長期記憶產生更符合 persona / 更具個人化的 opener。

論文也觀察到，MemGPT 的 opener 通常比較冗長，會涵蓋更多 persona 資訊。這代表它確實在使用記憶，但有時可能比人類自然對話更囉嗦。

## 12. 實驗二：Document Analysis

第二類實驗是文件分析。

作者想證明 MemGPT 不只適合聊天記憶，也能處理長文件和多文件。

文件分析會遇到 context window 限制。很多文件，例如 SEC Form 10-K，可能超過百萬 tokens。而且很多真實任務要跨多份文件建立關聯。

所以光把 context window 增加到 128k 也不一定夠。此外，Lost in the Middle 指出長 context 模型不一定能有效使用中間資訊。

因此，需要像 MemGPT 這種更彈性的 memory architecture。

### 12.1 Multi-document QA

作者使用 retriever-reader document QA task。

流程是：

```text
從 NaturalQuestions-Open 選問題
-> Retriever 根據問題找相關 Wikipedia documents
-> Reader model 根據提供文件回答問題
```

Baseline 和 MemGPT 使用相同 retriever。

Retriever 使用 OpenAI 的 `text-embedding-ada-002` 產生 embeddings，透過 cosine similarity 找 top-K documents。

MemGPT 的 archival storage 用 PostgreSQL + pgvector + HNSW index 實作向量搜尋。

實作流程是：

```text
Wikipedia passages
-> precompute embeddings
-> load into PostgreSQL
-> pgvector vector search
-> HNSW index acceleration
-> archival_storage.search()
-> paginated results
-> main context
-> LLM answer
```

### 12.2 Baseline 和 MemGPT 在 document QA 的差異

Baseline 是 fixed-context retriever-reader。它會先用 retriever 找 top-K documents，然後把這些 documents 塞進 LLM context。

問題是 context window 有限。如果 top-K documents 太多，塞不下，就必須 truncation，也就是裁切文件片段。

Documents retrieved 越多，裁切越嚴重，重要資訊越可能被截掉。因此 Figure 5 顯示 baseline 會隨著 compression / truncation 增加而 performance 下降。

MemGPT 則不同。

它把整個文件集合放進 archival storage。LLM 可以主動搜尋 archival storage，並且分頁查看結果。所以 MemGPT 不需要一次把全部 top-K documents 塞進 context。

它可以：

- 查第一頁結果。
- 如果不夠，再查第二頁。
- 需要時再繼續。

因此 MemGPT 的有效 context 不再被一次 context window 能塞幾篇文件限制。

Figure 5 的重點是：

> MemGPT 在 retrieved documents 增加時，表現比較穩定，不像 fixed-context baseline 受 truncation 影響而下降。

比較正式的說法是：

> MemGPT with GPT-4 / GPT-4 Turbo 在不同 document 數量下維持相對穩定的 accuracy。

### 12.3 Figure 6 的例子

Figure 6 問：

```text
Who won the first Nobel Prize in physics?
```

MemGPT 查：

```text
archival_storage.search("nobel physics")
```

得到很多結果，第 1 頁不夠。

再查：

```text
archival_storage.search("nobel physics", page=2)
```

在第 2 頁找到：

```text
The 1901 Nobel in physics was awarded to Wilhelm ...
```

最後回答：

```text
Wilhelm Conrad Röntgen
```

這個例子表明：MemGPT 可以透過 function call 搜尋 archival storage，並分頁取回資料。這就是 document QA 中 MemGPT 的 retrieval 方式。

## 13. Nested Key-Value Retrieval

Nested KV 不是 MemGPT 的主要應用功能，而是一個 synthetic benchmark，也就是合成驗證任務。

它用來驗證：

> MemGPT 是否真的能做 multi-hop retrieval，也就是多步驟檢索。

### 13.1 普通 KV Task 是什麼？

普通 Key-Value retrieval 是：

```text
A -> B
C -> D
E -> F
```

給模型 key A，要求模型回傳 value B。

這是單步查詢。

### 13.2 Nested KV 是什麼？

Nested KV 是 value 本身可能又是另一個 key。

例如：

```text
A -> B
B -> C
C -> D
```

問 A 的最終 value 是什麼？

答案不是 B，因為 B 還是 key，要繼續查。

整個查詢是：

```text
A -> B -> C -> D
```

最後答案是 D。

這叫 multi-hop lookup。

### 13.3 為什麼用 UUID？

UUID 沒有語意。

例如：

```text
831...ea5 -> 5b8...4c3
5b8...4c3 -> f37...617
```

這些 ID 本身沒有現實意義，不代表人名、疾病、文件標題。

作者使用 UUID 是為了防止模型靠語意或常識猜答案。如果用真實詞，模型可能靠語意推測。UUID 迫使模型只能靠查詢。

所以 UUID 的意義是：

> 排除語意干擾，純粹測模型的檢索流程控制能力。

### 13.4 Figure 8 的流程

使用者問：

```text
Find the value for key 831...ea5
```

MemGPT 第一次查：

```text
archival_storage.search("831...ea5")
```

找到：

```text
Key: 831...ea5, Value: 5b8...4c3
```

如果是普通 KV，到這裡就答 `5b8...4c3`。

但 nested KV 要確認這個 value 是否也是 key。

所以第二次查：

```text
archival_storage.search("5b8...4c3")
```

找到：

```text
Key: 5b8...4c3, Value: f37...617
```

這代表 `5b8...4c3` 也是 key，所以繼續。

第三次查：

```text
archival_storage.search("f37...617")
```

沒有找到：

```text
Key: f37...617, Value: ...
```

只找到它作為上一筆的 value 出現。

所以 MemGPT 判斷 `f37...617` 不是新的 key，它是終點。

最後答案：

```text
f37...617
```

### 13.5 Nested KV 的實驗設定

作者固定總共 140 組 UUID pairs，大約對應 8k tokens，也就是 GPT-4 baseline 的 context length。

然後改變 nesting levels：

- Level 0：查一次就結束。
- Level 1：需要繼續查一層。
- Level 2：需要更多層。
- Level 3、4：越來越多步。

作者也抽樣不同排序配置，避免 key 都放在容易找到的位置。

Figure 7 的結果大意是：

- GPT-3.5 在 nested variant 幾乎立刻失敗，常常只回傳第一層 value。
- GPT-4 和 GPT-4 Turbo 比 GPT-3.5 好，但 nesting level 增加後也會掉到 0。
- MemGPT with GPT-4 能穩定完成較高 nesting levels。
- MemGPT with GPT-4 Turbo / GPT-3.5 也比各自 baseline 好，但仍可能因為沒有查足夠多次而掉分。

這裡的重點是：

> MemGPT 的能力也依賴底層 LLM 的 tool use / function calling 能力。

架構給了它多步查詢能力，但模型本身如果不會持續查，還是會失敗。

### 13.6 Nested KV 為什麼重要？

因為 MemGPT 要證明自己不只是普通 RAG。

普通 RAG 通常是：

```text
查一次 top-K -> 塞進 prompt -> 回答
```

Nested KV 測的是：

```text
查 A -> 得到 B
判斷 B 是否還要查
查 B -> 得到 C
判斷 C 是否還要查
查 C -> 得到 D
停止
回答 D
```

所以 Nested KV 驗證的是：

> MemGPT 是否能透過 function calls 進行連續、多步驟、可中途判斷的外部記憶檢索。

這就是 MemGPT 作為 agentic memory system 的關鍵證據。它不是在強調 UUID 或 key-value 本身有現實意義，而是在用最乾淨的實驗環境測：模型會不會查到一個結果後，把那個結果當成下一次查詢目標，繼續查到終點。

## 14. Related Work：作者把 MemGPT 放在哪些研究脈絡？

論文後面把 MemGPT 放進三類研究脈絡。

### 14.1 Long-context LLMs

這類研究是想直接讓 LLM 支援更長 context。

方法包括：

- sparse attention
- low-rank approximation
- neural memory
- context extension methods

作者說 MemGPT 可以建立在這些方法之上。如果底層 LLM context 變長，MemGPT 的 main memory 也變大。

但是 MemGPT 的主要貢獻不是提出更長 context 的 LLM，而是提出：

> hierarchical tiered memory

也就是分層記憶體架構。

別人是在把 RAM 做大；MemGPT 是在設計 RAM + Disk + 管理機制。

### 14.2 Retrieval-augmented Models

這類研究就是 RAG / retriever-enhanced LLM。

作者承認 MemGPT 的 external memory 設計建立在很多 retrieval-augmented work 上。

它提到像 FLARE，讓 LLM 在生成過程中主動決定何時檢索、檢索什麼。也提到把 retrieval 和 Chain-of-Thought reasoning 交錯，用於 multi-step QA。

MemGPT 和它們相關，但 MemGPT 的定位是：

> 把 retrieval 整合成一個 LLM 可操作的記憶體階層。

### 14.3 LLMs as Agents

這類研究是讓 LLM 在互動環境中行動。

例如：

- Generative Agents
- WebGPT
- ReAct
- web shopping / games / puzzles benchmark

這些研究強調 planning、interactive environment、tool use。MemGPT 則聚焦於：

> 讓 agent 擁有長期記憶。

也就是如何讓 agent 記住使用者輸入，並在長期互動中保持一致性與可演化性。

## 15. Conclusion：作者最後的主張

MemGPT 是一個受作業系統啟發的 LLM system，用來管理 LLM 有限的 context window。

透過類似 OS 的 memory hierarchy 和 control flow，MemGPT 為 LLM 提供更大 context resource 的錯覺。

在 document analysis 中，MemGPT 可以透過 paging relevant context in and out of memory，處理超過當前 LLM context limit 的長文本。

在 conversational agents 中，MemGPT 可以維持 long-term memory、consistency 和 evolvability。

整體來說，MemGPT 展示了 OS 技術，例如 hierarchical memory management 和 interrupts，可以讓固定 context LLM 發揮更大能力。

未來方向包括：

- 應用到其他大量 / 無界 context 場景。
- 整合不同 memory tier，例如 databases、caches。
- 改進 control flow 和 memory management policies。

＝
## 16 最終重點整理

這篇論文有三個核心貢獻。

第一，提出 OS-inspired memory management。

也就是把 LLM context window 當成 main memory，把外部記憶體當成 disk，用類似 virtual memory 的方式擴展有效 context。

第二，提出 MemGPT 架構。

包含：

- main context
- external context
- working context
- FIFO queue
- recall storage
- archival storage
- Queue Manager
- Function Executor
- memory warning
- flush
- recursive summary
- function chaining

第三，用實驗驗證。

- 長期對話：證明 MemGPT 能提高 consistency 和 engagement。
- 文件分析：證明 MemGPT 能處理更長文件，不像 fixed-context baseline 受 truncation 影響。
- Nested KV：證明 MemGPT 能做 multi-hop retrieval，不只是單次 RAG。
