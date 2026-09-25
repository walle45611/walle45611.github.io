# 定義

- source: `raw/my-vault/Note/Research/Splay Trees.md`
- source_sha256: `9ea5a760598d6285fd70d135db87948351a5ba5ea530c10b47501f74434dbc60`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Splay Tree Lemma 與 Theorem 證明整理、Lemma：單次 splay 的 amortized cost、Theorem 10.1：n 次操作的總時間、Splay 操作規則、範例。

## Source Notes

- Splay Tree：一種二元搜尋樹 (BST)。
- 每次執行 Search / Insert / Delete 操作後，會對相關節點做 splay (伸展)，也就是透過一連串旋轉，將該節點移到樹根。
- 目標：利用平攤分析 (amortized analysis)，保證平均操作時間為 O(log n)。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
