# Famous Synchronization Problems

- source: `raw/my-vault/Note/Research/Famous Synchronization Problems.md`
- source_sha256: `2d7b91f1590cf7cdc53b1683eae2f39f8d43f0fe3883acb2b8f8ec76324e23c3`
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
