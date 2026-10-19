---
blog: true
blog_title: 'Unordered Log-Structured Storage：Bitcask 與 WiscKey'
blog_date: '2026-10-20'
blog_url: https://blog.walle4561.com/articles/posts/unordered-log-structured-storage/
blog_tag: DSA
blog_topic: 資料結構
---

## TL;DR

**Log-Structured 描述追加新紀錄的寫入方式；Unordered 則表示資料沒有依 Key 排序**。資料仍可按寫入順序存放，再由索引提供查找能力。

- **Bitcask**：磁碟保存 Append-Only Data Files，記憶體中的 KeyDir 直接指向每個 Key 的最新紀錄；代價是全部 Key 的索引資訊必須容納於 RAM，且沒有按 Key 排列的範圍掃描結構。
- **WiscKey**：Key 與位置資訊仍由有序 LSM 管理，Value 分離到無序 Value Log，減少 Compaction 搬移大型 Value 的成本；Value Log 需要另外回收過時資料。

LSS 的整體脈絡見 [Database Internals](https://blog.walle4561.com/articles/posts/database-internals-reading-notes/)，MemTable、SSTable 與 Compaction 的基本流程見 [LSM-Tree](https://blog.walle4561.com/articles/posts/lsm-tree/)。

---

## Unordered：不按 Key 排序，仍然有寫入順序

依序寫入 `key3、key1、key2、key3 的新值`，Append-Only 資料檔就依這個順序追加，不會重新排列成 key1、key2、key3。這裡的 Unordered 是相對於 Key 順序而言，不代表沒有任何順序。

Bitcask 是另一套儲存設計，不是把一般 LSM 的 SSTable 直接換成無序檔案；WiscKey 則是有序 Key LSM 與無序 Value Log 的混合，不能把它的全部元件都當成無序。

---

## Bitcask：KeyDir 與 Append-Only Data Files

### 資料存在哪裡？

| 位置 | 結構 | 保存內容 |
| --- | --- | --- |
| 記憶體 | KeyDir（Hash Table） | 每個 Key 最新紀錄的 File ID、Offset、大小等資訊 |
| 磁碟 | Data Files | 按寫入順序追加的實際 Key-Value 紀錄 |

![[Assets/Note/Tech/Unordered Log-Structured Storage/bitcask-keydir.png]]

圖中的 key3 曾寫入 v2、v9、v10，KeyDir 現在只指向 v10。灰色紀錄仍可能占用磁碟空間，但已不是查詢入口。

### 更新與查詢

1. **寫入**：將新 Key-Value 追加到 Active Data File，再更新 KeyDir 的位置資訊。檔案達到輪替條件後，關閉舊檔案，改由新檔案接收追加。
2. **查詢**：先查 RAM 中的 KeyDir，取得檔案與位置，再直接讀取紀錄；不必逐份檔案搜尋後比較版本。
3. **回收**：Merge 整理非 Active 檔案，保留有效紀錄並更新對應位置，回收過時內容。啟動時可透過資料檔與 Hint Files 重建 KeyDir。

KeyDir 不是 MemTable：它主要保存磁碟位置，不是等待 Flush 的完整更新集合。Bitcask 的資料檔本身也扮演日誌角色，不必另有一份獨立 WAL；是否同步持久化仍取決於寫入設定。設計與限制見 [Bitcask 原始設計](https://riak.com/assets/bitcask-intro.pdf)。

---

## WiscKey：Key–Value Separation

### Key 有序，Value 不必跟著排序

一般 LSM 的 Compaction 會重寫 Key 與 Value；Value 很大時，反覆搬移的成本也大。WiscKey 把兩者分開：**LSM 保存有序的 Key 與 Value 位置，實際 Value 追加到 Value Log（vLog）**。

![[Assets/Note/Tech/Unordered Log-Structured Storage/wisckey-key-lsm-vlog.png]]

圖中上方的 Index LSM 保存 Key 與 Value 位置，連到下方 vLog 的有效紀錄；灰色內容表示已被新版本取代的舊紀錄。

例如 Key LSM 中保存 `user:42 → vLog Offset 800`，該位置才是使用者資料。查詢先搜尋 Key LSM，再到 vLog 讀取 Value；更新則追加新的 Value，並讓 Key LSM 記錄新位置。Compaction 因此主要整理較小的 Key 與位置資訊，不必每次都帶著完整 Value 一起重寫。

### SSTable 仍在：保存 Key 與 Value 的位置

WiscKey 不是拿 vLog 取代整套 SSTable 機制。Key LSM 仍有 MemTable、Flush、SSTable 與 Compaction，只是原本的完整 Value 改成較小的位置資訊。例如 SSTable 裡可以理解成 `user:42 → Offset 800、長度 200 Bytes`，完整的使用者資料則保存在 vLog 對應的位置。

查詢先在 Key LSM 找出位置，再讀取 vLog；Key LSM 的 Compaction 主要重寫 Key 與位置資訊，不必反覆搬移完整的大型 Value。因此「只存 Key」是簡稱，實際還要保存定位 Value 的資訊。

### 與 Bw-Tree Delta Chain 的相似處

**兩者都有「寫到新位置，再更新查找入口」的間接定位思路**。WiscKey 將新 Value 追加到 vLog，再更新 Key LSM 中對應的位置；Bw-Tree 則把新 Delta 接到舊入口前端，再以 CAS 更新 Mapping Table，讓 Logical Page ID 指向最新的節點入口。

| | WiscKey | Bw-Tree |
| --- | --- | --- |
| 分開管理的內容 | Key 索引與完整 Value | Base Page 與後續 Delta Records |
| 映射的對象 | Key → vLog 中的 Value 位置 | Logical Page ID → 節點最新入口 |
| 讀取方式 | 查 Key LSM，再到 vLog 取 Value | 依需要解讀 Delta Chain 與 Base Page，重建節點有效內容 |
| 後續整理 | Key LSM 做 Compaction，vLog 另外 GC | Consolidation 整合 Delta Chain 與 Base Page，舊物件安全回收 |

因此可以借用 Bw-Tree 理解「資料不必原地修改，查找透過入口定位」的概念；但 **vLog 不是 Delta Chain**，WiscKey 並不是沿一串頁面修改紀錄還原 Value。Bw-Tree 的節點更新與 Consolidation 見 [B-Tree Variants](https://blog.walle4561.com/articles/posts/b-tree-variants/)。

### 成本移到哪裡？

Value Log 中的舊內容仍須回收：GC 檢查待清理紀錄是否仍被 Key LSM 指向，搬移有效內容、更新位置，確認安全後再回收舊區段。因此 Key 排序與 Value 回收是分開的工作。

### Range Query：Key 有序，不代表 Value 也連續

相較於使用 Hash Table KeyDir 的 Bitcask，WiscKey 保留有序的 Key LSM，因此能定位範圍起點，再按 Key 順序走訪。例如查詢 `user:10～user:20`，先在 Key LSM 找到起點，依序取得符合範圍的 Key 與位置，再到 vLog 讀取 Values。

但這不代表它的 Range Query 一定比一般 LSM 更快：

| | 一般 LSM | WiscKey |
| --- | --- | --- |
| Key 與 Value 的配置 | 在 SSTables 中一起按 Key 排列 | Key 與位置資訊有序，完整 Value 分散在 vLog |
| 範圍讀取 | 從相關 SSTables 順序讀取區塊，並整合結果 | 按序走訪 Key LSM，再讀取各個 Value 位置 |
| 主要成本 | 讀取多個 Runs、處理版本；完整 Value 也隨 Compaction 重寫 | Value 讀取可能需要位置跳轉，另須維護 vLog GC |
| 設計取捨 | 保持 Key-Value 的排序區域性 | 減少 Compaction 搬移大型 Value，同時保留按 Key 的範圍查詢能力 |

WiscKey 論文透過 SSD 平行讀取與預取減輕分散 Value 的讀取成本；效能仍取決於硬體、Value 大小與工作負載，不能只因 Key 有序就認定完整資料能連續讀取。參考 [WiscKey 原論文](https://www.usenix.org/conference/fast16/technical-sessions/presentation/lu)。
