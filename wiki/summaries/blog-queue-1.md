# 資料結構：Queue

- source: `raw/queue-1.md`
- original title: 資料結構-Queue篇-1
- author: Walle
- published: 2024-09-09
- source_created: 2024-09-09
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇資料結構筆記整理 Queue 的 FIFO 性質、Queue ADT、線性陣列的限制、Circular Queue 與 linked list 實作，並附上 enqueue/dequeue 程式碼。

## Key Points

- Queue 的插入與刪除位於相反兩端，front 與 rear 分別代表兩個操作位置。
- Linear array 在元素移除後可能產生空間利用問題，Circular Queue 用索引回繞改善此限制。
- 文章以 Queue CreateQ、IsFullQ 等操作定義抽象介面，再對照陣列與鏈結串列實作。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
