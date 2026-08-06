# 作業系統：死結管理

- source: `raw/deadlock-management.md`
- original title: 作業系統-死結管理
- author: Walle
- published: 2025-01-06
- source_created: 2025-01-06
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇文章整理死結的系統模型、定義與處理策略，從四個必要條件延伸到預防、避免、檢測與恢復，並介紹 Banker’s Algorithm 與資源分配圖。

## Key Points

- 執行緒使用資源時會經過 request、use、release 的生命週期。
- 死結需要同時具備 mutual exclusion、hold and wait、no preemption 與 circular wait 等條件。
- Deadlock prevention、avoidance、detection and recovery 是不同層次的處理方法，選擇取決於系統可接受的成本與保證程度。
- Banker’s Algorithm 與 resource-allocation graph 可用來判斷安全狀態或分析循環等待。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
