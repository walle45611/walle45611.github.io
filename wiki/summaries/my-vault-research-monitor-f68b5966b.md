# 同步原語互轉實作 (Semaphore $\Leftrightarrow$ Monitor)

- source: `raw/my-vault/Note/Research/monitor.md`
- source_sha256: `9ce9d41a1d75ec6fb82a11843ff70c664b225bd0a67594bf8c9240a0342decd2`
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
