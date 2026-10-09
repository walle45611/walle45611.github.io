# Lamport's Bakery Algorithm (LBA)

- source: `raw/my-vault/Note/Research/Lamport's Bakery Algorithm (LBA).md`
- source_sha256: `9347316fbbb327cf4d6f24f013a88b3fc582ab25abfb19ae0eb7185365cd812b`
- source_reviewed: 2026-10-09
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

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：1. Core Concept、Lexicographical Order (優先權比較規則)、2. Shared Data Structures、3. Algorithm Implementation、4. Correctness Proof、A. Mutual Exclusion、B. Progress (No Deadlock)、C. Bounded Waiting (有限等待 / Fairness)、5. Theoretical Foundation、Safe Registers vs. Atomic Registers、6. Analysis & Limitations、7. Modern Variants。

## 待核對的表述

來源「number 只增不減」與退出臨界區時清零的演算法片段需區分；有限整數的溢位風險不等同每個程序的號碼永遠單調增加。FCFS 的到達界線與現代記憶體模型下的實作條件仍需原始證明或教材核對。[[operating-system-concurrency]]
