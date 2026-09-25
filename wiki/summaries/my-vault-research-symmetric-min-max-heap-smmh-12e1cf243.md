# 對稱最小最大堆積 (Symmetric Min-Max Heap，SMMH)

- source: `raw/my-vault/Note/Research/對稱最小最大堆積 (Symmetric Min-Max Heap，SMMH).md`
- source_sha256: `09b55dc413588a32566df142d76d881a7299154592306112d759e65791332f80`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Def、對等於：假設 elements(N) ≠ 0，則 N 滿足以下性質、INSERT X IN SMMH、範例：插入值 X = 2、DELETE-MIN in SMMH、DELETE-MIN in SMMH 範例流程。

## Source Notes

- 為 Complete Binary Tree，支援雙端優先佇列（double-ended priority queue）的實作。
- Insert、delete-min、delete-max 操作皆為 $O(\log n)$ 時間複雜度。需滿足以下性質
- 左兄弟 ≤ 右兄弟資料（left sibling <= right sibling data）【性質 P₁】

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
