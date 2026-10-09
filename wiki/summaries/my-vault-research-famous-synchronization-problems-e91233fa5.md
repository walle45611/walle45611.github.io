# Famous Synchronization Problems

- source: `raw/my-vault/Note/Research/Famous Synchronization Problems.md`
- source_sha256: `d5d782173b315e09c0f58e049d42cf4e6f9598352d5f7c0f3e4ad4e72e6bd23e`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Bounded-Buffer Problem (Producer-Consumer)、Shared Variables、Producer Process、Consumer Process、Readers-Writers Problem、A. First Readers-Writers Problem (Reader Priority)。

## Source Notes

- 此問題描述生產者 (Producer) 將資料放入有限大小的緩衝區，消費者 (Consumer) 從中取出資料。必須確保
- Mutual Exclusion: 存取 Buffer 時互斥。
- Synchronization: Buffer 滿時 Producer 等待，Buffer 空時 Consumer 等待。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：1. Bounded-Buffer Problem (Producer-Consumer)、Shared Variables、Producer Process、Consumer Process、2. Readers-Writers Problem、A. First Readers-Writers Problem (Reader Priority)、Shared Variables、Writer Process、Reader Process、B. Second Readers-Writers Problem (Writer Priority)、Shared Variables、Writer Process、Reader Process、C. Comparison Summary、3. Dining-Philosophers Problem、A. Semaphore Solution、B. Monitor Solution (Deadlock-free)、4. The Sleeping Barber Problem、Shared Variables、Barber Process、Customer Process。
