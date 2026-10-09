# 同步原語互轉實作 (Semaphore $\Leftrightarrow$ Monitor)

- source: `raw/my-vault/Note/Research/monitor.md`
- source_sha256: `16279f2282a841251746f6ee8ad601802574b66318033d32a2f5a1bfa950a4b3`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Part 1. 低階轉高階：用 Semaphore 實作 Monitor、變數宣告 (The Monitor Structure)、外部程序控制 (Entry & Exit Protocol)、條件變數操作 (Condition Variables)、A. x.wait() 的實作、B. x.signal() 的實作。

## Source Notes

- 我們需要一組全域 Semaphore 來模擬 Monitor 的「大門」與「緊急暫停區」。
- 為了確保互斥，任何外部函式 $F$ 的前後都必須加上控制碼。
- 邏輯：釋放目前的鎖 $\to$ 進入睡眠 $\to$ 醒來後恢復執行。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：同步原語互轉實作 (Semaphore $\Leftrightarrow$ Monitor)、Part 1. 低階轉高階：用 Semaphore 實作 Monitor、1. 變數宣告 (The Monitor Structure)、2. 外部程序控制 (Entry & Exit Protocol)、3. 條件變數操作 (Condition Variables)、A. `x.wait()` 的實作、B. `x.signal()` 的實作、4. 邏輯流程圖 (Mermaid)、Part 2. 高階轉低階：用 Monitor 實作 Semaphore、C++ 實作碼、Part 3. 綜合比較 (Exam Focus)。

## 待核對的表述

來源比較表將 semaphore 的有記憶性與 signal 特性並列；應區分 condition-variable notification 與 semaphore 的資源計數，不能只從 notify 推論訊號被保存。完整 Hoare／Mesa 行為與範例保證待依指定教材進一步核對。[[operating-system-concurrency]]
