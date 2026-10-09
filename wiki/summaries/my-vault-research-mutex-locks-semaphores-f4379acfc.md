# Mutex Locks & Semaphores

- source: `raw/my-vault/Note/Research/Mutex Locks & Semaphores.md`
- source_sha256: `882a6a533813283a11a6fc515c404d73ae216ea11f966b90995a63ebd81bc97e`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Hardware-based Mutex Implementation、A. Test-and-Set (TAS)、Instruction Definition、Application: Simple Spinlock (Algorithm 1)、Proof of Correctness、B. Compare-and-Swap (CAS)。

## Source Notes

- 現代 Mutex 通常依賴 Atomic Hardware Instructions 來保證操作的不可分割性。
- 這是最基礎的 Mutex 實作，採用 Busy Waiting。
- 為了修正上述硬體鎖缺乏 Bounded Waiting 的問題，引入 waiting[] 陣列來建立排隊機制。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：1. Hardware-based Mutex Implementation、A. Test-and-Set (TAS)、Instruction Definition、Application: Simple Spinlock (Algorithm 1)、Proof of Correctness、B. Compare-and-Swap (CAS)、Instruction Definition、Application: Simple Spinlock、Proof of Correctness、C. Bounded-waiting Mutex (Algorithm 2)、Code Implementation、Proof of Correctness、2. Mutex Locks (High-Level Tool)、定義與特性、Code Structure、3. Semaphores (Robust Tool)、A. Operations、B. Implementation Strategies、1. Busy-waiting (Binary Semaphore) Implementation、2. Counting Semaphore Implementation、3. Non-busy Waiting Implementation (Block/Wakeup)、4. Semaphore Implementation Strategies (Construction)、A. Non-busy Waiting Semaphores (Block/Wakeup)、Algorithm 1: Disable Interrupt、Algorithm 2: Hardware Instructions (TAS/CAS)、B. Busy Waiting Semaphores (Spinlocks)、Algorithm 3: Disable Interrupt、Algorithm 4: Hardware Instructions (TAS/CAS)、Comparison Summary。
