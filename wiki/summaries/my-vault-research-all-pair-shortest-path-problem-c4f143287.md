# All-Pair Shortest Path Problem

- source: `raw/my-vault/Note/Research/All-Pair Shortest Path Problem.md`
- source_sha256: `50dc55c167e691b58f0b35ab9568ed6b42c0852b777a730cd1b6d4cede7c1667`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 範例、應用、核心問題、核心思想：「重設權重 (Re-weighting)」、演算法步驟、關鍵推導：為什麼 Re-weighting 有效？。

## Source Notes

- 方法概念：把每個頂點當成來源，重複執行單源最短路演算法，蒐集所有 $s\to v$ 的距離與路徑。
- 鄰接矩陣：每次 $O(V^2)$，共 $n=V$ 次 ⇒ $O(V^3)$。
- 鄰接串列＋最小堆：每次 $O((V+E)\log V)$ ⇒ $O(VE\log V)$（亦可寫 $O(V^2\log V + VE\log V)$）。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
