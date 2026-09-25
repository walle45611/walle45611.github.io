# properties

- source: `raw/my-vault/Note/Research/Red-Black tree.md`
- source_sha256: `81a2e56e041b2286cdc1c39e5c09e35becf5aebb2899b20b86af60ec9f23f394`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 定理、Proof、步驟、特性、Rotation、pseudocode。

## Source Notes

- 對於每個節點，從該節點到所有後代葉子的所有簡單路徑上，必須包含相同數量的黑色節點。
- 葉子（leaf）實際上就是 NIL，不會存放真實 key，書裡的做法是用一個哨兵 (sentinel) 物件 T.nil 代表所有的 NIL，它的 color 固定是 BLACK，其他屬性（p, left, right, key）值隨意，因為不會被用到。
- 如果每個 NIL 都是獨立物件，就會有很多額外的節點。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
