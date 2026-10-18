---
blog: true
blog_title: Database Internals 閱讀筆記
blog_date: '2026-10-15'
blog_url: https://blog.walle4561.com/articles/posts/database-internals-reading-notes/
blog_tag: OS & Networking
blog_topic: OS
---

**OLTP 與 OLAP**：OLTP（Online Transaction Processing）處理日常交易，著重高併發（Concurrency）、低延遲（Low Latency）、短交易（Short Transactions）與 ACID。例如電商建立訂單、扣除庫存、記錄付款、查詢訂單狀態，會頻繁執行 `INSERT`、`UPDATE`、`DELETE`、`SELECT`，每筆交易通常只處理少量資料。OLAP 則著重資料分析，常涉及複雜 `SELECT`、`JOIN` 與聚合，單次查詢可能需要處理大量資料。

**TPC-C 與 TPC-H**：OLTP 是工作負載，TPC-C 則是衡量 OLTP 系統效能的 Benchmark。TPC-C 模擬批發商的訂單與庫存管理，混合執行 New-Order（建立訂單）、Payment（付款）、Order-Status（訂單狀態）、Delivery（出貨）與 Stock-Level（庫存水位）五種交易。

TPC-C 的指標為 tpmC（Transactions per Minute C）。例如 10,000 tpmC，表示在符合測試規範及其他交易負載條件下，每分鐘完成 10,000 筆 New-Order，**不是所有 SQL 操作或交易加總後的 TPS**。TPC-C 成績高，也不代表所有實際應用都比較快。OLAP 的複雜查詢與分析效能則可透過 TPC-H、TPC-DS 等 Benchmark 評估。

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

**Index-Organized Table 與 Heap File**：Index-Organized Table 將實際 Record 放在索引的 Leaf Node，沿著 B+ Tree 找到 Leaf Node 就能取得資料。Heap File 則將資料與索引分開，索引的 Leaf Node 保存 Record Location，再透過這個位置找到 Data Block 中的 Record。

**Clustered Index 與 Non-Clustered Index**：Clustered Index 依索引順序組織資料本身，一個 Table 最多有一個。Non-Clustered Index 則額外維護查找結構，透過索引定位資料，必要時回表取得完整資料。

例如 `Users` 依 `ID` 排列為 Amy（`ID` 1）、Walle（`ID` 2）、John（`ID` 3），查詢 `ID BETWEEN 1 AND 3` 時，可以沿 `ID` 順序取得資料。若再建立依 `Name` 排序的索引，順序就變成 Amy、John、Walle，索引項目分別指向 `ID` 1、3、2，提供另一條查找路徑。

**Secondary Index（次級索引）**：讓查詢能使用 Primary Key 以外的欄位，例如透過 `Email` 找 `User`。依儲存方式不同，Secondary Index 可能直接保存資料列位置，也可能保存 Primary Key，先取得 Key，再查 Primary Index 才能拿到完整資料。這也說明了為什麼透過索引找到項目後，有時仍需要額外回表。

**B+ Tree 與 LSM-Tree**：B+ Tree 使用有序的結構，常見實作傾向於原地更新 Page；LSM-Tree 則先利用記憶體 Buffer 接受寫入，再透過不可變（Immutable）的磁碟檔案與 Compaction 整理資料。Append-Only、Copy-on-Write 都涉及把新版本寫到新位置的思路，但具體更新方式仍取決於儲存引擎。B+ Tree 的高 Fanout 可以降低樹高，減少查找需要經過的節點，進而降低可能發生的 Disk I/O。

---

**Database Page 與 OS Page Cache**：資料庫從 B+ Tree 取得 `Page ID` 後，需要先定位檔案中的位置，再把需要的資料讀進記憶體。以 Linux 使用一般 buffered `pread()` 的情況為例，OS 會先檢查 Page Cache，已有快取就能使用；沒有快取時，通常需要從 Disk 讀入，再將資料複製到資料庫的 `User`-Space Buffer。

假設 Database Page 為 8 KiB（8192 Bytes），OS Memory Page 為 4 KiB（4096 Bytes），且 `Page ID` 從 0 開始、資料頁在檔案中連續排列。讀取 Database Page 5 時，File Offset 為 `5 × 8192 = 40960 Bytes`，也就是 `0xA000`。這個 8 KiB 的資料頁涵蓋 OS File Page 10、11，檔案範圍分別為 `0xA000`–`0xAFFF`、`0xB000`–`0xBFFF`。**一個 Database Page 可以涵蓋多個 OS Pages，載入後對應的實體記憶體也不必連續。**

**Virtual Memory、TLB 與 Page Table**：資料庫同樣使用虛擬記憶體。Page Cache 查找的是檔案內容是否已在 RAM，TLB 則快取 Virtual Page 到 Physical Frame 的位址映射。兩者雖然都涉及 Page，處理的問題不同；CPU 在核心態存取 Page Cache、複製資料，以及在使用者態存取 Database Buffer 時，都涉及虛擬位址轉換。

延續上例，假設 Database Buffer 的起始虛擬位址為 `0x7F0010000000`，要讀取 Buffer 內偏移 4660 Bytes 的欄位。4660 的十六進位是 `0x1234`，因此該欄位的 Virtual Address 為 `0x7F0010001234`。這個 Offset 是示範值，並不是 B+ Tree 固定的欄位位置。

OS Page Size 為 4096 Bytes，也就是 2¹² Bytes，因此位址低 12 bits 是 Page Offset。`4660 = 4096 + 564`，表示資料位於 Buffer 的第二個 OS Page，距離該 Page 開頭 564 Bytes（`0x234`）；Virtual Page Number 則為 `0x7F0010001`。

CPU 使用這個 Virtual Page Number 查 TLB。若 TLB Hit，就直接取得 Physical Frame；若 TLB Miss，則需要查 Page Table。假設查到的 Physical Frame 起始位址為 `0x009A7000`，加上 Page Offset `0x234`，得到 Physical Address `0x009A7234`。有效映射可以快取到 TLB，供後續存取使用。

**Page Cache Miss、TLB Miss 與 Page Fault**：Page Cache Miss 表示檔案內容沒有快取，通常需要讀取 Disk；TLB Miss 表示沒有快取位址映射，需要查 Page Table；Page Fault 則表示目前頁表映射無法直接滿足存取，需要 OS 處理。**TLB Miss 不等於 Page Fault，也不等於 Page Cache Miss，更不代表一定需要 Disk I/O。**例如 `mmap()` 的檔案頁若已在 Page Cache，可能只需要 Minor Page Fault 建立映射；若還必須從儲存裝置讀入，則可能涉及 Major Page Fault。

