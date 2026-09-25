# Hardware Synchronization Implementation & Proofs

- source: `raw/my-vault/Note/Research/Hardware Synchronization Implementation & Proofs.md`
- source_sha256: `5051e3e63d4e75c5bed55f4c1049ca5e5a4a81a25dfc9d65fdb46e8a05e3cb5f`
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
