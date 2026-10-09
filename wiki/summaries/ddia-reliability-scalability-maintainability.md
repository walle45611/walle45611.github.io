# DDIA：延遲、可維護性、擴展與多資料庫

- source: `raw/my-vault/Note/Tech/DDIA ：Designing Data-Intensive Applications.md`
- source_sha256: `a32a8f607756df8e6c8d5f3e610e7b6b314beb54a0b1089669431ccdb9354f16`
- source_created: not specified
- ingested_at: 2026-10-09
- type: source summary

## 摘要與關鍵資訊

閱讀筆記串連四個系統設計主題，重點是依 workload 與可靠性需求做取捨。

- 平均延遲會遮蔽少量很慢的 request；p50 是 median，p95／p99／p99.9 觀察 tail latency。需等待多個 backend 的請求會受最慢者影響，形成 tail latency amplification。
- SLI 是量測指標、SLO 是目標、SLA 是與客戶的正式約定；percentile 可描述多少比例的請求在指定時間內完成。
- Maintainability 包含 operability、simplicity、evolvability；內部設計易理解與 UI 簡單是不同面向。
- Shared-nothing 節點各有 CPU、memory、storage；scale-out 可逐步加節點，也引入 partitioning、replication、consistency、network failure 與 coordination 成本。沒有脫離 workload 的通用擴展方案。
- Polyglot persistence 依資料用途混合儲存技術。NoSQL 到 RDBMS 每日 batch 的銀行例子是筆記中的假設，不能當成已知銀行實作；同步延遲、原子性、耐久性、重複寫入與重試仍須處理。

## 來源邊界

來源是目前的讀書筆記，未提供完整書籍章節與頁碼，不代表已完成整本 DDIA 摘要。資料庫設計主題與既有 SQL 查詢頁相鄰，但查詢語法練習不足以決定多資料庫交易與同步架構。

## 相關概念

- [[database-and-sql]]