資料庫層處理 `Page ID` 到 File Offset，Page Cache 處理檔案內容的快取，Virtual Memory 則處理 Virtual Page 到 Physical Frame。把這三層分開，才能理解從 B+ Tree 找到 Page，到 CPU 實際存取資料之間的流程。

---

**B-Tree Complexity**：以 M 表示資料總數（Total Data Number）、k 表示 Fanout、h 表示 Tree Height，可以用以下關係理解樹高：

$$
k^h \approx M
$$

$$
h \approx \log_k M
$$

延伸閱讀：[M-Way Search Tree 與 B-Tree](https://blog.walle4561.com/articles/posts/m-way-search-tree-and-b-tree/)。

**B+ Tree 的 Leaf Links**：典型的 B+ Tree 會把相鄰 Leaf Nodes 串接起來，讓範圍查詢先沿樹找到起始 Leaf，再沿葉節點連結依序讀取後面的資料，不必反覆回到父節點尋找下一頁。這主要用來提高範圍查詢與順序掃描的效率；只有 Leaf Nodes 相連，並不足以判定這是一棵 B-link Tree。

**B-link Tree 的 Right Links 與 High Key**：B-link Tree 在 B-tree／B+ Tree 的結構上加入同層節點的 Right Sibling Link，並配合 High Key 標示頁面範圍，這個機制也涵蓋 Internal Nodes。它的關鍵用途是處理並行搜尋與節點分裂：即使父節點尚未更新，搜尋仍可能先落到分裂前的舊頁面，再藉由 High Key 判斷目標已超出範圍，沿 Right Link 找到新的右側頁面。

![[Assets/Note/Tech/Database Internals/sibling-links.png]]

因此，B+ Tree 的葉節點串接與 B-link Tree 的橫向連結可以同時存在，不能只靠「有沒有兄弟連結」區分。圖中的 Internal Nodes 也有橫向連結，應搭配 B-link Tree 的機制理解，而不只解釋成葉節點的範圍掃描。PostgreSQL 採用 Lehman–Yao 的 B-link Tree 思路，並在各層維護左右兄弟連結。

**Rightmost Pointer（最右側子節點指標）**：Internal Node 有 N 個分隔 Key，就需要 `N+1` 個 Child Pointers，因為 N 個 Key 將搜尋範圍分成 `N+1` 段。例如只有 10、20 兩個 Key，仍需要三個 Child Pointers，而不是三個 Key。依圖中的邊界慣例，三段分別是小於 10、大於等於 10 且小於 20，以及大於等於 20。

![[Assets/Note/Tech/Database Internals/rightmost-pointer.png]]

圖中的 `Kₛ` 是 Search Key，也就是正在查找的值；它用來與 `K₁`、`K₂`、`K₃` 比較，決定走哪一個 Child Pointer，並不是額外放到最右側的 Key。當 `Kₛ` 大於等於 `K₃`，就沿 Rightmost Pointer 往下查找。頁面內可以把每個 Key 與左側 Child Pointer 放進同一個 Cell，再把多出來的最右側 Pointer 獨立存放在 Header 中。

SQLite 就採用這種將 Rightmost Pointer 獨立存放的配置：Interior B-Tree Page 的 Header 在 Offset 8 保存一個 4 Bytes 的最右側子頁面編號，Leaf Page 則沒有這個欄位。不過相等值的路由慣例要分開看：SQLite 官方描述的是左側子樹包含小於等於分隔 Key 的值，右側子樹包含大於分隔 Key 的值，因此不能直接套用圖中的所有不等號。

**Rightmost Pointer 與 High Key 的配對**：假設 Internal Node 有 20、50 兩個分隔 Key，以及 `P₀`、`P₁`、`P₂` 三個 Child Pointers。採用左閉右開的範圍慣例時，`P₀` 負責小於 20 的子樹，`P₁` 負責 `20 ≤ Key < 50`，`P₂` 則負責 `Key ≥ 50`。例如查詢 30，因為 `20 ≤ 30 < 50`，所以沿 `P₁` 向下查找。`P₂` 稱為 Rightmost Pointer，是因為它指向目前節點最右側的子樹。

如果把每個 Pointer 與其搜尋範圍的上界配對，就會得到`（P₀, 20）`、`（P₁, 50）`，但 `P₂` 在目前節點內沒有下一個一般分隔 Key 可以配對。這不代表它的範圍一定無限延伸；非根節點的整體範圍仍受到祖先節點的分隔條件限制。

假設目前節點管理的範圍上界為 80，可以將這個上界記為 **`High Key = 80`**。在把最右側 Child Pointer 與節點 High Key 放進同一個 Cell 的配置中，三個 Cell 就能統一表示為 **`（P₀, 20）`、`（P₁, 50）`、`（P₂, 80）`**。其中 `P₂` 對應的範圍為 `50 ≤ Key < 80`；所有子樹也都受到目前節點整體下界的限制。

![[Assets/Note/Tech/Database Internals/high-key-pointer-ranges.png]]

圖 a 的 `K₁`、`K₂` 將範圍切成三段，分別由 `P₀`、`P₁`、`P₂` 指向對應子樹；`P₂` 在這個節點內沒有額外的 Key 標示上界，圖中以 `+∞` 表示。圖 b 則加入 `K₃` 作為目前節點的 High Key，讓最右側的 `P₂` 也有明確的範圍上界，可以和 `K₃` 放在同一個 Cell。`K₃` 並沒有增加第四個 Child Pointer，而是為原本的最右側子樹補上上界。

**一般分隔 Key 與 High Key**：20、50 用來區分目前節點內不同子樹的搜尋範圍；80 則表示整個目前節點的範圍上界。在上述配置中，High Key 同時提供最右側子樹的上界，讓 `P₂` 也能採用與其他 Child Pointers 相同的 Cell 格式，不必另外保留獨立的 Rightmost Pointer 欄位。這是另一種頁面配置選擇，與前面 SQLite 將 Rightmost Pointer 獨立存放在 Header 的方式不同。

**High Key 與 Right Sibling Pointer**：延續 `High Key = 80`、上界不包含 80 的假設，搜尋 90 時，因為 `90 ≥ 80`，表示目標已超出目前節點的範圍。在 B-link Tree 中，可以沿 Right Sibling Pointer 前往同一層右側的兄弟節點，繼續尋找目標所屬的範圍。

**Rightmost Child Pointer 是向下走，Right Sibling Pointer 則是向右走。**前者指向目前節點最右側的子樹，後者指向同一層的相鄰節點。High Key 的用途是標示節點範圍；把它與 Rightmost Child Pointer 配成一個 Cell，是利用這個上界統一儲存格式，並非 High Key 必須依賴這種配對才能存在，也不是所有 B-link Tree 都必須使用相同的實體配置。此處的比較條件沿用範例的左閉右開慣例，實作時仍須依資料庫對邊界相等值的規則判斷。

**Split 與 Rightmost Pointer 更新**：圖中右側子節點分裂後，原本的一個 Child 變成兩個，父節點需要加入分隔 Key `K₄`，以及對應的 Child Pointer。父節點原本的最右側路徑也要調整，讓 Rightmost Pointer 指向分裂後最右邊的新節點，其他搜尋範圍則由相應的 Cell 引導。

![[Assets/Note/Tech/Database Internals/rightmost-pointer-split.png]]

**Rebalancing（重新分配）**：當某個 Page 很滿、相鄰的兄弟 Page 卻還有空間，可以先把部分 Cells 移到較空的 Page，平衡各頁面的空間使用率（Node Occupancy），延後 Split；刪除資料時，也可以從兄弟節點移入資料，避免太早 Merge。這裡關心的是資料占用空間，而不是讀寫請求集中造成的 Hot Spot。

![[Assets/Note/Tech/Database Internals/rebalancing.png]]

圖中調整前，左節點有 `10、13、16`，右節點只有 `18`；調整後把 `16` 移到右節點，變成左邊 `10、13`、右邊 `16、18`。兩個節點負責的搜尋範圍改變，因此父節點的分隔 Key 也從 `18` 更新為 `16`。

這次調整前後的樹高相同。長期提高節點填滿程度，才可能讓相同資料量使用更少的 Pages 與層數；不是每次 Rebalancing 都會降低樹高。另外，跨 Page 重新分配 Cells 是這裡的 Rebalancing，同一個 Page 內搬動 Cells、整理零碎空間則是 Compaction／Defragmentation。

---

**Right-Only Appends**：當索引 Key 單調遞增，例如自動遞增的 `ID`，新增資料就會持續落在最右側的 Leaf Page。這讓儲存引擎可以針對追加寫入優化：PostgreSQL 的 Fastpath 著重省掉重複搜尋 Leaf 的路徑，SQLite 的 Quickbalance 則著重最右側 Page 滿了以後，如何減少重新分配資料的工作。

**PostgreSQL Fastpath**：記住先前使用的最右側 Leaf Page，下一次插入時先檢查這一頁是否仍符合條件、是否有足夠空間。符合條件就能直接在該頁內找到適當位置並插入，省掉從 Root 重新往下搜尋的過程；不符合條件則回到一般插入流程。這不是 First Fit：First Fit 通常是依序找第一個放得下的空間，Fastpath 則是先嘗試已知的最右側 Page。它優化的是尋找目標頁面的成本，並不代表忽略頁內排序或節點容量限制。

**SQLite Quickbalance**：在符合適用條件、插入位置位於最右端且目標 Leaf Page 已滿時，可以保留原頁面的資料，另外建立一個新的最右側 Leaf Page，先放入新增的最大 Key，再更新父節點的分隔 Key 與 Child Pointer。它不要求這次把新舊兩頁的資料平均分配，因為預期後續遞增 Key 還會繼續填入新頁。這是特定追加情境的優化，不能套用到所有位置的插入。

例如簡化假設每個 Leaf Page 最多容納 4 筆資料，原本左頁存放 `1、2、3、4`，最右頁存放 `5、6、7、8`。現在新增 `9`，最右頁已經沒有空間；Quickbalance 的做法是保留這兩頁，建立新的最右頁，先只存放 `9`，並讓父節點加入相應的導航資訊。之後再插入 `10、11、12`，新頁就逐步填成 `9、10、11、12`，不必為了這次插入，先把舊頁的 `5、6、7、8` 搬過去平均分配。實際頁面容量取決於 Cell 大小，這裡的 4 筆只是方便理解的假設。

**填滿程度不平均，不等於樹的高度失衡。**上述例子假設父節點仍有空間，因此新增 Leaf Page 後，各 Leaf 的深度相同，樹高不變；若父節點也放不下，仍需要進一步處理，必要時向上傳遞。Quickbalance 只是接受新頁暫時較空，以減少當次搬移資料的成本，並沒有放棄 B-Tree 的結構規則。

Rebalancing 與 Quickbalance 的取捨也不同：前者先利用兄弟 Page 的剩餘空間，盡量延後 Split；後者則利用「後續資料很可能繼續往右新增」的特性，保留舊頁面滿載，讓新頁承接後續寫入。Fastpath 解決的是如何更快找到可插入的頁面，Quickbalance 解決的則是特定情況下頁面已滿時如何安排資料，兩者不是完全相同的機制。

---

**Bulk Loading 與 Bottom-Up 建樹**：Bulk Loading 是一次載入整批資料；這裡討論的 B-Tree 建索引方式採用 **Bottom-Up（由下往上）**，先建立 Leaf Pages，再逐層建立 Internal Pages，最後形成 Root。

例如要為 `1～1,000,000` 的資料建立索引，先按 Key 排序，再依序把資料填入 Leaf Pages，接著依各個 Leaf 所負責的範圍，建立上層的分隔 Key 與 Child Pointers。如果一層仍放不下所有指標，就繼續向上建立，直到形成 Root。這與逐筆插入、每次從 Root 尋找目標 Leaf，再於空間不足時進行 Split 的流程不同，可以減少重複搜尋與分裂的成本。

`page-wise` 表示「以 Page 為單位」。在這個情境中，可以理解成一頁一頁建立與填入資料，再從這批頁面建立上一層，而不是每筆資料都重新走完整的插入路徑。實作上也可以保留部分可用空間，供後續插入使用，不一定把每頁填到全滿。

**Bulk Loading 強調整批處理，Bottom-Up 強調建樹方向。**一般的批次匯入不一定採用 Bottom-Up；此處描述的是利用排序後的整批資料，由 Leaf 向上直接建立 B-Tree 索引的方式。

---

**Breadcrumbs（麵包屑）與 Parent Pointer**：Node Split 可能需要往上更新父節點，因此搜尋時可以順便記錄從 Root 往下經過的頁面，以及每一層選擇的 Child Pointer 位置。這份路徑紀錄就是 Breadcrumbs，讓操作在需要時能沿原路向上回溯。

![[Assets/Note/Tech/Database Internals/breadcrumbs.png]]

圖中的黑色連線表示往下搜尋的路徑，右側 Breadcrumbs 則記錄各層經過的節點與所選位置。它保存的是「這次從哪裡走下來」，並不是在每個 Child Page 裡永久增加一條指向 Parent 的連結。圖中的 1、2、2 是所選位置的示意，不代表 PostgreSQL 欄位一定使用相同的編號方式。

**Parent Node 存在，不代表 Child 必須保存 Parent Pointer。除了 Root 以外，B-Tree 的每個節點都有父節點；父節點原本就透過 Child Pointer（Downlink）指向子節點。往上找 Parent 時，可以使用搜尋過程留下的 Breadcrumbs，不必再從 Child 讀取 Parent Pointer。例如下降路徑為 Root `P1`、Internal `P5`、Internal `P8`，最後到達 Leaf `P12`；若 `P12` 發生 Split，就先從 Stack 取得 `P8`，若父節點也分裂，再回到 `P5`，必要時一路處理到 Root。

Breadcrumbs 通常是一次操作或 Cursor 在記憶體中的 **Traversal State（遍歷狀態）**，生命週期結束後可以釋放。它與 Buffer Pool／Page Cache 的用途不同：後者快取資料頁內容，Breadcrumbs 則記錄搜尋路徑。Parent Pointer 是另一種向上定位方式，但當子節點因 Split、Merge 或 Rebalancing 換了父節點，就需要維護對應關係。兩者可以並用，卻不是都必須存在；若沒有保留路徑，也沒有 Parent Pointer，仍可從 Root 重新尋找父節點。

**PostgreSQL 的 BTStack**：PostgreSQL 的 B-Tree 使用 `BTStack` 保存下降路徑，底層由 `BTStackData` 串接成 Linked List，作為回溯用的 Stack。`bts_blkno` 記錄 Parent Page 的 Block Number，`bts_offset` 記錄該頁所選 Downlink 的位置，`bts_parent` 則指向更上一層的 Stack 紀錄。這個 `bts_parent` 屬於暫存的路徑結構，並不是持久化在 Child Page 內的 Parent Pointer。

`_bt_search()` 透過 `returnstack` 決定是否建立並回傳 Stack；呼叫端不需要向上回溯時，可以省下這份配置，因此不是每次搜尋都會建立 `BTStack`。並行的 Page Split 或 Page Deletion 可能使紀錄的位置過期，使用時仍須重新確認；Breadcrumbs 提供定位線索，不保證樹的結構一直不變。


---

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

**Overflow Pages**：B-Tree 的 Node 通常以固定大小的 Page 儲存，但每個 Record 的 Payload 可能是變長的（Variable-Size Payload）。因此，**Key 數量尚未達到概念上的上限，不代表 Page 還有足夠的 Free Space**；少數幾筆很大的 Record，就可能先把 Page 的位元組空間用完。實際上，對變長資料而言，Node 能容納多少 Key 也會受到每個 Cell 大小的影響，不能只看 Key 的數量。

當單筆 Record 太大，可以將部分 Payload 保留在 Primary Page，超出的部分放到 Overflow Page，再由原本的 Cell 透過 Pointer 找到剩餘內容。這樣就不必單純因為一筆 Record 太大，而把它全部擠在同一個 B-Tree Node 裡；即使進行 Node Split，也不一定能解決單筆 Record 本身大於 Page 可容納空間的問題。

![[Assets/Note/Tech/Database Internals/overflow-pages.png]]

圖中的 Payload1、Payload2 都有一部分留在 Primary Page，其餘部分延伸到 Overflow Page。若額外內容仍放不下，多個 Overflow Pages 可以透過 `Page ID` 串接成 Linked List，每頁記錄下一個 Overflow Page 的位置，讀取完整 Record 時再沿連結取得後續 Payload。

`max_payload_size` 可以用來表示單筆 Payload 允許直接存放在 B-Tree Node 內的大小上限，超出這個限制的部分改存到 Overflow Page。實際門檻、欄位名稱與保留在原頁面的大小，仍依儲存引擎的實作而定。

**Overflow Page 與 Node Split 的用途不同**：Overflow Page 延伸的是單筆 Record 的儲存空間，不是 B-Tree 的 Child Node，也不會直接增加樹高；Node Split 則是把索引節點的內容分配到兩個節點，並更新父節點的分隔 Key 與 Child Pointer。把大型 Payload 移到 Overflow Page 可以減輕原頁面的空間壓力，但若原頁面仍放不下新的 Cell 或索引項目，依然可能需要 Split。

**Overflow Pages 的讀取成本與 BLOB Storage**：Overflow Pages 解決的是 Variable-Size Payload 超過 Primary Page 可用空間的問題，但完整讀取大型 Record 時，需要額外存取 Overflow Pages，並沿 `Page ID` 逐頁追蹤（Pointer Traversal）。若相關頁面尚未快取在記憶體中，就可能增加 Disk I/O；即使已有快取，仍有走訪連結的處理成本。

當大量 Records 都具有大型 Payload 時，可以考慮專門的 BLOB（Binary Large Object）Storage，讓大型內容由適合的儲存機制管理，索引或主要 Record 則保留對應的識別資訊。是否採用獨立的 BLOB Storage，仍取決於資料庫本身的支援方式與讀寫需求。

---

**Transaction Processing and Recovery（交易處理與恢復）**：交易處理需要考慮 ACID，也就是 Atomicity（原子性）、Consistency（一致性）、Isolation（隔離性）與 Durability（持久性）。這一段先從資料頁如何在記憶體與磁碟之間移動開始，理解快取中的修改何時寫回，以及頁面空間不足時如何處理。

**Buffer Pool 與 Page Cache**：資料庫不會每次存取 B-Tree Page 都重新讀取 Disk，而是把部分頁面保留在記憶體中。Buffer Pool 是 DBMS 管理資料頁快取的空間；書中也可能以 Page Cache 描述這類功能，但必須與前面提到的 **OS Page Cache** 區分，後者由 Kernel 管理。使用一般 Buffered I/O 時，同一份檔案內容可能同時存在於 OS Page Cache 與 DBMS Buffer Pool，形成兩層快取。

這種把部分資料留在記憶體、需要時再從磁碟載入的思路，可以類比 Virtual Memory。DBMS 自行管理 Buffer Pool，也不代表它必須使用 `mmap()`，或讓 OS 直接決定所有 Database Pages 的去留。

![[Assets/Note/Tech/Database Internals/buffer-pool-and-disk.png]]

圖中的 B-Tree Pages 對應到 Disk 上的頁面，Buffer Pool 則只容納其中一部分。虛線表示頁面與快取位置之間的對應；讀取某個節點時，若頁面已在 Buffer Pool，就能直接使用，否則才需要載入。圖示本身沒有標示哪些頁面是 Dirty，不能只依灰色填色判斷。

**Paged In、Dirty、Flushed 與 Evicted**：將需要的頁面從 Disk 載入記憶體，稱為 Paged In。當記憶體中的頁面被修改，而變更尚未寫回對應的儲存位置時，會標記為 **Dirty Page**。將變更寫回的動作稱為 Flush；若寫回期間沒有新的修改，頁面才可以視為 Clean。

Flush 是寫回，Eviction 是移出快取。Flush 後的頁面仍可留在 Buffer Pool，供後續存取；當 Buffer Pool 需要騰出空間，才由 Replacement Policy 選擇可移除的頁面，將其 Evict，重用該 Buffer Frame。Clean Page 通常可以直接移除，Dirty Page 則必須先妥善保存修改，不能直接丟棄；寫回也可以由背景程序提前進行，不必等到 Buffer Pool 完全滿了才開始。

例如某個 Leaf Page 載入後被修改，就會變成 Dirty；背景程序可以先把它 Flush，再繼續留在快取中。之後需要空間時，才將這個已經 Clean 的頁面 Evict。因此，Dirty、Flush 與 Eviction 描述的是不同狀態與動作，不是一發生修改就立刻寫回並移除。

**`O_DIRECT` 與自行管理快取**：部分 DBMS 在 Linux 上開啟檔案時使用 `O_DIRECT`，讓資料 I/O 通常繞過 OS Page Cache，直接在儲存裝置與 User-Space Buffer 之間傳輸，減少重複快取，並讓 DBMS 自行安排頁面的快取與讀寫。這並不是繞過整個 Kernel；實際支援與記憶體位址、I/O 長度、File Offset 的對齊要求，仍依檔案系統與裝置而定。

**`O_DIRECT` 本身不代表 Async I/O，也不單獨保證資料已持久化。**是否非同步取決於使用的 I/O 介面；是否達到持久化要求，則還需要考慮同步與儲存裝置的語意。它的取捨是把更多快取與預讀工作交給 DBMS，也增加實作與調校的複雜度；不能直接歸納為所有情境都比較快。參考 [Linux `open(2)`：`O_DIRECT`](https://man7.org/linux/man-pages/man2/open.2.html)。

**Read-Ahead 與 `posix_fadvise()`**：DBMS 可能比 Kernel 更清楚接下來會做順序掃描、隨機查找，或讀取哪些 Pages。使用 Buffered I/O 時，可以透過 `posix_fadvise()` 提供存取模式提示，例如 `POSIX_FADV_SEQUENTIAL`、`POSIX_FADV_RANDOM`；`POSIX_FADV_WILLNEED` 可提示近期將使用的範圍，`POSIX_FADV_DONTNEED` 則可請求釋放不再需要的快取。

這些是給 Kernel 的 **Advice（提示）**，不是完全接管 OS Page Cache 的 Replacement Policy，也不保證指定頁面立即被 Evict。`O_DIRECT` 路徑不依賴一般 OS Page Cache 的預讀，DBMS 仍可自行安排預讀與非同步請求。參考 [Linux `posix_fadvise(2)`](https://man7.org/linux/man-pages/man2/posix_fadvise.2.html)。

**Memory Mapping（記憶體映射）**：`mmap()` 可以把檔案映射到 Virtual Address Space，讓程式透過記憶體存取方式讀取內容；一般檔案映射的載入、Page Fault 與快取回收，主要由 OS 的虛擬記憶體與 Page Cache 機制處理。它能省去部分顯式讀取與複製工作，但 DBMS 對載入、寫回與回收時機的直接控制較少。仍可透過相關 Advice 介面提供提示，因此不是完全沒有控制方式。這是另一種存取設計，不代表應把同一檔案範圍的 `mmap()` 與 `O_DIRECT` 混用。參考 [Linux `mmap(2)`](https://man7.org/linux/man-pages/man2/mmap.2.html)。

**Page Replacement 與 Locking Pages in Cache**：Buffer Pool 容量有限，Replacement Policy 要決定哪些頁面繼續保留、哪些可以移除。比較直接的做法，是讓少量高頻使用的頁面常駐快取，避免反覆讀取 Disk。B-Tree 的 Root 與靠近 Root 的 Internal Pages 通常會被許多查找共用，因此可能比單一 Leaf Page 更值得保留，減少重複 I/O。

這裡的 Locking Pages in Cache 是指避免頁面被置換，不是交易處理中用來控制並行存取的 Lock。實作時也要區分 **Pin／Unpin**：Pin 通常保護目前正在使用的 Buffer Frame，使用完後 Unpin，才有機會成為置換候選；長期保留熱門頁面則是另一項快取策略。也不能直接把 DBMS 的 Pin 等同於 OS 的 `mlock()`，兩者管理的層次不同。

---

**FIFO、LRU 與 Belady’s Anomaly**：FIFO（First-In, First-Out）依照頁面進入快取的先後順序淘汰，Hit 不會改變排隊順序；LRU（Least Recently Used）則淘汰最久沒有被存取的頁面，Hit 時也會更新最近使用順序。LRU 關心的是「最近何時使用」，不是單純累計使用次數。

**Belady’s Anomaly（Belady 異常現象）**是指在相同參考序列與置換規則下，增加可容納頁面的 Frame 數量，Miss／Page Fault 反而變多。這裡增加的是 **Page Frames 的數量，不是 Page Size**。例如從空快取開始，依序存取 `1, 2, 3, 4, 1, 2, 5, 1, 2, 3, 4, 5`，FIFO 使用 3 個 Frames 時有 9 次 Miss，增加到 4 個 Frames，反而有 10 次。

這裡要區分 OS 與 DBMS 的用語：作業系統教材以 Page Fault 計算缺頁；DBMS Buffer Pool 找不到資料頁時，較精確的說法是 **Buffer Pool Miss**，不代表 CPU 一定發生 OS Page Fault。以下置換範例都以 Hit／Miss 表示是否在所討論的快取中找到頁面。

**Stack Property（堆疊性質）**：對相同參考序列，若容量為 N 的快取內容始終包含於容量 N+1 的快取內容中，增加容量就不會增加 Miss。精確 LRU 具有這個性質；OPT（Optimal Page Replacement）則利用未來資訊，淘汰下一次最晚才會使用的頁面，最優 Miss 數也不會因容量增加而變多。FIFO 不具備這項保證，因此可能出現 Belady’s Anomaly。OPT 需要預知未來，主要用作比較基準。

**LRU 的 Cache Pollution 與 2Q**：LRU 能保留近期使用的頁面，但一次性的大量順序掃描，也可能把原本常用的頁面擠出快取，形成 Cache Pollution（快取污染）或 Sequential Flooding。2Q（Two-Queue Algorithm）把新進頁面與已確認會再次使用的頁面分開管理，避免每個只讀一次的頁面都立即取得主要快取的地位。**2Q 是另一種置換策略，不是精確 LRU 的兩個 Queue 實作，也不能直接沿用 LRU 的 Stack Property 保證。**

**Simplified 2Q 與 Full 2Q**：簡化版使用 FIFO 的 `A1` 與 LRU 的 `Am`，頁面在 `A1` 期間再次被存取，就可升到 `Am`。原論文的 Full 2Q 則拆成 `A1in`、`A1out` 與 `Am`：`A1in` 以 FIFO 保存新進頁面；`A1out` 是 Ghost Queue，只保留已從 `A1in` 淘汰的 Page ID；`Am` 以 LRU 保存通過再次存取判斷的實際頁面。Ghost Queue 仍有少量識別資訊的記憶體成本，但不占用存放完整資料頁的 Frames。

Full 2Q 命中 `A1in` 時，保留 FIFO 順序，不直接升到 `Am`，避免把同一波短時間內的重複存取誤認為長期熱門。若頁面已離開 `A1in`，但 Page ID 仍在 `A1out`，再次存取時才重新載入到 `Am`；此時雖然命中 Ghost 紀錄，**仍是資料頁的 Miss**。命中 `Am` 時，才依 LRU 規則更新到 MRU（Most Recently Used）端。若 Ghost 紀錄也已消失，就當成新進頁面處理。

**Full 2Q 的空間回收規則**：有空閒 Frame 時先使用，不會只因 `A1in` 達到某個固定格數，就讓其他空間閒置。整個 Buffer 已滿時，原論文依 `A1in` 的大小與 `Kin` 門檻決定回收來源：`A1in` 大於門檻時，淘汰其 FIFO 最舊頁並把 ID 放入 `A1out`；否則淘汰 `Am` 的 LRU 頁，這個頁面不加入 `A1out`。Ghost 紀錄超過 `Kout` 時，刪除最舊 ID。因此，`Kin` 是選擇回收來源的門檻，不能直接當成不可借用的固定分區容量。參考 [Johnson 與 Shasha：2Q 原論文，Section 2](https://www.cs.cmu.edu/~natassa/courses/15-721/papers/P439.PDF)。

**對話中的 2Q 演算：固定分區教學變體**：原對話使用 4 個 Frames，將 `A1in` 與 `Am` 各限制為 2 個資料頁，`A1out` 最多記錄 2 個 ID。這個假設可示範 FIFO、Ghost 與 LRU 的互動，卻不是原論文共用空閒 Frames 的完整回收規則。以下保留這個固定分區假設；所有 Queue 都以左邊最舊、右邊最新表示，存取序列為 `A, B, A, C, A, D, B, E, C, B`。

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

例如頁面 A 離開 Admission Window，估計頻率高於 Probation 中的候選 B，就可以取代 B、進入 Probation；之後 A 再次被存取，就升到 Protected。若 Protected 已滿，其中最久沒有被使用的 C 會降回 Probation，仍有機會再次命中後升級。**TinyLFU 的頻率比較決定是否准入，SLRU 的命中與容量規則決定升降級**，兩者合作，並非三個 Queue 都按 LFU 排序。參考 [Caffeine：Window TinyLFU](https://github.com/ben-manes/caffeine/wiki/Efficiency#window-tinylfu)。

---

**WAL、Write-Ahead Log 與 Commit Log**：WAL 可以指 Write-Ahead Logging（預寫式日誌機制），也可以指 Write-Ahead Log（日誌本身）。它的核心規則是：**資料頁寫入持久化儲存之前，必須先持久化足以恢復該頁修改的相關日誌。**資料可以先在 Buffer Pool 中被修改，不必每次都先同步日誌才允許修改記憶體；必須遵守的是 Log 與 Data Page 的持久化順序。

部分系統把這類日誌稱為 Commit Log，例如 Cassandra；名稱仍依系統而定，不能因此理解成日誌只包含已提交交易，或只在 Commit 時才產生記錄。對支援交易的儲存引擎而言，交易進行中的修改與提交狀態，都可能需要記錄。參考 [Cassandra：Storage Engine](https://cassandra.apache.org/doc/stable/cassandra/architecture/storage-engine.html)。

**帳戶 A：未提交的扣款，卻已經寫到磁碟**：假設帳戶 A 的餘額原本是 1,000 元，交易 T1 在 Buffer Pool 中把它改成 900 元，但尚未 Commit。系統先把足以恢復這次修改的日誌持久化，再把包含 900 元的資料頁提前寫回磁碟，以便騰出 Buffer Frame。此時 DB 突然當機，記憶體中的 Buffer Pool 消失，磁碟上的 A 卻已是 900 元。因為 T1 沒有完成提交，這次扣款不能成為有效結果，恢復時必須撤銷其影響，讓 A 回到 1,000 元。

**帳戶 B：入帳已提交，資料頁卻還沒寫到磁碟**：另一筆獨立交易 T2 把帳戶 B 從 2,000 元改成 2,500 元。系統已持久化必要修改日誌與 Commit Record，並回覆提交成功，但資料頁仍只在 Buffer Pool，磁碟上還是 2,000 元。此時 DB 當機，新的 Buffer Pool 內容雖然消失，T2 的提交結果仍應保留，因此要利用持久化日誌重做修改，讓 B 恢復成 2,500 元。這兩個例子假設 A、B 位於不同資料頁，且沒有其他交易修改這兩個值，方便分開理解。

**STEAL 與 NO-STEAL**：STEAL 允許把含有未提交修改的 Dirty Page 提前寫回磁碟，並重用它的 Buffer Frame；它是允許這樣做，不是要求所有修改立即寫回。帳戶 A 就是這種情況。NO-STEAL 則不允許含有未提交修改的頁面以這種方式寫回。因此，STEAL 的好處是減少未完成交易對 Buffer 空間的占用，代價是在傳統恢復模型中需要 UNDO，處理已落盤的未提交修改。

**FORCE 與 NO-FORCE**：FORCE 要求交易完成提交時，其修改的資料頁都已寫回持久化儲存；NO-FORCE 允許提交完成後，資料頁仍留在 Buffer Pool，稍後才寫回。帳戶 B 就是 NO-FORCE 的情況。NO-FORCE 可以降低提交時等待資料頁 I/O 的成本，但在傳統恢復模型中需要 REDO，補回已提交卻尚未落盤的修改。**FORCE 討論的是資料頁，不是只把 WAL Flush。**

**為什麼可以同時使用 STEAL + NO-FORCE？**：這組政策允許磁碟上的資料頁暫時包含未提交修改，也暫時缺少已提交修改，因此不能把資料頁本身當成完整的交易結果。正確性要由資料頁、持久化日誌、交易狀態與恢復機制共同保證。關鍵是 WAL 的順序：資料頁落盤之前，相關恢復資訊必須先持久化；在本例要求的同步持久化提交模式下，回覆提交成功之前，必要修改日誌與 Commit Record 也必須先持久化。日誌只留在記憶體或 OS Page Cache，都不足以提供這項保證。

**REDO 為什麼在 Buffer 消失後仍能作用？**：DB 重新啟動會建立新的 Buffer Pool，將磁碟上的資料頁讀進來，再利用持久化 WAL 中的重做資訊，重新套用必要修改。帳戶 B 原本的 Buffer 雖然已經消失，磁碟資料頁與 WAL 仍在，所以可以從磁碟上的 2,000 元重建出 2,500 元，再於適當時機寫回。重做資訊可能是新值、頁面變更或操作描述，依日誌格式而定；不需要找回當機前的記憶體。

**UNDO 如何知道要撤銷什麼？**：以同時保存重做與撤銷資訊的 ARIES 類型日誌為例，系統會從日誌與交易狀態辨識未完成交易，再沿該交易的修改紀錄反向撤銷。帳戶 A 的日誌可簡化理解為「T1 將 A 從 1,000 改成 900」，因而有足夠資訊取消這次扣款。恢復時可把磁碟上的 900 元讀進新的 Buffer Pool，套用撤銷，再寫回；不是只拿 WAL 與資料頁比較數值，也不是看到 900 就憑空猜原值。不同引擎可能使用獨立 Undo Log 或版本可見性機制，不能把這個流程套到所有 DBMS。

**ARIES 是什麼？**：ARIES 的全名是 **Algorithms for Recovery and Isolation Exploiting Semantics**，可理解為「利用操作語意的恢復與隔離演算法」。它是一套以 WAL 為基礎、支援 STEAL + NO-FORCE 的交易恢復設計，不是另一種 Log 名稱，也不是所有 DBMS 都完整採用的固定流程。它將當機恢復分成 **Analysis、REDO、UNDO** 三個階段。

**Analysis：先辨識恢復範圍與交易狀態**：重新啟動後，從 Checkpoint 及後續日誌重建交易與 Dirty Pages 的追蹤資訊，辨識哪些交易已提交、哪些仍待撤銷，以及日誌應從哪裡開始重做。以上述例子而言，T1 是未完成交易，T2 則已提交。Checkpoint 能縮小掃描範圍，不代表它一定是所有資料頁都完全寫回的瞬間快照。

**REDO：先重建日誌描述的必要歷史**：ARIES 的 REDO 採 Repeating History，會重做必要的已記錄操作，可能包含未提交交易的修改與先前的撤銷紀錄；它不是只挑已提交交易來重播。已反映在資料頁上的修改，可利用 `pageLSN` 等資訊判斷並跳過，避免重複套用。帳戶 A 的 900 元若已在磁碟且頁面進度足夠，就不必再做相同修改；帳戶 B 尚未反映的 2,500 元則需要補回。

**UNDO：再移除未完成交易的影響**：完成必要的重做後，再撤銷像 T1 這樣未完成交易的修改。這個簡化例子的最終結果，就是 A 回到 1,000 元，B 保留已提交的 2,500 元。UNDO 的目的不是讓未提交交易繼續執行到 Commit，而是取消它不應留下的效果。

**恢復途中再次當機怎麼辦？**：ARIES 在執行撤銷時，會產生 Compensation Log Record（CLR，補償日誌紀錄），記錄已執行的撤銷動作與接下來的撤銷位置，並遵守 WAL 的持久化順序。若恢復過程再次當機，下次可重做已記錄的補償動作，繼續尚未完成的撤銷，避免把同一項修改錯誤地撤銷兩次。參考 [CMU：ARIES 與 Database Recovery](https://15445.courses.cs.cmu.edu/fall2017/notes/22-notes-recovery.pdf)。

**LSN（Log Sequence Number）與持久化進度**：LSN 用來識別日誌中的位置或順序，例如 PostgreSQL 使用 WAL 串流中的位元組位置；它不是 Transaction ID，也不是時間戳。系統可以分別追蹤已產生日誌的位置、已寫出的位置，以及已確保持久化的位置。將 Log 寫到 OS Page Cache，不等於已經 Flush 到能承受當機或斷電的持久化儲存。參考 [PostgreSQL：WAL Internals](https://www.postgresql.org/docs/current/wal-internals.html)。

**Page LSN 與 Flushed LSN**：以常見的恢復模型說明，`pageLSN` 表示目前頁面所反映的最新修改日誌位置，`flushedLSN` 表示日誌已持久化到的進度。寫回某個 Dirty Page 之前，必須確保相關日誌已完整持久化，概念上可寫成 **`flushedLSN ≥ pageLSN`**。這個比較沿用教科書把 LSN 視為記錄序號的簡化方式；若實作用 LSN 表示位元組位置，則還必須涵蓋相關記錄的結尾，不能只寫到記錄開頭。

例如某頁面最新修改的日誌序號為 120，而 `flushedLSN = 100`，就不能先把包含該修改的頁面寫回；必須先持久化涵蓋記錄 120 的日誌，再寫回資料頁。因此，「Flush 到目標 LSN」表示確保**直到該目標所需的日誌前綴都已持久化**，不是只刷新單筆記錄，也不是把所有資料頁都刷新到同一個 LSN。日誌已持久化到哪裡，也不代表恢復程序已經重播到哪裡，兩者是不同的進度。

**Buffer Flush 與 Commit 的關係**：資料頁 Flush 不等於交易 Commit，Commit 也不一定需要立即 Flush 所有 Dirty Pages。在要求本機同步持久化的提交模式下，通常要先將涵蓋該交易必要修改與 Commit Record 的日誌持久化，再向 Client 回報提交成功；資料頁可以稍後寫回，故障時再依日誌恢復。非同步提交模式則可能先回覆，因此可能在故障後遺失近期已回覆的交易，須依實際設定判斷。參考 [PostgreSQL：Write-Ahead Logging](https://www.postgresql.org/docs/current/wal-intro.html)。

---

**Concurrency Control（並行控制）**：多筆交易可能交錯讀寫相同資料，因此需要協調它們能看到什麼、何時可以修改，以及衝突時應等待還是重試。Transaction 是操作的交易單位，不等於悲觀鎖；OCC、PCC 與 MVCC 也不是完全互斥的三選一分類。

**OCC（Optimistic Concurrency Control，樂觀並行控制）**：先執行讀取與計算，再驗證是否與其他交易衝突；未通過就撤銷或重試，而不是先把未驗證的結果正式提交。應用層常見方式是為資料加上會隨修改變動的 `version` 或可靠的版本時間戳。固定不變的 Primary Key 只能識別資料列，不能單獨識別它是否被修改。

例如讀到 `balance = 1000, version = 5`，更新時可以執行 `UPDATE accounts SET balance = 900, version = version + 1 WHERE id = 1 AND version = 5`。若版本已被別人改變，就無法匹配原版本，需要處理衝突；沒有匹配也可能是資料列已被刪除。**版本條件必須和更新放在同一個原子操作中**，不能先查版本、再無條件更新。這是單列樂觀並行控制的例子，不代表已驗證整筆交易的所有讀寫依賴。參考 [Microsoft：Optimistic Concurrency](https://learn.microsoft.com/en-us/ef/core/saving/concurrency)。

**PCC（Pessimistic Concurrency Control，悲觀並行控制）**：先取得必要的鎖，再進行受保護的操作。例如在交易中以 `SELECT ... FOR UPDATE` 鎖住目標資料列，其他衝突操作通常需要等待，直到鎖釋放。若 T1 持有 A 的鎖並等待 B，T2 卻持有 B 的鎖並等待 A，就可能形成 Deadlock；DB 通常需要中止其中一筆交易，應用再依需求重試。悲觀控制不代表鎖住整個 DB，鎖的範圍與生命週期依協定而定。參考 [PostgreSQL：Explicit Locking](https://www.postgresql.org/docs/current/explicit-locking.html)。

**OCC／PCC 的選擇：看衝突程度與成本**：讀多寫少的工作負載常適合 OCC，但核心不是只看讀寫比例，而是資料競爭程度（Contention）。大量寫入若分散在不同帳戶，仍可能很少衝突；整體寫入不多，卻集中搶同一筆庫存，也可能高度競爭。衝突少、重試成本低時，OCC 可以減少等待；衝突多、反覆重做成本高時，PCC 可能較合適，但也要付出鎖管理、等待與 Deadlock 處理的成本。這是選擇方向，不是所有實作與工作負載都成立的效能定律。參考 [Microsoft：Concurrency Control](https://learn.microsoft.com/en-us/sql/relational-databases/sql-server-transaction-locking-and-row-versioning-guide)。

**MVCC（Multi-Version Concurrency Control，多版本並行控制）**：保留資料的多個版本，依 Snapshot 與隔離規則，決定交易能讀到哪個版本。單純增加一個 `version` 欄位檢查更新衝突，不等於實作 MVCC；MVCC 還涉及版本可見性與舊版本回收。它可以搭配樂觀驗證或鎖定，採用 MVCC 也不代表寫入永遠不等待、沒有 Deadlock，或自動具備 Serializable。參考 [PostgreSQL：MVCC](https://www.postgresql.org/docs/current/mvcc-intro.html)。

**Serializable（可序列化）與串行順序**：Serializable 要求已提交交易的並行執行效果，等價於某個逐筆執行的合法順序。交易實際上仍可以交錯執行；要檢查的不只是最後的資料值，也包含交易讀到了什麼、依此做出哪些操作。OCC 與 PCC 都能透過適當設計實現這項保證，但單列版本檢查或局部鎖定不自動保證整筆交易可序列化。應用是否需要選擇 Serializable，取決於要保護的規則與其他協調方式。

假設有 T1、T2、T3，忽略其他執行限制，串行排列有 `T1→T2→T3`、`T1→T3→T2`、`T2→T1→T3`、`T2→T3→T1`、`T3→T1→T2`、`T3→T2→T1`，共 `3! = 6` 種。這是交易整體的串行順序，不是全部並行交錯方式，也不代表一定有 6 種不同結果；每筆交易內有多個步驟時，交錯方式可能更多。

**DB 怎麼知道哪個順序才對？**：DB 不需要先選定唯一正確順序，而是透過鎖定或依賴驗證，確保提交的執行歷史能對應到某個合法串行順序。若無法滿足，就讓衝突操作等待，或撤銷交易並要求重試。Serializable 本身也不保證先開始的交易一定先執行完畢或先取得資源。

**庫存只剩一件的例子**：T1、T2 都想購買同一件商品，且交易邏輯正確檢查庫存。如果效果等價於 `T1→T2`，T1 可以買到，T2 看到庫存不足；等價於 `T2→T1` 則相反，兩者都合法。若兩筆交易都因讀到庫存 1 而購買成功，就無法等價於這兩種串行執行中的任何一種。OCC 可以利用原子版本檢查或適當驗證讓其中一筆失敗，重試後重新檢查庫存；PCC 可以先鎖住庫存列，讓另一筆取得鎖後再檢查。Serializable 協調並行效果，交易本身仍要寫對庫存檢查與更新邏輯。參考 [PostgreSQL：Transaction Isolation](https://www.postgresql.org/docs/current/transaction-iso.html)。

**Serializable 與 Eventual Consistency（最終一致性）不同**：前面討論的是交易隔離，不能概括為「DB 最後在意的都是最終一致性」。Eventual Consistency 通常用來描述多副本資料：若沒有新的更新，且更新傳播與衝突處理能持續完成，各副本最終會收斂；在此之前，讀取可能看到舊資料。它本身不保證多筆交易的執行歷史可序列化，也不保證庫存不超賣。例如兩個副本最後都同步成同一個超賣結果，仍不符合庫存業務規則。參考 [AWS：DynamoDB Read Consistency](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadConsistency.html)。

**ACID 的 Consistency 也要分開理解**：ACID 的一致性是交易應維持資料約束與業務不變條件；Serializable 是交易隔離的保證；Eventual Consistency 則描述副本或讀取結果的收斂性質。它們討論的層次不同，可以在同一個系統中分別設計。例如主庫內的交易可要求 Serializable，同時以非同步方式把結果傳給唯讀副本；從副本讀取仍可能落後主庫。
