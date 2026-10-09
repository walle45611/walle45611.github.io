# SQL 查詢、分組與逐列判斷

## Current View

現有 SQL 筆記以 LeetCode 題目整理語句選擇。先辨識需求要保留哪些列、如何配對、是否聚合，以及輸出分類或修改資料，再選語句；不把單次 runtime 當成通用效能結論。

## Working Model

- **保留與配對**：`LEFT JOIN` 保留左表；反連接找沒有配對的列。Self join 讓同一表扮演不同角色。[[my-vault-tech-join-cecad0b21]]
- **分組與修改**：`GROUP BY` 組合相同值，`HAVING` 篩選群組；查出重複 email 與刪除重複列是不同任務。[[my-vault-tech-note-08db955d8]]
- **日期與視窗**：`LAG()` 的上一列不必然是昨天，#197 另檢查日期差為一天。[[my-vault-tech-note-8413de499]]
- **逐列分類**：#610 保留全部列並算出三角形標籤，用 `SELECT` 中的 `CASE WHEN`；`WHERE` 用來篩掉列。[[my-vault-tech-case-when-affdf4577]]

## Concrete Example

#511 的首次登入可用 self join 排除更早紀錄，或 `GROUP BY player_id` 搭配 `MIN(event_date)`。前者須選有資料的左表日期；兩種寫法的效能比較仍需計畫、索引與重複測量。

## Boundaries

內容限於現有練習筆記，MySQL 日期與 DELETE 寫法不能直接推定其他資料庫方言適用。SQL 主題與 Oracle 安裝入口見 [[my-vault-dashboard-sql-overview-bad041aa8]]，題號分類見 [[my-vault-dashboard-overview-4e3b3b280]]。[[data-structures-and-algorithms]] 提供相鄰的刷題入口，但程式演算法複雜度不等同資料庫查詢執行計畫。

## Sources

- [[my-vault-dashboard-sql-overview-bad041aa8]]
- [[my-vault-tech-join-cecad0b21]]
- [[my-vault-tech-note-08db955d8]]
- [[my-vault-tech-note-8413de499]]
- [[my-vault-tech-case-when-affdf4577]]

## 2026-10-09：相鄰主題：資料儲存架構

[[ddia-reliability-scalability-maintainability]] 補充 workload 驅動的 scale-up／scale-out 與 polyglot persistence。每日 batch 的銀行例子是來源假設；查詢語法頁不能證明跨資料庫原子性、即時一致性或失敗重試設計。系統設計中可維護性的關聯見 [[harness-engineering]]。
