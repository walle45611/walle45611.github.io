# Hardware Synchronization Implementation & Proofs

- source: `raw/my-vault/Note/Research/Hardware Synchronization Implementation & Proofs.md`
- source_sha256: `7de9e4c7e2c798a42ff1035b9a3a7a5b67b52f3af79fc1bc6a807b1dd12c5ad8`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Memory Barriers、Proof of Failure by Reordering、Correct Implementation、Test-and-Set (TAS)、Instruction Definition、A. Simple TAS Mutex ALGO1。

## Source Notes

- 若缺乏 Memory Barriers，Processor 可能對無 Data Dependency 的指令進行 Reordering (指令重排)。
- Initial State: flag = false, data = 0
- Thread 1 (Producer): 寫入資料，設定旗標。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：1. Memory Barriers、Proof of Failure by Reordering、Correct Implementation、2. Test-and-Set (TAS)、Instruction Definition、A. Simple TAS Mutex ALGO1、B. Bounded-waiting TAS ALGO2、3. Compare-and-Swap (CAS)、Instruction Definition、A. CAS Mutex (Simple)、B. Bounded-waiting CAS ALGO2、C. Atomic Integer Implementation、D. Proof of Limitation (Atomic Variables)。
