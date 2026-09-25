# Lamport's Bakery Algorithm (LBA)

- source: `raw/my-vault/Note/Research/Lamport's Bakery Algorithm (LBA).md`
- source_sha256: `eb7ab042d71d05c14622b7f13149c0b9efda5dee6909dd50ebaabb61028c70e5`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Core Concept、Lexicographical Order (優先權比較規則)、Shared Data Structures、Algorithm Implementation、Correctness Proof、A. Mutual Exclusion。

## Source Notes

- 由 Leslie Lamport 於 1974 年提出，這是一個具有里程碑意義的演算法。它證明了在沒有硬體原子指令 (Atomic Instructions) 的支援下，僅使用最基礎的 安全暫存器 (Safe Registers) 讀寫操作，就能解決 $N$ 個 Process 的互斥問題。
- 演算法的核心思想源自麵包店的取號機制，並嚴格遵循 先到先服務 (FCFS) 原則。
- 系統使用 (ticket #, process ID) 的來決定優先順序。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
