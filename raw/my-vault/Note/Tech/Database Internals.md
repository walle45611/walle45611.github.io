---
blog: true
blog_title: Database Internals 閱讀筆記
blog_date: '2026-10-15'
blog_url: https://blog.walle4561.com/articles/posts/database-internals-reading-notes/
blog_tag: OS & Networking
blog_topic: OS
---

## TL;DR

**儲存引擎要讓資料找得到、讀寫有效率，並在交易並行與故障後維持正確；資料結構、快取與日誌因此必須一起設計**。

- **資料組織與索引**：工作負載影響 Row／Column 配置與索引選擇；B-Tree 定位資料頁，Page／Cell 格式決定內容如何實際儲存。
- **記憶體與快取**：Buffer Pool 保留常用頁面，置換與准入策略管理有限空間；DBMS Buffer Pool、OS Page Cache 與虛擬位址轉換是不同層次。
- **寫回與恢復**：資料頁 Flush 不等於 Commit；WAL 協調日誌與資料頁的持久化順序，讓已提交修改可重做、未完成修改可依恢復設計撤銷。
- **並行交易**：隔離層級決定交易的可見性與隔離保證，OCC、PCC、MVCC 提供不同的協調方式；選擇時要考慮正確性需求、衝突與重試成本。
- **頁面保護**：交易 Lock 保護邏輯資料，Latch 短暫保護頁面與樹結構；Latch Crabbing 在確認修改不會向上傳播後，及早釋放祖先 Latches。

這份閱讀筆記從工作負載與資料配置開始，接著追蹤索引如何定位資料頁、頁面如何進入記憶體，以及修改如何在並行與故障下維持正確。B-Tree 的基本原理與設計變種，於索引段落連結到各自的延伸筆記。

---

## 工作負載與儲存方式

**OLTP 與 OLAP**：OLTP（Online Transaction Processing）處理日常交易，著重高併發（Concurrency）、低延遲（Low Latency）、短交易（Short Transactions）與 ACID。例如電商建立訂單、扣除庫存、記錄付款、查詢訂單狀態，會頻繁執行 `INSERT`、`UPDATE`、`DELETE`、`SELECT`，每筆交易通常只處理少量資料。OLAP 則著重資料分析，常涉及複雜 `SELECT`、`JOIN` 與聚合，單次查詢可能需要處理大量資料。

**TPC-C 與 TPC-H**：OLTP 是工作負載，TPC-C 則是衡量 OLTP 系統效能的 Benchmark。TPC-C 模擬批發商的訂單與庫存管理，混合執行 New-Order（建立訂單）、Payment（付款）、Order-Status（訂單狀態）、Delivery（出貨）與 Stock-Level（庫存水位）五種交易。

TPC-C 的指標為 tpmC（Transactions per Minute C）。例如 10,000 tpmC，表示在符合測試規範及其他交易負載條件下，每分鐘完成 10,000 筆 New-Order， **不是所有 SQL 操作或交易加總後的 TPS**。TPC-C 成績高，也不代表所有實際應用都比較快。OLAP 的複雜查詢與分析效能則可透過 TPC-H、TPC-DS 等 Benchmark 評估。

---

**In-Memory DBMS 與持久化**：In-Memory DBMS 主要直接存取 RAM 中的資料，但仍需要將資料持久化到 Disk，例如建立 Backup Copy、Snapshot 與保存 Log。相關背景處理可以採非同步（Async）方式，與 Client 的請求處理解耦（Decouple）。

例如 DB 在 10:00 有 1,000 筆資料，10:05 建立 Snapshot 並存入 Disk，10:06 又新增 100 筆資料，並將變更記錄到持久化日誌中。如果 DB 在 10:07 故障，重新啟動後，就先載入 10:05 的 Snapshot，再依序重播之後的 Log，恢復新增的資料。Snapshot 保存某個時間點的資料狀態，Log 記錄後續變更；Checkpoint 則與復原時從哪個進度繼續處理有關，具體方式依儲存引擎而定。

---

**Row-Oriented 與 Column-Oriented**：Row-Oriented 以資料列為單位組織資料，例如將同一個人的 `ID`、`Name`、`Phone No.` 放在一起。MySQL、PostgreSQL 常見的資料表採這類方式，適合一次取得一筆資料的多個欄位；但只查詢其中一個欄位時，讀取的 Block 可能也包含用不到的欄位，增加資料讀取量。

Column-Oriented 則把同一個欄位的值放在一起。例如 `Name Column` 為 Walle、John、Mary，`Age Column` 為 22、25、21，相同位置對應同一筆資料，可以用 `Virtual ID` 與 Array 的概念理解：`Name[0] = Walle`，`Age[0] = 22`。執行 `SELECT AVG(age) FROM USER` 時，只需要處理 `Age` 欄位，因此這類配置適合欄位掃描與聚合分析，也常用於涉及 `JOIN` 的分析工作負載。

**Wide-Column Store**：以 `Row Key` 為核心，透過 Column Family 組織大量且可能稀疏的欄位。它與 Column-Oriented 的分類重點不同：前者著重 `Row Key` 與 Column Family 的資料組織，後者著重同一欄位的值如何儲存與讀取。

---

**SSD、GC 與 FTL Remapping**：SSD 以 Page 為單位寫入，以整個 Block 為單位擦除，因此需要 GC（Garbage Collection）整理空間。例如 `P1` 存放 A、`P2` 存放 B，當 A 更新為 A′ 時，可以將 A′ 寫到新的 `P3`，原本的 A 則成為舊版本。

FTL 負責邏輯與實體位置的映射，例如 Logical Page 100 對應 Physical Block 5 的 Page 2。當資料更新或搬移，實體位置可能改變，FTL Remapping 會調整對應關係，讓 OS 不需要追蹤資料實際搬到哪裡。這裡的 SSD Page，與資料庫管理的 Database Page、虛擬記憶體使用的 OS Memory Page，屬於不同層次。

---

## 從資料組織到 B-Tree 索引

資料放在哪裡，決定索引找到 Key 後能否直接取得 Record；索引本身則需要控制樹高、管理頁面範圍，並在資料增減時維護結構。

**Index-Organized Table 與 Heap File**：Index-Organized Table 將實際 Record 放在索引的 Leaf Node，沿著 B+ Tree 找到 Leaf Node 就能取得資料。Heap File 則將資料與索引分開，索引的 Leaf Node 保存 Record Location，再透過這個位置找到 Data Block 中的 Record。

**Clustered Index 與 Non-Clustered Index**：Clustered Index 依索引順序組織資料本身，一個 Table 最多有一個。Non-Clustered Index 則額外維護查找結構，透過索引定位資料，必要時回表取得完整資料。

例如 `Users` 依 `ID` 排列為 Amy（`ID` 1）、Walle（`ID` 2）、John（`ID` 3），查詢 `ID BETWEEN 1 AND 3` 時，可以沿 `ID` 順序取得資料。若再建立依 `Name` 排序的索引，順序就變成 Amy、John、Walle，索引項目分別指向 `ID` 1、3、2，提供另一條查找路徑。

**Secondary Index（次級索引）**：讓查詢能使用 Primary Key 以外的欄位，例如透過 `Email` 找 `User`。依儲存方式不同，Secondary Index 可能直接保存資料列位置，也可能保存 Primary Key，先取得 Key，再查 Primary Index 才能拿到完整資料。這也說明了為什麼透過索引找到項目後，有時仍需要額外回表。

**B+ Tree 與 LSM-Tree**：B+ Tree 使用有序的結構，常見實作傾向於原地更新 Page；LSM-Tree 則先利用記憶體 Buffer 接受寫入，再透過不可變（Immutable）的磁碟檔案與 Compaction 整理資料。Append-Only、Copy-on-Write 都涉及把新版本寫到新位置的思路，但具體更新方式仍取決於儲存引擎。B+ Tree 的高 Fanout 可以降低樹高，減少查找需要經過的節點，進而降低可能發生的 Disk I/O。

