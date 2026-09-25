# Mutex Locks & Semaphores

- source: `raw/my-vault/Note/Research/Mutex Locks & Semaphores.md`
- source_sha256: `6ff820c2e2d9df71351311a8129987be9020891f177da56ad3205e19f4da2c7c`
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
