# 作業系統：競爭條件與同步

- source: `raw/race-condition-and-synchronization.md`
- original title: 作業系統-競爭條件與同步
- author: Walle
- published: 2025-01-04
- source_created: 2025-01-04
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇文章從競爭條件與臨界區問題出發，整理 Peterson’s Solution、硬體原子指令、互斥鎖、自旋鎖、信號量、Monitors 與經典同步問題。

## Key Points

- Race condition 的結果取決於多個 process 存取共享資料的順序。
- 臨界區解法需要處理 mutual exclusion、progress 與 bounded waiting。
- 原文比較 Test-and-Set、Compare-and-Swap、Peterson’s Solution、Mutex、Spinlock 與 Semaphore 的角色。
- 後半段延伸到 liveness、priority inversion、producer-consumer、readers-writers 與 dining philosophers。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