LSM-Tree 的 MemTable、SSTable、Flush 與 Compaction 流程，接續整理於 [LSM-Tree](https://blog.walle4561.com/articles/posts/lsm-tree/)。

### LSS：寫入方式與 Key 排序是不同面向

Log-Structured Storage（LSS）將新紀錄或新版本追加到新的儲存位置，再以索引與後續清理管理它們。它描述的是儲存與寫入方式，不等同於某一種搜尋樹，也不代表一定不按 Key 排序；LSM 的 Sorted Runs，以及部分 B-Tree 引擎採用的底層儲存設計，都可以運用 Log-Structured 的思路。

#### In-Place Update 與 Out-of-Place Update

In-Place Update 指更新既有位置的內容，例如 B+ Tree 修改某個 Database Page 後，將它寫回原本的檔案位置。Out-of-Place Update 則把新版本寫到其他位置，再更新指向它的資訊；Copy-on-Write B+ Tree 就是一個例子：複製受影響的 Page 與必要的祖先路徑，發布新的 Root，舊頁面等不再被 Snapshot 使用後才回收。具體流程見 [B-Tree Variants 的 Copy-on-Write B-Tree](https://blog.walle4561.com/articles/posts/b-tree-variants/#copy-on-write-b-tree)。

Out-of-Place Update 是較廣的更新方式，CoW 是其中一種實作；LSS 也採用寫到新位置、之後回收舊內容的思路，但 CoW B+ Tree 不因此等同於 LSM-Tree。

#### LSS、LSM-Tree 與 SSD：從小筆更新到批次寫入

| 層次 | 負責的事情 |
| --- | --- |
| LSS：儲存設計方向 | 將更新寫到新位置，再清理不再需要的舊內容。 |
| LSM-Tree：儲存引擎的具體結構 | 在 MemTable 累積更新，Flush 成有序 SSTable，再透過 Compaction 整理。 |
| SSD：底下的儲存裝置 | 由控制器管理 NAND Page 的寫入、位置映射與 Block 回收。 |

例如應用程式不斷更新不同 Key，每筆只有幾十 Bytes，LSM-Tree 可以先將這些小筆更新累積在 MemTable，再批次寫出較大的 SSTable；尚未 Flush 的更新則由 WAL 協助故障復原。Key 到達的順序可以很亂，緩衝與排序仍能把它們轉為較大的批次寫入。[RocksDB 官方概述](https://github.com/facebook/rocksdb/wiki/RocksDB-Overview)

SSD 的 NAND 以 Page 為單位寫入，但必須以整個 Block 為單位抹除。更新通常寫到新的 Page，再由 FTL 更新邏輯位址對應，舊 Page 便成為無效資料。下圖的 Live Page 仍保存有效資料，Discarded Page 已失效但尚未抹除，Empty Page 則表示可供寫入的空間。

![[Assets/Note/Tech/Database Internals/ssd-live-discarded-empty-pages.png]]

GC 回收 Block 時，會先把其中仍有效的 Pages 搬到其他有可寫空間的 Block，更新 FTL 映射，再整塊抹除舊 Block，讓它重新供後續寫入使用。下圖的箭頭表示有效資料搬移；無效 Pages 不必複製，也無法單獨抹除。[OSTEP：Flash-Based SSDs](https://pages.cs.wisc.edu/~remzi/OSTEP/file-ssd.pdf)

![[Assets/Note/Tech/Database Internals/ssd-gc-live-page-relocation.png]]

例如一個 Block 有 8 個 Pages，其中 6 個已失效、2 個仍有效，GC 只需搬走這 2 個有效 Pages，再抹除整個 Block。這也說明回收成本與需要搬移多少有效資料有關。

資料庫的 In-Place Update 與 SSD 的實體更新屬於不同層次：即使 B+ Tree 寫回同一個檔案位置，SSD 控制器仍可能把內容寫到新的 NAND Page，再調整 FTL 映射。LSM 的批次寫入也不保證剛好對齊實體 NAND Block；Compaction 與 SSD GC 都可能重寫資料，因此實際效能與耗損仍取決於工作負載、整理策略與 SSD。

Unordered 則描述磁碟內容不維持 Key 順序。Bitcask 使用 KeyDir 定位追加式資料檔，WiscKey 則保留有序的 Key LSM、把 Value 分離到無序 Value Log。查詢、更新與回收方式另見 [Unordered Log-Structured Storage：Bitcask 與 WiscKey](https://blog.walle4561.com/articles/posts/unordered-log-structured-storage/)。

#### 上下層不一致：儲存單位與寫入安排的落差

資料庫、檔案系統與 SSD 各自管理寫入、放置與回收，但各層使用的單位大小、邊界與整理時機不一定一致，因此可能增加 I/O 與資料搬移。

1. 儲存與回收邊界不對齊：上層的 Segment 可能跨越多個下層回收單位。上層丟棄某個 Segment 後，下層單位中仍可能混有其他有效資料，必須先搬走它們，才能回收整個單位。即使單位大小相同，起始位置不同也可能造成這種情況。
2. 多條寫入流交錯：Data Records 與 Log Records 即使各自循序寫入，到了下層仍可能交錯；記錄大小與 Page 邊界的落差也可能造成碎片。上層的循序寫入，不保證在下層保有相同的連續性。
3. 各層重複整理：下層可能剛搬移某份資料，上層就將它合併或丟棄。如果各層不了解彼此的安排，就可能做出很快失去用途的搬移。

#### Mindful Stacking：透過 API 協調上下層

Mindful Stacking 保留分層設計，但讓軟體層之間的 API 傳遞足夠的結構與生命週期資訊，協調資料放置、整合與回收。一般讀寫介面主要表達「讀寫這個位置的 Bytes」；以邏輯頁面為單位的介面，則能表達「這份 Delta 是哪個 Page 的更新」及「這份完整新頁面取代哪個舊狀態」。

Bw-tree 與 LLAMA 的協作是具體例子：Bw-tree 管理 Key、樹結構與 Delta 的語意；LLAMA 提供快取、位置映射與 Log-Structured 儲存管理。其 API 包含下列操作，表中僅列核心參數與用途：

| API | 傳遞的資訊與用途 |
| --- | --- |
| `Update-D(PID, …)` | 提供邏輯頁面 ID、先前狀態的指標與新 Delta，把更新接到該頁面的狀態鏈前端。 |
| `Update-R(PID, …)` | 提供整合後的完整頁面，取代該 PID 的既有狀態；Bw-tree 可用它發布 Consolidation 的結果。 |
| `Read(PID, …)` | 依邏輯頁面 ID 取得資料，由 LLAMA 的 Mapping Table 處理目前位置。 |

例如邏輯頁面 `PID 42` 的狀態是 `Delta 2 → Delta 1 → Base Page`。LLAMA 能追蹤它們屬於同一個邏輯頁面，並在儲存整理時把相關部分集中；Bw-tree 則理解這些 Delta 如何改變 Key-Value 內容，能把它們套用到新的 Base Page，再以 `Update-R` 替換舊狀態。這種分工讓實體位置管理與語意整合可以協作，減少碎片與重複工作。[LLAMA 原論文](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/llama-vldb2013.pdf)

這裡的 API 連接 Bw-tree 與 LLAMA，並不直接控制 SSD 的實體 NAND 配置。Mindful Stacking 是跨層協作的設計原則，沒有一個通用的「Mindful Stacking API」；LLAMA 也不是所有跨層對齊問題的通用解法。每層各自最佳化，仍需要檢視整體是否重複搬移與整理。

### 儲存結構比較：Buffered、Mutable 與 Ordered

Buffered 指先緩衝更新再批次處理；Mutable 指磁碟頁面／既有儲存內容是否採就地更新；Ordered 指資料在儲存配置中是否依 Key 排序，不代表只能使用有序的查詢 API。這些欄位描述不同面向：LSM 的 MemTable 可以修改，表中的 Mutable 仍為 No，因為它描述的是不可變的磁碟資料組織。

| 儲存結構 | Buffered | Mutable | Ordered |
| --- | --- | --- | --- |
| B+ Trees | No | Yes | Yes |
| WiredTiger | Yes | Yes | Yes |
| LA-Trees | Yes | Yes | Yes |
| CoW B-Trees | No | No | Yes |
| Two-component LSM Trees | Yes | No | Yes |
| Multicomponent LSM Trees | Yes | No | Yes |
| FD-Trees | Yes | No | Yes |
| [Bitcask](https://blog.walle4561.com/articles/posts/unordered-log-structured-storage/) | No | No | No |
| [WiscKey](https://blog.walle4561.com/articles/posts/unordered-log-structured-storage/) | Yes | No | Yes |
| Bw-Trees | No | No | No |

B-Tree 類型與緩衝設計的細節見 [B-Tree Variants](https://blog.walle4561.com/articles/posts/b-tree-variants/)；兩種 LSM 模型與 Compaction 流程則見 [LSM-Tree](https://blog.walle4561.com/articles/posts/lsm-tree/)。

---

### 樹的基本規則與儲存引擎的實作

B-Tree 用高 Fanout 降低樹高，節點通常以 Page 儲存。分隔 Key、子指標、Split／Merge、Rebalancing、Breadcrumbs 與 Bottom-Up 建樹的基本原理，見 [多路搜尋樹與 B-Tree](https://blog.walle4561.com/articles/posts/m-way-search-tree-and-b-tree/)；以下保留儲存引擎如何利用這些規則優化存取、安排頁面格式的細節。

B-link Tree 的 High Key 與橫向連結，以及 CoW／LMDB、WiredTiger、LA-Tree、FD-Tree、Bw-Tree 和 Cache-Oblivious 的取捨，見 [B-Tree Variants](https://blog.walle4561.com/articles/posts/b-tree-variants/)。

**Right-Only Appends**：當索引 Key 單調遞增，例如自動遞增的 `ID`，新增資料就會持續落在最右側的 Leaf Page。這讓儲存引擎可以針對追加寫入優化：PostgreSQL 的 Fastpath 著重省掉重複搜尋 Leaf 的路徑，SQLite 的 Quickbalance 則著重最右側 Page 滿了以後，如何減少重新分配資料的工作。

**PostgreSQL Fastpath**：記住先前使用的最右側 Leaf Page，下一次插入時先檢查這一頁是否仍符合條件、是否有足夠空間。符合條件就能直接在該頁內找到適當位置並插入，省掉從 Root 重新往下搜尋的過程；不符合條件則回到一般插入流程。這不是 First Fit：First Fit 通常是依序找第一個放得下的空間，Fastpath 則是先嘗試已知的最右側 Page。它優化的是尋找目標頁面的成本，並不代表忽略頁內排序或節點容量限制。

**SQLite Quickbalance**：在符合適用條件、插入位置位於最右端且目標 Leaf Page 已滿時，可以保留原頁面的資料，另外建立一個新的最右側 Leaf Page，先放入新增的最大 Key，再更新父節點的分隔 Key 與 Child Pointer。它不要求這次把新舊兩頁的資料平均分配，因為預期後續遞增 Key 還會繼續填入新頁。這是特定追加情境的優化，不能套用到所有位置的插入。

例如簡化假設每個 Leaf Page 最多容納 4 筆資料，原本左頁存放 `1、2、3、4`，最右頁存放 `5、6、7、8`。現在新增 `9`，最右頁已經沒有空間；Quickbalance 的做法是保留這兩頁，建立新的最右頁，先只存放 `9`，並讓父節點加入相應的導航資訊。之後再插入 `10、11、12`，新頁就逐步填成 `9、10、11、12`，不必為了這次插入，先把舊頁的 `5、6、7、8` 搬過去平均分配。實際頁面容量取決於 Cell 大小，這裡的 4 筆只是方便理解的假設。

**填滿程度不平均，不等於樹的高度失衡**。上述例子假設父節點仍有空間，因此新增 Leaf Page 後，各 Leaf 的深度相同，樹高不變；若父節點也放不下，仍需要進一步處理，必要時向上傳遞。Quickbalance 只是接受新頁暫時較空，以減少當次搬移資料的成本，並沒有放棄 B-Tree 的結構規則。

前述基本樹操作中的 Rebalancing，與這裡 Quickbalance 的取捨也不同：前者先利用兄弟 Page 的剩餘空間，盡量延後 Split；後者則利用「後續資料很可能繼續往右新增」的特性，保留舊頁面滿載，讓新頁承接後續寫入。Fastpath 解決的是如何更快找到可插入的頁面，Quickbalance 解決的則是特定情況下頁面已滿時如何安排資料，兩者不是完全相同的機制。

---

搜尋時可把下降路徑記在暫存 Stack，供 Split 後向上更新父節點；Breadcrumbs 與 Parent Pointer 的基本差異見 [多路搜尋樹與 B-Tree](https://blog.walle4561.com/articles/posts/m-way-search-tree-and-b-tree/)。

**PostgreSQL 的 BTStack**：PostgreSQL 的 B-Tree 使用 `BTStack` 保存下降路徑，底層由 `BTStackData` 串接成 Linked List，作為回溯用的 Stack。`bts_blkno` 記錄 Parent Page 的 Block Number，`bts_offset` 記錄該頁所選 Downlink 的位置，`bts_parent` 則指向更上一層的 Stack 紀錄。這個 `bts_parent` 屬於暫存的路徑結構，並不是持久化在 Child Page 內的 Parent Pointer。

`_bt_search()` 透過 `returnstack` 決定是否建立並回傳 Stack；呼叫端不需要向上回溯時，可以省下這份配置，因此不是每次搜尋都會建立 `BTStack`。並行的 Page Split 或 Page Deletion 可能使紀錄的位置過期，使用時仍須重新確認；Breadcrumbs 提供定位線索，不保證樹的結構一直不變。


---

**Rightmost Pointer 的實體配置**：頁面內可以把每個 Key 與左側 Child Pointer 放進同一個 Cell，再把多出來的最右側 Pointer 獨立存放在 Header 中。

SQLite 就採用這種將 Rightmost Pointer 獨立存放的配置：Interior B-Tree Page 的 Header 在 Offset 8 保存一個 4 Bytes 的最右側子頁面編號，Leaf Page 則沒有這個欄位。不過相等值的路由慣例要分開看：SQLite 官方描述的是左側子樹包含小於等於分隔 Key 的值，右側子樹包含大於分隔 Key 的值，因此判斷邊界相等值時，須依 SQLite 的路由規則，不能直接套用其他 B+ Tree 範例的不等號。

### 從邏輯節點到 Page 與 Cell

樹上的 Key 與 Pointer，最後都要編碼成 Page 裡的位元組。Slotted Page 用 Slot 間接定位 Cell；變長 Record 則讓容量判斷與 Overflow Pages 成為必要的頁面管理工作。

![[Assets/Note/Tech/Database Internals/b-tree-pages-and-cell-layout.png]]

**Magic Number**：用來確認這是什麼格式的檔案。

**Slotted Page**：Page 由 Header、Slot Directory、可用空間與 Cells 組成。Slot Directory 中的每個 Slot 儲存 Offset，指向 Page 內對應的 Cell。圖中的 Pointers 就是這些 Slot；虛線表示它們與 Cells 的對應關係，Slot 的排列順序不必與 Cells 的實際存放順序相同。

![[Assets/Note/Tech/Database Internals/slotted-page.png]]

透過 Slot 間接定位資料，Page 內整理空間、移動 Cell 時，可以更新對應的 Offset，不必讓 Slot 跟著 Cell 一起移動。可用空間與 Cells 的位置由 Page 的配置管理。

**Key Cell**：用於 Non-Leaf Node，包含 `Key Size`（4 Bytes）、`Page ID`（4 Bytes）與 Key（N Bytes）。圖上方的 0、4、8 表示各欄位相對於 Cell 開頭的 Byte Offset；Key 從 Offset 8 開始，長度由 `Key Size` 決定。`Page ID` 用來定位對應的子頁面。

![[Assets/Note/Tech/Database Internals/key-cell.png]]

**Key-Value Cell**：用於 Leaf Node，包含 `Flags`（1 Byte）、`Key Size`（4 Bytes）、`Value Size`（4 Bytes）、Key（N Bytes）與 `Data Record`（M Bytes）。`Flags` 從 Offset 0 開始，`Key Size` 從 1 開始，`Value Size` 從 5 開始，Key 從 9 開始，`Data Record` 則從 `9 + Key Size` 開始。圖中分成兩行是為了排版，仍表示同一個 Cell 內連續的欄位配置。

![[Assets/Note/Tech/Database Internals/key-value-cell.png]]

---

**Overflow Pages**：B-Tree 的 Node 通常以固定大小的 Page 儲存，但每個 Record 的 Payload 可能是變長的（Variable-Size Payload）。因此， **Key 數量尚未達到概念上的上限，不代表 Page 還有足夠的 Free Space**；少數幾筆很大的 Record，就可能先把 Page 的位元組空間用完。實際上，對變長資料而言，Node 能容納多少 Key 也會受到每個 Cell 大小的影響，不能只看 Key 的數量。

當單筆 Record 太大，可以將部分 Payload 保留在 Primary Page，超出的部分放到 Overflow Page，再由原本的 Cell 透過 Pointer 找到剩餘內容。這樣就不必單純因為一筆 Record 太大，而把它全部擠在同一個 B-Tree Node 裡；即使進行 Node Split，也不一定能解決單筆 Record 本身大於 Page 可容納空間的問題。

![[Assets/Note/Tech/Database Internals/overflow-pages.png]]

圖中的 Payload1、Payload2 都有一部分留在 Primary Page，其餘部分延伸到 Overflow Page。若額外內容仍放不下，多個 Overflow Pages 可以透過 `Page ID` 串接成 Linked List，每頁記錄下一個 Overflow Page 的位置，讀取完整 Record 時再沿連結取得後續 Payload。

`max_payload_size` 可以用來表示單筆 Payload 允許直接存放在 B-Tree Node 內的大小上限，超出這個限制的部分改存到 Overflow Page。實際門檻、欄位名稱與保留在原頁面的大小，仍依儲存引擎的實作而定。

**Overflow Page 與 Node Split 的用途不同**：Overflow Page 延伸的是單筆 Record 的儲存空間，不是 B-Tree 的 Child Node，也不會直接增加樹高；Node Split 則是把索引節點的內容分配到兩個節點，並更新父節點的分隔 Key 與 Child Pointer。把大型 Payload 移到 Overflow Page 可以減輕原頁面的空間壓力，但若原頁面仍放不下新的 Cell 或索引項目，依然可能需要 Split。

**Overflow Pages 的讀取成本與 BLOB Storage**：Overflow Pages 解決的是 Variable-Size Payload 超過 Primary Page 可用空間的問題，但完整讀取大型 Record 時，需要額外存取 Overflow Pages，並沿 `Page ID` 逐頁追蹤（Pointer Traversal）。若相關頁面尚未快取在記憶體中，就可能增加 Disk I/O；即使已有快取，仍有走訪連結的處理成本。

當大量 Records 都具有大型 Payload 時，可以考慮專門的 BLOB（Binary Large Object）Storage，讓大型內容由適合的儲存機制管理，索引或主要 Record 則保留對應的識別資訊。是否採用獨立的 BLOB Storage，仍取決於資料庫本身的支援方式與讀寫需求。

---

## 資料頁如何進入記憶體

找到 B-Tree 的目標頁面後，下一步是把檔案內容載入記憶體，再讓 CPU 存取頁內資料。這一段依序連起 Database Page、OS Page Cache、虛擬位址，以及 DBMS 自行管理的 Buffer Pool。

**Database Page 與 OS Page Cache**：資料庫從 B+ Tree 取得 `Page ID` 後，需要先定位檔案中的位置，再把需要的資料讀進記憶體。以 Linux 使用一般 buffered `pread()` 的情況為例，OS 會先檢查 Page Cache，已有快取就能使用；沒有快取時，通常需要從 Disk 讀入，再將資料複製到資料庫的 `User`-Space Buffer。

假設 Database Page 為 8 KiB（8192 Bytes），OS Memory Page 為 4 KiB（4096 Bytes），且 `Page ID` 從 0 開始、資料頁在檔案中連續排列。讀取 Database Page 5 時，File Offset 為 `5 × 8192 = 40960 Bytes`，也就是 `0xA000`。這個 8 KiB 的資料頁涵蓋 OS File Page 10、11，檔案範圍分別為 `0xA000`–`0xAFFF`、`0xB000`–`0xBFFF`。 **一個 Database Page 可以涵蓋多個 OS Pages，載入後對應的實體記憶體也不必連續**。

**Virtual Memory、TLB 與 Page Table**：資料庫同樣使用虛擬記憶體。Page Cache 查找的是檔案內容是否已在 RAM，TLB 則快取 Virtual Page 到 Physical Frame 的位址映射。兩者雖然都涉及 Page，處理的問題不同；CPU 在核心態存取 Page Cache、複製資料，以及在使用者態存取 Database Buffer 時，都涉及虛擬位址轉換。

延續上例，假設 Database Buffer 的起始虛擬位址為 `0x7F0010000000`，要讀取 Buffer 內偏移 4660 Bytes 的欄位。4660 的十六進位是 `0x1234`，因此該欄位的 Virtual Address 為 `0x7F0010001234`。這個 Offset 是示範值，並不是 B+ Tree 固定的欄位位置。

OS Page Size 為 4096 Bytes，也就是 2¹² Bytes，因此位址低 12 bits 是 Page Offset。`4660 = 4096 + 564`，表示資料位於 Buffer 的第二個 OS Page，距離該 Page 開頭 564 Bytes（`0x234`）；Virtual Page Number 則為 `0x7F0010001`。

CPU 使用這個 Virtual Page Number 查 TLB。若 TLB Hit，就直接取得 Physical Frame；若 TLB Miss，則需要查 Page Table。假設查到的 Physical Frame 起始位址為 `0x009A7000`，加上 Page Offset `0x234`，得到 Physical Address `0x009A7234`。有效映射可以快取到 TLB，供後續存取使用。

**Page Cache Miss、TLB Miss 與 Page Fault**：Page Cache Miss 表示檔案內容沒有快取，通常需要讀取 Disk；TLB Miss 表示沒有快取位址映射，需要查 Page Table；Page Fault 則表示目前頁表映射無法直接滿足存取，需要 OS 處理。 **TLB Miss 不等於 Page Fault，也不等於 Page Cache Miss，更不代表一定需要 Disk I/O**。例如 `mmap()` 的檔案頁若已在 Page Cache，可能只需要 Minor Page Fault 建立映射；若還必須從儲存裝置讀入，則可能涉及 Major Page Fault。

資料庫層處理 `Page ID` 到 File Offset，Page Cache 處理檔案內容的快取，Virtual Memory 則處理 Virtual Page 到 Physical Frame。把這三層分開，才能理解從 B+ Tree 找到 Page，到 CPU 實際存取資料之間的流程。

### DBMS 的 Buffer Pool 與頁面生命週期

**Buffer Pool 與 Page Cache**：資料庫不會每次存取 B-Tree Page 都重新讀取 Disk，而是把部分頁面保留在記憶體中。Buffer Pool 是 DBMS 管理資料頁快取的空間；書中也可能以 Page Cache 描述這類功能，但必須與前面提到的 **OS Page Cache** 區分，後者由 Kernel 管理。使用一般 Buffered I/O 時，同一份檔案內容可能同時存在於 OS Page Cache 與 DBMS Buffer Pool，形成兩層快取。

這層記憶體快取讓上層仍以磁碟頁面為單位存取：命中時直接使用記憶體內容，未命中才載入實體儲存中的頁面。書中將這個抽象稱為 **Virtual Disk**，也就是這裡的 Page Cache／Buffer Pool。它管理的是資料頁的快取；前面 Virtual Memory 管理的則是虛擬位址到實體記憶體的映射。DBMS 可以自行管理 Buffer Pool，不必透過 `mmap()`。

![[Assets/Note/Tech/Database Internals/buffer-pool-and-disk.png]]

圖中的 B-Tree Pages 對應到 Disk 上的頁面，Buffer Pool 則只容納其中一部分。虛線表示頁面與快取位置之間的對應；讀取某個節點時，若頁面已在 Buffer Pool，就能直接使用，否則才需要載入。

**Paged In、Dirty、Flushed 與 Evicted**：將需要的頁面從 Disk 載入記憶體，稱為 Paged In。當記憶體中的頁面被修改，而變更尚未寫回對應的儲存位置時，會標記為 **Dirty Page**。將變更寫回的動作稱為 Flush；若寫回期間沒有新的修改，頁面才可以視為 Clean。

Flush 是寫回，Eviction 是移出快取。Flush 後的頁面仍可留在 Buffer Pool，供後續存取；當 Buffer Pool 需要騰出空間，才由 Replacement Policy 選擇可移除的頁面，將其 Evict，重用該 Buffer Frame。Clean Page 通常可以直接移除，Dirty Page 則必須先妥善保存修改，不能直接丟棄；寫回也可以由背景程序提前進行，不必等到 Buffer Pool 完全滿了才開始。

例如某個 Leaf Page 載入後被修改，就會變成 Dirty；背景程序可以先把它 Flush，再繼續留在快取中。之後需要空間時，才將這個已經 Clean 的頁面 Evict。因此，Dirty、Flush 與 Eviction 描述的是不同狀態與動作，不是一發生修改就立刻寫回並移除。

**`O_DIRECT` 與自行管理快取**：部分 DBMS 在 Linux 上開啟檔案時使用 `O_DIRECT`，讓資料 I/O 通常繞過 OS Page Cache，直接在儲存裝置與 User-Space Buffer 之間傳輸，減少重複快取，並讓 DBMS 自行安排頁面的快取與讀寫。這並不是繞過整個 Kernel；實際支援與記憶體位址、I/O 長度、File Offset 的對齊要求，仍依檔案系統與裝置而定。

**`O_DIRECT` 本身不代表 Async I/O，也不單獨保證資料已持久化**。是否非同步取決於使用的 I/O 介面；是否達到持久化要求，則還需要考慮同步與儲存裝置的語意。它的取捨是把更多快取與預讀工作交給 DBMS，也增加實作與調校的複雜度；不能直接歸納為所有情境都比較快。參考 [Linux `open(2)`：`O_DIRECT`](https://man7.org/linux/man-pages/man2/open.2.html)。

**Read-Ahead 與 `posix_fadvise()`**：DBMS 可能比 Kernel 更清楚接下來會做順序掃描、隨機查找，或讀取哪些 Pages。使用 Buffered I/O 時，可以透過 `posix_fadvise()` 提供存取模式提示，例如 `POSIX_FADV_SEQUENTIAL`、`POSIX_FADV_RANDOM`；`POSIX_FADV_WILLNEED` 可提示近期將使用的範圍，`POSIX_FADV_DONTNEED` 則可請求釋放不再需要的快取。

這些是給 Kernel 的 **Advice（提示）**，不是完全接管 OS Page Cache 的 Replacement Policy，也不保證指定頁面立即被 Evict。`O_DIRECT` 路徑不依賴一般 OS Page Cache 的預讀，DBMS 仍可自行安排預讀與非同步請求。參考 [Linux `posix_fadvise(2)`](https://man7.org/linux/man-pages/man2/posix_fadvise.2.html)。

**Memory Mapping（記憶體映射）**：`mmap()` 可以把檔案映射到 Virtual Address Space，讓程式透過記憶體存取方式讀取內容；一般檔案映射的載入、Page Fault 與快取回收，主要由 OS 的虛擬記憶體與 Page Cache 機制處理。它能省去部分顯式讀取與複製工作，但 DBMS 對載入、寫回與回收時機的直接控制較少。仍可透過相關 Advice 介面提供提示，因此不是完全沒有控制方式。這是另一種存取設計，不代表應把同一檔案範圍的 `mmap()` 與 `O_DIRECT` 混用。參考 [Linux `mmap(2)`](https://man7.org/linux/man-pages/man2/mmap.2.html)。

**Page Replacement 與 Locking Pages in Cache**：Buffer Pool 容量有限，Replacement Policy 要決定哪些頁面繼續保留、哪些可以移除。比較直接的做法，是讓少量高頻使用的頁面常駐快取，避免反覆讀取 Disk。B-Tree 的 Root 與靠近 Root 的 Internal Pages 通常會被許多查找共用，因此可能比單一 Leaf Page 更值得保留，減少重複 I/O。

這裡的 Locking Pages in Cache 是指避免頁面被置換，不是交易處理中用來控制並行存取的 Lock。實作時也要區分 **Pin／Unpin**：Pin 通常保護目前正在使用的 Buffer Frame，使用完後 Unpin，才有機會成為置換候選；長期保留熱門頁面則是另一項快取策略。也不能直接把 DBMS 的 Pin 等同於 OS 的 `mlock()`，兩者管理的層次不同。

---

### 快取空間有限：頁面置換與准入

**FIFO、LRU 與 Belady’s Anomaly**：FIFO（First-In, First-Out）依照頁面進入快取的先後順序淘汰，Hit 不會改變排隊順序；LRU（Least Recently Used）則淘汰最久沒有被存取的頁面，Hit 時也會更新最近使用順序。LRU 關心的是「最近何時使用」，不是單純累計使用次數。

**Belady’s Anomaly（Belady 異常現象）** 是指在相同參考序列與置換規則下，增加可容納頁面的 Frame 數量，Miss／Page Fault 反而變多。這裡增加的是 **Page Frames 的數量，不是 Page Size**。例如從空快取開始，依序存取 `1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5`，FIFO 使用 3 個 Frames 時有 9 次 Miss，增加到 4 個 Frames，反而有 10 次。

這裡要區分 OS 與 DBMS 的用語：作業系統教材以 Page Fault 計算缺頁；DBMS Buffer Pool 找不到資料頁時，較精確的說法是 **Buffer Pool Miss**，不代表 CPU 一定發生 OS Page Fault。以下置換範例都以 Hit／Miss 表示是否在所討論的快取中找到頁面。

**Stack Property（堆疊性質）**：對相同參考序列，若容量為 N 的快取內容始終包含於容量 N+1 的快取內容中，增加容量就不會增加 Miss。精確 LRU 具有這個性質；OPT（Optimal Page Replacement）則利用未來資訊，淘汰下一次最晚才會使用的頁面，最優 Miss 數也不會因容量增加而變多。FIFO 不具備這項保證，因此可能出現 Belady’s Anomaly。OPT 需要預知未來，主要用作比較基準。

**LRU 的 Cache Pollution 與 2Q**：LRU 能保留近期使用的頁面，但一次性的大量順序掃描，也可能把原本常用的頁面擠出快取，形成 Cache Pollution（快取污染）或 Sequential Flooding。2Q（Two-Queue Algorithm）把新進頁面與已確認會再次使用的頁面分開管理，避免每個只讀一次的頁面都立即取得主要快取的地位。 **2Q 是另一種置換策略，不是精確 LRU 的兩個 Queue 實作，也不能直接沿用 LRU 的 Stack Property 保證**。

**Simplified 2Q 與 Full 2Q**：簡化版使用 FIFO 的 `A1` 與 LRU 的 `Am`，頁面在 `A1` 期間再次被存取，就可升到 `Am`。原論文的 Full 2Q 則拆成 `A1in`、`A1out` 與 `Am`：`A1in` 以 FIFO 保存新進頁面；`A1out` 是 Ghost Queue，只保留已從 `A1in` 淘汰的 Page ID；`Am` 以 LRU 保存通過再次存取判斷的實際頁面。Ghost Queue 仍有少量識別資訊的記憶體成本，但不占用存放完整資料頁的 Frames。

Full 2Q 命中 `A1in` 時，保留 FIFO 順序，不直接升到 `Am`，避免把同一波短時間內的重複存取誤認為長期熱門。若頁面已離開 `A1in`，但 Page ID 仍在 `A1out`，再次存取時才重新載入到 `Am`；此時雖然命中 Ghost 紀錄， **仍是資料頁的 Miss**。命中 `Am` 時，才依 LRU 規則更新到 MRU（Most Recently Used）端。若 Ghost 紀錄也已消失，就當成新進頁面處理。

**Full 2Q 的空間回收規則**：有空閒 Frame 時先使用，不會只因 `A1in` 達到某個固定格數，就讓其他空間閒置。整個 Buffer 已滿時，原論文依 `A1in` 的大小與 `Kin` 門檻決定回收來源：`A1in` 大於門檻時，淘汰其 FIFO 最舊頁並把 ID 放入 `A1out`；否則淘汰 `Am` 的 LRU 頁，這個頁面不加入 `A1out`。Ghost 紀錄超過 `Kout` 時，刪除最舊 ID。因此，`Kin` 是選擇回收來源的門檻，不能直接當成不可借用的固定分區容量。參考 [Johnson 與 Shasha：2Q 原論文，Section 2](https://www.cs.cmu.edu/~natassa/courses/15-721/papers/P439.PDF)。

**2Q 演算範例：固定分區教學變體**：以下範例使用 4 個 Frames，將 `A1in` 與 `Am` 各限制為 2 個資料頁，`A1out` 最多記錄 2 個 ID。這個假設可示範 FIFO、Ghost 與 LRU 的互動，卻不是原論文共用空閒 Frames 的完整回收規則。以下保留這個固定分區假設；所有 Queue 都以左邊最舊、右邊最新表示，存取序列為 `A, B, A, C, A, D, B, E, C, B`。

前兩步載入 A、B，`A1in = [A, B]`，皆為 Miss。第三步再次讀取 A，為 Hit，`A1in` 的 FIFO 順序不變。第四步讀取 C，因固定的 `A1in` 分區已滿，移除 A 並記錄其 ID，得到 `A1in = [B, C]`、`A1out = [A]`；即使 `Am` 尚有空間，這個教學變體仍先淘汰 A。

第五步讀取 A，只在 Ghost Queue 找到 ID，因此仍為 Miss；重新載入後，`A1in = [B, C]`、`A1out = []`、`Am = [A]`。第六步讀取 D，從 `A1in` 淘汰 B，得到 `A1in = [C, D]`、`A1out = [B]`。第七步讀取 B，再次透過 Ghost 紀錄載入 `Am`，得到 `Am = [A, B]`，仍為 Miss。

第八步讀取 E，淘汰 `A1in` 的 C，得到 `A1in = [D, E]`、`A1out = [C]`、`Am = [A, B]`。第九步讀取 C，Ghost 命中但資料仍需載入；因固定的 `Am` 分區已滿，淘汰其中最久未使用的 A，得到 `Am = [B, C]`、`A1out = []`。第十步讀取 B，為 Hit，更新 LRU 順序成為 `Am = [C, B]`。

這個固定分區變體共有 **8 次 Miss、2 次 Hit，Hit Ratio 為 20%**；同一序列採用 4 Frames 的精確 LRU，則是 **6 次 Miss、4 次 Hit**。但若使用前述原論文的回收規則，設定 Buffer 容量 4、`Kin = 2`、`Kout = 2`，前四種不同頁面 A、B、C、D 可以先使用空閒 Frames；讀取 E 才淘汰 A，後續 C、B 都仍命中，整段為 **5 次 Miss、5 次 Hit**。三者結果反映的是具體規則與容量配置，不能據此推論任何演算法永遠較好。

---

**Second Chance 與 CLOCK**：CLOCK 是以環狀 Buffer 與 Clock Hand 實作 Second Chance 的常見方式，用 Reference Bit（存取位元，記為 `R`）近似判斷最近是否使用過，省去每次 Hit 都維護精確 LRU 排序的工作。頁面被存取時設為 `R = 1`；需要找 Victim 時，Hand 遇到 `R = 1` 就清為 0 並繼續，給該頁第二次機會；遇到 `R = 0`，才選作置換對象。

![[Assets/Note/Tech/Database Internals/clock-sweep.png]]

圖中的頁面以環狀方式排列，箭頭指向下一個要檢查的位置，格內的 0、1 表示存取狀態／計數。依原圖圖說，灰色表示目前正被引用的頁面，白色表示目前未被引用的頁面；因此，不能只看填色判斷 Reference Bit，也不能把白色直接視為 `R = 0`，圖中也有白色且數值為 1 的頁面。實際掃描時，還需要判斷頁面是否正在使用、是否可以置換；下方則以單一 Reference Bit 的基本 CLOCK 規則演算。

**CLOCK 的手動演算**：假設 3 個 Frames，初始為空，序列為 `A, B, C, A, D, E`。本例規定每次新載入或 Hit 都把 `R` 設為 1；載入後 Hand 移到下一格，Hit 則不移動 Hand。先依序載入 A、B、C，產生 3 次 Miss，Frames 為 `[A(1), B(1), C(1)]`，Hand 回到 A。第四步讀取 A，為 Hit，A 的 `R` 維持 1，Hand 仍指向 A。

第五步讀取 D，發生 Miss。Hand 依序檢查 A、B、C，三者的 `R` 都是 1，因此逐一清成 0。繞回 A 時，A 已經是 `R = 0`，於是淘汰 A、載入 D，結果為 `[D(1), B(0), C(0)]`，Hand 指向 B。即使 A 剛被存取，單一 Reference Bit 也無法區分它與 B、C 的精確先後，這正是 CLOCK 與精確 LRU 的差別。

第六步讀取 E，發生 Miss；Hand 所指的 B 已是 `R = 0`，可直接淘汰，不需要再繞一整圈。最後為 `[D(1), E(1), C(0)]`，Hand 指向 C。整段共有 **5 次 Miss、1 次 Hit**。此例的位元初始化與 Hand 移動方式已明確指定，其他變體需要依各自規則演算。

**Reference Bit、Dirty Bit 與 Enhanced Second Chance**：`R` 表示近期是否被存取；Modify／Dirty Bit（記為 `M`）表示是否有尚未寫回的修改，兩者用途不同。Enhanced Second Chance 同時參考 `(R, M)`，常見分類依序為 `(0,0)`、`(0,1)`、`(1,0)`、`(1,1)`，希望優先找出不常用且不需寫回的 Clean Page；實際掃描、清除 `R` 與背景寫回方式依實作而異。Additional Reference Bits 則是在多個取樣時間保留存取歷史，與基本 CLOCK 的單一 Reference Bit 不同。

**CLOCK 與 DBMS Buffer Pool 的限制**：CLOCK 是 LRU Approximation，不能因名稱含有 LRU 就認為它具有精確 LRU 的 Stack Property，也不保證避免 Belady’s Anomaly 或順序掃描污染。實際 DBMS 還必須略過正在使用、不可置換的 Pinned Frames；選到 Dirty Page 時，也需要配合寫回與恢復規則，不能只看到 `R = 0` 就直接丟棄資料。參考 [CMU：Buffer Pool Management](https://15445.courses.cs.cmu.edu/spring2026/notes/04-bufferpool.pdf)。

---

**LFU：Frequency 與 Recency 的取捨**：LRU 以最近存取時間決定頁面去留，但某些資料庫工作負載也需要考慮 Frequency（存取頻率）。例如常用頁面暫時沒有被讀取，卻遇到一次性的大量掃描，單靠 Recency 可能把它淘汰。LFU（Least Frequently Used）則追蹤 Page Reference Events，也就是頁面被存取的事件，包含快取命中；若只記錄 Page-In Events，就會漏掉頁面留在快取期間的大量存取，無法反映使用頻率。

LFU 也有取捨：若只累計歷史次數，曾經熱門、現在已經不常用的頁面，可能因舊計數過高而長期占用空間。因此實務上的頻率策略可以搭配取樣、Aging（老化）與近似計數，讓較新的存取模式逐漸影響決策。這不表示所有 DBMS 都使用 LFU 或 TinyLFU，仍須依工作負載與實作成本選擇。

**TinyLFU 與 W-TinyLFU**：TinyLFU 主要是 **Admission Policy（准入策略）**，用近期存取歷史的近似頻率，比較新候選項目與既有淘汰候選，決定是否值得讓新項目進入主要快取。它可以與其他 Replacement Policies 搭配，不是單純把所有頁面按照精確頻率排序。頻率估計也可以記錄目前不在快取中的請求，並透過老化避免過度偏向過去的熱門項目。參考 [TinyLFU 原論文](https://arxiv.org/abs/1512.00727)。

圖中的 Admission、Probation、Protected 三區配置，更精確地說是 **Window TinyLFU（W-TinyLFU）**：前面的小型 Admission Window 使用 LRU，後面的 Main Cache 使用 Segmented LRU（SLRU），再分成 Probation 與 Protected；Frequency Filter 則由 TinyLFU 提供。這個組合同時考慮 Recency 與 Frequency。

![[Assets/Note/Tech/Database Internals/w-tinylfu-admission-probation-protected.png]]

**Admission Window 與 Frequency Filter**：新頁面先進入 Admission Window，先有一小段機會承接短時間內的重複存取。當 Window 需要移出頁面、主快取也需要騰出空間時，離開 Window 的候選會與主快取中通常來自 Probation 的 Victim 比較估計頻率。候選較值得保留時，才准入主快取並取代 Victim；否則可以拒絕候選。這不是設定一個固定頻率門檻，也不是所有新頁面都一定能進入 Probation。

**Probation 與 Protected 的升降級**：通過准入的頁面先進入 Probation；在 Probation 中再次 Hit，就可升到 Protected，不需要先累積很多次存取。Protected 內命中時更新最近使用順序；若升級使 Protected 超過容量，就把其中的 LRU 頁面降回 Probation，而不是立即淘汰。Probation 保留待觀察與降級的頁面，需要回收主快取空間時，通常從其 LRU 端挑選 Victim。圖中的雙向箭頭表示這種升級與降級，向下箭頭則表示 Eviction。

例如頁面 A 離開 Admission Window，估計頻率高於 Probation 中的候選 B，就可以取代 B、進入 Probation；之後 A 再次被存取，就升到 Protected。若 Protected 已滿，其中最久沒有被使用的 C 會降回 Probation，仍有機會再次命中後升級。 **TinyLFU 的頻率比較決定是否准入，SLRU 的命中與容量規則決定升降級**，兩者合作，並非三個 Queue 都按 LFU 排序。參考 [Caffeine：Window TinyLFU](https://github.com/ben-manes/caffeine/wiki/Efficiency#window-tinylfu)。

---

## 修改如何持久化：WAL 與當機恢復

Buffer Pool 中的修改與磁碟資料頁不會隨時一致。交易處理需要同時維持 ACID；這裡先用帳戶例子理解提交、寫回與恢復的關係，再看並行交易的隔離問題。

**WAL、Write-Ahead Log 與 Commit Log**：WAL 可以指 Write-Ahead Logging（預寫式日誌機制），也可以指 Write-Ahead Log（日誌本身）。它的核心規則是： **資料頁寫入持久化儲存之前，必須先持久化足以恢復該頁修改的相關日誌**。資料可以先在 Buffer Pool 中被修改，不必每次都先同步日誌才允許修改記憶體；必須遵守的是 Log 與 Data Page 的持久化順序。

部分系統把這類日誌稱為 Commit Log，例如 Cassandra；名稱仍依系統而定，不能因此理解成日誌只包含已提交交易，或只在 Commit 時才產生記錄。對支援交易的儲存引擎而言，交易進行中的修改與提交狀態，都可能需要記錄。參考 [Cassandra：Storage Engine](https://cassandra.apache.org/doc/stable/cassandra/architecture/storage-engine.html)。

**帳戶 A：未提交的扣款，卻已經寫到磁碟**：假設帳戶 A 的餘額原本是 1,000 元，交易 T1 在 Buffer Pool 中把它改成 900 元，但尚未 Commit。系統先把足以恢復這次修改的日誌持久化，再把包含 900 元的資料頁提前寫回磁碟，以便騰出 Buffer Frame。此時 DB 突然當機，記憶體中的 Buffer Pool 消失，磁碟上的 A 卻已是 900 元。因為 T1 沒有完成提交，這次扣款不能成為有效結果，恢復時必須撤銷其影響，讓 A 回到 1,000 元。

**帳戶 B：入帳已提交，資料頁卻還沒寫到磁碟**：另一筆獨立交易 T2 把帳戶 B 從 2,000 元改成 2,500 元。系統已持久化必要修改日誌與 Commit Record，並回覆提交成功，但資料頁仍只在 Buffer Pool，磁碟上還是 2,000 元。此時 DB 當機，新的 Buffer Pool 內容雖然消失，T2 的提交結果仍應保留，因此要利用持久化日誌重做修改，讓 B 恢復成 2,500 元。這兩個例子假設 A、B 位於不同資料頁，且沒有其他交易修改這兩個值，方便分開理解。

**STEAL 與 NO-STEAL**：STEAL 允許把含有未提交修改的 Dirty Page 提前寫回磁碟，並重用它的 Buffer Frame；它是允許這樣做，不是要求所有修改立即寫回。帳戶 A 就是這種情況。NO-STEAL 則不允許含有未提交修改的頁面以這種方式寫回。因此，STEAL 的好處是減少未完成交易對 Buffer 空間的占用，代價是在傳統恢復模型中需要 UNDO，處理已落盤的未提交修改。

**FORCE 與 NO-FORCE**：FORCE 要求交易完成提交時，其修改的資料頁都已寫回持久化儲存；NO-FORCE 允許提交完成後，資料頁仍留在 Buffer Pool，稍後才寫回。帳戶 B 就是 NO-FORCE 的情況。NO-FORCE 可以降低提交時等待資料頁 I/O 的成本，但在傳統恢復模型中需要 REDO，補回已提交卻尚未落盤的修改。 **FORCE 討論的是資料頁，不是只把 WAL Flush**。

**STEAL + NO-FORCE 的恢復條件**：這組政策允許磁碟上的資料頁暫時包含未提交修改，也暫時缺少已提交修改，因此不能把資料頁本身當成完整的交易結果。正確性要由資料頁、持久化日誌、交易狀態與恢復機制共同保證。關鍵是 WAL 的順序：資料頁落盤之前，相關恢復資訊必須先持久化；在本例要求的同步持久化提交模式下，回覆提交成功之前，必要修改日誌與 Commit Record 也必須先持久化。日誌只留在記憶體或 OS Page Cache，都不足以提供這項保證。

**REDO：從資料頁與持久化日誌重建 Buffer**：DB 重新啟動會建立新的 Buffer Pool，將磁碟上的資料頁讀進來，再利用持久化 WAL 中的重做資訊，重新套用必要修改。帳戶 B 原本的 Buffer 雖然已經消失，磁碟資料頁與 WAL 仍在，所以可以從磁碟上的 2,000 元重建出 2,500 元，再於適當時機寫回。重做資訊可能是新值、頁面變更或操作描述，依日誌格式而定；不需要找回當機前的記憶體。

**UNDO：依交易日誌撤銷未完成修改**：以同時保存重做與撤銷資訊的 ARIES 類型日誌為例，系統會從日誌與交易狀態辨識未完成交易，再沿該交易的修改紀錄反向撤銷。帳戶 A 的日誌可簡化理解為「T1 將 A 從 1,000 改成 900」，因而有足夠資訊取消這次扣款。恢復時可把磁碟上的 900 元讀進新的 Buffer Pool，套用撤銷，再寫回；不是只拿 WAL 與資料頁比較數值，也不是看到 900 就憑空猜原值。不同引擎可能使用獨立 Undo Log 或版本可見性機制，不能把這個流程套到所有 DBMS。

**ARIES 的三階段恢復**：ARIES 的全名是 **Algorithms for Recovery and Isolation Exploiting Semantics**，可理解為「利用操作語意的恢復與隔離演算法」。它是一套以 WAL 為基礎、支援 STEAL + NO-FORCE 的交易恢復設計，不是另一種 Log 名稱，也不是所有 DBMS 都完整採用的固定流程。它將當機恢復分成 **Analysis、REDO、UNDO** 三個階段。

**Analysis：先辨識恢復範圍與交易狀態**：重新啟動後，從 Checkpoint 及後續日誌重建交易與 Dirty Pages 的追蹤資訊，辨識哪些交易已提交、哪些仍待撤銷，以及日誌應從哪裡開始重做。以上述例子而言，T1 是未完成交易，T2 則已提交。Checkpoint 能縮小掃描範圍，不代表它一定是所有資料頁都完全寫回的瞬間快照。

**REDO：先重建日誌描述的必要歷史**：ARIES 的 REDO 採 Repeating History，會重做必要的已記錄操作，可能包含未提交交易的修改與先前的撤銷紀錄；它不是只挑已提交交易來重播。已反映在資料頁上的修改，可利用 `pageLSN` 等資訊判斷並跳過，避免重複套用。帳戶 A 的 900 元若已在磁碟且頁面進度足夠，就不必再做相同修改；帳戶 B 尚未反映的 2,500 元則需要補回。

**UNDO：再移除未完成交易的影響**：完成必要的重做後，再撤銷像 T1 這樣未完成交易的修改。這個簡化例子的最終結果，就是 A 回到 1,000 元，B 保留已提交的 2,500 元。UNDO 的目的不是讓未提交交易繼續執行到 Commit，而是取消它不應留下的效果。

**CLR：讓恢復過程可以再次恢復**：ARIES 在執行撤銷時，會產生 Compensation Log Record（CLR，補償日誌紀錄），記錄已執行的撤銷動作與接下來的撤銷位置，並遵守 WAL 的持久化順序。若恢復過程再次當機，下次可重做已記錄的補償動作，繼續尚未完成的撤銷，避免把同一項修改錯誤地撤銷兩次。參考 [CMU：ARIES 與 Database Recovery](https://15445.courses.cs.cmu.edu/fall2017/notes/22-notes-recovery.pdf)。

**LSN（Log Sequence Number）與持久化進度**：LSN 用來識別日誌中的位置或順序，例如 PostgreSQL 使用 WAL 串流中的位元組位置；它不是 Transaction ID，也不是時間戳。系統可以分別追蹤已產生日誌的位置、已寫出的位置，以及已確保持久化的位置。將 Log 寫到 OS Page Cache，不等於已經 Flush 到能承受當機或斷電的持久化儲存。參考 [PostgreSQL：WAL Internals](https://www.postgresql.org/docs/current/wal-internals.html)。

**Page LSN 與 Flushed LSN**：以常見的恢復模型說明，`pageLSN` 表示目前頁面所反映的最新修改日誌位置，`flushedLSN` 表示日誌已持久化到的進度。寫回某個 Dirty Page 之前，必須確保相關日誌已完整持久化，概念上可寫成 **`flushedLSN ≥ pageLSN`**。這個比較沿用教科書把 LSN 視為記錄序號的簡化方式；若實作用 LSN 表示位元組位置，則還必須涵蓋相關記錄的結尾，不能只寫到記錄開頭。

例如某頁面最新修改的日誌序號為 120，而 `flushedLSN = 100`，就不能先把包含該修改的頁面寫回；必須先持久化涵蓋記錄 120 的日誌，再寫回資料頁。因此，「Flush 到目標 LSN」表示確保 **直到該目標所需的日誌前綴都已持久化**，不是只刷新單筆記錄，也不是把所有資料頁都刷新到同一個 LSN。日誌已持久化到哪裡，也不代表恢復程序已經重播到哪裡，兩者是不同的進度。

**Buffer Flush 與 Commit 的關係**：資料頁 Flush 不等於交易 Commit，Commit 也不一定需要立即 Flush 所有 Dirty Pages。在要求本機同步持久化的提交模式下，通常要先將涵蓋該交易必要修改與 Commit Record 的日誌持久化，再向 Client 回報提交成功；資料頁可以稍後寫回，故障時再依日誌恢復。非同步提交模式則可能先回覆，因此可能在故障後遺失近期已回覆的交易，須依實際設定判斷。參考 [PostgreSQL：Write-Ahead Logging](https://www.postgresql.org/docs/current/wal-intro.html)。

---

## 並行交易：先看異常，再看隔離保證

資料能在故障後恢復，還不足以保證多筆交易交錯執行時的結果正確。先用讀寫異常辨識可能出錯的情況，再看 OCC、PCC、MVCC 與隔離層級如何協調存取。

**Read Anomalies（讀取異常）與 Write Anomalies（寫入異常）**：並行交易交錯執行時，可能讀到尚未提交的修改、在同一筆交易中前後看到不同內容，或覆蓋其他交易的修改。這裡的 Dirty 是相對於其他交易尚未完成提交而言，與 Buffer Pool 中「修改尚未寫回」的 Dirty Page 定義不同；讀到 Dirty Page 並不必然就是 Dirty Read。

**Dirty Read（髒讀）**：T1 把 V 從 100 改成 90，尚未 Commit，T2 就讀到 90，此時已經構成 Dirty Read。若 T1 後來失敗並 Rollback，V 回到 100，T2 便曾依據最後沒有成立的修改做出判斷。但 **T1 後來是否真的失敗，不是髒讀成立的必要條件；重點是 T2 讀取當下，修改尚未提交**。

**Nonrepeatable Read（不可重複讀）**：T1 先讀取某筆資料，得到 V = 100；接著 T2 把同一筆資料改成 120 並 Commit；T1 在自己同一筆交易尚未結束前再次讀取，卻得到 V = 120。兩次讀到的都可以是已提交值，因此不需要發生 Dirty Read。重點是同一交易重讀同一筆資料時，內容因其他已提交交易而改變；資料列被刪除，也可能造成重讀時找不到。若 T1 已結束，再開始另一筆交易讀到新值，則不屬於這裡的不可重複讀。

**Phantom Records 與 Phantom Read（幻讀）**：T1 以相同查詢條件重查時，符合條件的資料列集合因其他交易已提交的修改而改變。例如先查詢 `price < 100` 得到 3 筆，T2 新增一筆符合條件的商品並 Commit，T1 在同一交易重查得到 4 筆，新出現的符合條件資料列就是 Phantom Record，而觀察到集合改變的現象稱為 Phantom Read。刪除符合條件的列，或更新欄位讓列進入、離開查詢集合，也可能造成這種變化；不一定要使用數值範圍，任何符合條件的集合查詢都可能涉及這個問題。通常稱為「幻讀」，不是「換讀」。

**不可重複讀與幻讀的區別**：前者著重同一筆資料重讀時值或存在性改變，後者著重相同查詢條件得到的資料列集合改變；某些具體修改可能同時涉及兩種觀察角度。三種讀取異常不要求所有隔離層級都會發生，實際行為取決於 DBMS 與隔離設定。參考 [PostgreSQL：Transaction Isolation](https://www.postgresql.org/docs/current/transaction-iso.html)。

**Lost Update（遺失更新）**：假設 V = 100，T1、T2 都先讀到這個舊值。T1 計算加 10，寫入 110 並 Commit；T2 卻仍依據自己先前讀到的 100 計算加 20，再寫入 120 並 Commit。原本應保留兩次加值、得到 130，最後卻只剩 120，T1 的更新效果被覆蓋。重點是以過期的讀取結果進行 Read-Modify-Write，而不是任何較晚的寫入都算遺失更新。此例即使 T2 等到 T1 Commit 後才寫入，仍可能發生，所以不能只靠禁止 Dirty Write 解決。依需求可以使用版本檢查、先鎖再讀，或適當的原子更新。參考 [Microsoft：Concurrency Effects](https://learn.microsoft.com/en-us/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide)。

**Dirty Write（髒寫）**：T1 把 V 從 100 改成 90，尚未 Commit 或完成 Rollback，T2 就直接覆寫同一個資料項目，例如寫成 80，這就是 Dirty Write。它不要求 T2 已經 Commit，也不要求 T2 曾讀到 T1 的值；判斷重點是 **T2 寫入當下，正在覆寫 T1 尚未結束的修改**。這會讓 Rollback 變得複雜：若 T1 失敗，系統不能只把值改回 100，否則可能又破壞 T2 的修改。實務上常藉由寫入鎖或寫入衝突檢查阻止這類覆寫。

**Lost Update 與 Dirty Write 的區別**：Lost Update 關心某筆更新的效果因並行操作而遺失；Dirty Write 關心寫入是否覆蓋另一筆未結束交易的修改。前面的 Lost Update 範例刻意讓 T1 先 Commit、T2 才寫入，因此不是 Dirty Write；兩者不是單純靠「T2 是否 Commit」區分。參考 [Berenson 等：A Critique of ANSI SQL Isolation Levels](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/tr-95-51.pdf)。

### 並行控制與隔離層級

**Concurrency Control（並行控制）**：多筆交易可能交錯讀寫相同資料，因此需要協調它們能看到什麼、何時可以修改，以及衝突時應等待還是重試。Transaction 是操作的交易單位，不等於悲觀鎖；OCC、PCC 與 MVCC 也不是完全互斥的三選一分類。

**OCC（Optimistic Concurrency Control，樂觀並行控制）**：先執行讀取與計算，再驗證是否與其他交易衝突；未通過就撤銷或重試，而不是先把未驗證的結果正式提交。應用層常見方式是為資料加上會隨修改變動的 `version` 或可靠的版本時間戳。固定不變的 Primary Key 只能識別資料列，不能單獨識別它是否被修改。

完整交易層級的 OCC 通常分成 **Read、Validation、Write** 三階段：Read Phase 在私有工作區讀取、計算與暫存修改，建立 Read Set 與 Write Set；Validation Phase 檢查與其他交易的讀寫依賴是否衝突；通過後才於 Write Phase 發布修改，失敗則撤銷或重試。驗證可以回頭檢查已提交交易（Backward Validation），或檢查仍在執行的交易（Forward Validation）；驗證與發布仍須協調，避免兩筆交易同時通過卻破壞彼此假設，因此樂觀控制不代表內部完全沒有同步。

例如讀到 `balance = 1000, version = 5`，更新時可以執行 `UPDATE accounts SET balance = 900, version = version + 1 WHERE id = 1 AND version = 5`。若版本已被別人改變，就無法匹配原版本，需要處理衝突；沒有匹配也可能是資料列已被刪除。 **版本條件必須和更新放在同一個原子操作中**，不能先查版本、再無條件更新。這是單列樂觀並行控制的例子，不代表已驗證整筆交易的所有讀寫依賴。參考 [Microsoft：Optimistic Concurrency](https://learn.microsoft.com/en-us/ef/core/saving/concurrency)。

**PCC（Pessimistic Concurrency Control，悲觀並行控制）**：在交易執行過程中，先對可能衝突的操作進行協調，不安全時就等待或中止，而不是等全部執行完才統一驗證。常見方式是在交易中以 `SELECT ... FOR UPDATE` 鎖住目標資料列，取得存取權後再讀取與修改，其他衝突操作等待鎖釋放。鎖可以保護交易的邏輯資料；前面討論的 Latch 則保護短時間的頁面與索引結構操作，兩者用途不同。參考 [PostgreSQL：Explicit Locking](https://www.postgresql.org/docs/current/explicit-locking.html)。

**Lock-Based PCC 與 2PL**：Two-Phase Locking 將鎖管理分成 Growing Phase（取得鎖、不釋放）與 Shrinking Phase（釋放鎖、不再取得新鎖），重點是一旦開始釋放，就不能再取得新的交易鎖。它不是要求一開始就拿齊所有鎖，也不是 Two-Phase Commit（2PC）；基本 2PL 提供衝突可序列化，仍需搭配適當的資料列／範圍保護與提交、回滾規則，Strict 2PL 等變體會進一步限制鎖的釋放時機。

**Timestamp Ordering（時間戳排序）**：本書也把執行中逐步檢查的時間戳排序放在 PCC 下，不同教材可能採用不同分類，因此 PCC 不能只理解為「一定使用交易鎖」。每筆交易有 `TS(T)`，每個資料項目維護最大讀取時間戳 `R_TS` 與最大寫入時間戳 `W_TS`：讀取時若 `TS(T) < W_TS`，或寫入時 `TS(T) < R_TS`，表示違反預定邏輯順序，需要中止；基本協定在 `TS(T) < W_TS` 的寫入也會中止，但採用 Thomas Write Rule 時，可以略過這次過時寫入、讓交易繼續，而不是讓舊值覆蓋新值。這裡的時間戳用來排序，不一定是實際時鐘；也不同於僅用一個 `version` 欄位做應用層的樂觀更新檢查。

**OCC／PCC 的選擇：看衝突程度與成本**：讀多寫少的工作負載常適合 OCC，但核心不是只看讀寫比例，而是資料競爭程度（Contention）。大量寫入若分散在不同帳戶，仍可能很少衝突；整體寫入不多，卻集中搶同一筆庫存，也可能高度競爭。衝突少、重試成本低時，OCC 可以減少等待；衝突多、反覆重做成本高時，PCC 可能較合適，但也要付出鎖管理、等待與 Deadlock 處理的成本。這是選擇方向，不是所有實作與工作負載都成立的效能定律。參考 [Microsoft：Concurrency Control](https://learn.microsoft.com/en-us/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide)。

**MVCC（Multi-Version Concurrency Control，多版本並行控制）**：保留資料的多個版本，依 Snapshot 與隔離規則，決定交易能讀到哪個版本。單純增加一個 `version` 欄位檢查更新衝突，不等於實作 MVCC；MVCC 還涉及版本可見性與舊版本回收。它可以搭配樂觀驗證或鎖定，採用 MVCC 也不代表寫入永遠不等待、沒有 Deadlock，或自動具備 Serializable。參考 [PostgreSQL：MVCC](https://www.postgresql.org/docs/current/mvcc-intro.html)。

**Isolation Level 可以在 Transaction 中設定**：隔離層級是 DBMS 提供的交易行為設定，不是把 Dirty Read、Phantom Read 等異常逐項打開或關閉。交易未指定時，會沿用連線或資料庫的預設隔離層級；應用程式也可以在開始單筆交易時，透過 SQL、Driver 或 ORM 的 Transaction API 指定本次需要的 Isolation Level。這是在選擇需要的隔離保證，不是直接切換 OCC、PCC 或 MVCC 實作。

以 PostgreSQL 為例，可以用 `BEGIN TRANSACTION ISOLATION LEVEL SERIALIZABLE;` 開始交易，執行本次查詢與修改後再 `COMMIT;`；也可以先 `BEGIN;`，再執行 `SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;`。隔離層級必須在該交易第一個查詢或資料修改之前設定，不能讀到一半才提高層級。`SET TRANSACTION` 只影響目前這筆交易，後續交易仍沿用預設值；其他 DBMS 的語法與生效範圍需分別確認。參考 [PostgreSQL：SET TRANSACTION](https://www.postgresql.org/docs/current/sql-set-transaction.html)。

**實務上怎麼選擇**？：常見做法是先確認 DBMS 的預設值與保證，讓一般交易在已驗證正確的預設設定下執行，需要更強隔離的交易再個別指定。例如 PostgreSQL 預設為 Read Committed；單列庫存扣減可以視需求使用帶條件的原子更新、版本檢查或先鎖再讀。若一筆交易包含多次讀取並要求固定快照，可以評估該 DBMS 的 Repeatable Read／Snapshot Isolation；若要保護跨列、範圍或聚合判斷下的不變條件，可以評估 Serializable 或足夠的顯式協調。使用 Serializable 仍需保留正確的業務檢查，並處理 Serialization Failure，必要時從頭重試整筆交易，而不是只重跑最後一條 SQL。隔離層級的選擇應依正確性需求與實測成本決定。參考 [PostgreSQL：Transaction Isolation](https://www.postgresql.org/docs/current/transaction-iso.html)。

**四種標準隔離層級與讀取異常**：下表表示 SQL 標準對三種讀取異常的最低限制；「可能」代表允許發生，不代表每次都會發生。

| 隔離層級 | Dirty Read | Nonrepeatable Read | Phantom Read |
| --- | --- | --- | --- |
| Read Uncommitted | 可能 | 可能 | 可能 |
| Read Committed | 禁止 | 可能 | 可能 |
| Repeatable Read | 禁止 | 禁止 | 標準允許 |
| Serializable | 禁止 | 禁止 | 禁止 |

實際 DBMS 可以提供比標準最低要求更強的保證。例如 PostgreSQL 的 Read Uncommitted 等同 Read Committed，其 Repeatable Read 也禁止 Phantom Read，但仍可能發生 Write Skew 等序列化異常。因此，這三種異常都被禁止，不足以單獨證明已達到 Serializable；上表也未列出所有寫入異常。參考 [PostgreSQL：Transaction Isolation](https://www.postgresql.org/docs/current/transaction-iso.html)。

**Serializable（可序列化）與串行順序**：Serializable 要求已提交交易的並行執行效果，等價於某個逐筆執行的合法順序。交易實際上仍可以交錯執行；要檢查的不只是最後的資料值，也包含交易讀到了什麼、依此做出哪些操作。OCC 與 PCC 都能透過適當設計實現這項保證，但單列版本檢查或局部鎖定不自動保證整筆交易可序列化。應用是否需要選擇 Serializable，取決於要保護的規則與其他協調方式。

假設有 T1、T2、T3，忽略其他執行限制，串行排列有 `T1→T2→T3`、`T1→T3→T2`、`T2→T1→T3`、`T2→T3→T1`、`T3→T1→T2`、`T3→T2→T1`，共 `3! = 6` 種。這是交易整體的串行順序，不是全部並行交錯方式，也不代表一定有 6 種不同結果；每筆交易內有多個步驟時，交錯方式可能更多。

**透過等待或驗證維持合法串行順序**：DB 不需要先選定唯一正確順序，而是透過鎖定或依賴驗證，確保提交的執行歷史能對應到某個合法串行順序。若無法滿足，就讓衝突操作等待，或撤銷交易並要求重試。Serializable 本身也不保證先開始的交易一定先執行完畢或先取得資源。

**庫存只剩一件的例子**：T1、T2 都想購買同一件商品，且交易邏輯正確檢查庫存。如果效果等價於 `T1→T2`，T1 可以買到，T2 看到庫存不足；等價於 `T2→T1` 則相反，兩者都合法。若兩筆交易都因讀到庫存 1 而購買成功，就無法等價於這兩種串行執行中的任何一種。OCC 可以利用原子版本檢查或適當驗證讓其中一筆失敗，重試後重新檢查庫存；PCC 可以先鎖住庫存列，讓另一筆取得鎖後再檢查。Serializable 協調並行效果，交易本身仍要寫對庫存檢查與更新邏輯。參考 [PostgreSQL：Transaction Isolation](https://www.postgresql.org/docs/current/transaction-iso.html)。

**Serializable 與 Eventual Consistency（最終一致性）不同**：前面討論的是交易隔離，不能概括為「DB 最後在意的都是最終一致性」。Eventual Consistency 通常用來描述多副本資料：若沒有新的更新，且更新傳播與衝突處理能持續完成，各副本最終會收斂；在此之前，讀取可能看到舊資料。它本身不保證多筆交易的執行歷史可序列化，也不保證庫存不超賣。例如兩個副本最後都同步成同一個超賣結果，仍不符合庫存業務規則。參考 [AWS：DynamoDB Read Consistency](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadConsistency.html)。

**ACID 的 Consistency 也要分開理解**：ACID 的一致性是交易應維持資料約束與業務不變條件；Serializable 是交易隔離的保證；Eventual Consistency 則描述副本或讀取結果的收斂性質。它們討論的層次不同，可以在同一個系統中分別設計。例如主庫內的交易可要求 Serializable，同時以非同步方式把結果傳給唯讀副本；從副本讀取仍可能落後主庫。

---

## 儲存引擎內部的頁面保護：Latch

交易層協調的是邏輯資料，執行緒實際操作 Page 與索引結構時，還需要更短時間的保護。這也把並行控制接回前面的 B-Tree 分裂與頁面修改。

**Latch 與讀者／寫者問題**：交易的隔離保護邏輯資料，儲存引擎內部還需要保護 Page 內容與 B-Tree 結構，避免執行緒讀到修改或分裂到一半的頁面。這類短時間的保護稱為 Latch，可以使用 Readers–Writer Lock 實作：多個 Readers 以 Shared 模式同時讀取，Writer 則必須取得 Exclusive Latch，才能修改同一個頁面。這與作業系統中的讀者／寫者問題相通，Reader Priority、Writer Priority 與 Semaphore 的完整流程可參考另一篇 [Famous Synchronization Problems](https://blog.walle4561.com/articles/posts/famous-synchronization-problems/)。

若採用 **Writer Priority（寫者優先）**，等待中的 Writer 會阻止新 Readers 插隊，待既有 Readers 釋放 Shared Latch 後取得獨占權寫入，期間其他 Readers 與 Writers 都必須等待。

**Latch 的實作方式**：可以使用 Busy-wait（忙等，例如 Spinlock）、Blocking（阻塞等待，由排程器喚醒），或先短暫自旋再阻塞的混合方式；Spinlock 是忙等的一種實作，不是與 Busy-wait 分開的另一類。Readers–Writer Lock 則描述讀共享、寫獨占的存取規則，等待時也能採用上述方式。

**Latch Crabbing（Latch Coupling）**：沿 B-Tree 向下時，先取得子節點的 Latch；若插入空間足夠、不會 Split，或刪除後仍符合占用要求、不需向父節點傳播 Merge／Rebalancing，就可以釋放父節點的 Latch。若可能 Split 或因刪除而需要 Merge／Rebalancing，就保留必要的祖先 Latches，直到確認不再需要向上修改，再及早釋放。

![[Assets/Note/Tech/Database Internals/latch-crabbing.png]]
