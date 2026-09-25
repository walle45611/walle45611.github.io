# LCS vs. Minimum Edit Distance

- source: `raw/my-vault/Note/Research/LCS vs. Minimum Edit Distance.md`
- source_sha256: `0e932b82c74233b8816fb646835aa23e07d9d9edcc23798eb70ccbbc98b0a23c`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 核心差異總覽 (Core Differences)、最長公共子序列 (LCS)、A. 問題定義、B. 結構、狀態與轉移、最優解的結構 (Optimal Substructure)、狀態與轉移方程。

## Source Notes

- 給定兩序列 $X=\langle x_1,\dots,x_m\rangle$ 與 $Y=\langle y_1,\dots,y_n\rangle$，找 $X$ 與 $Y$ 的最長共同子序列。
- 子序列：刪除若干元素且不改變相對順序所得，如 "ace" 為 "abcde" 之子序列，"aec" 不是。
- 這是推導 DP 公式最關鍵的一步。我們想找出 $X_m$ ( $X$ 的所有字元) 和 $Y_n$ ( $Y$ 的所有字元) 之間的 LCS，我們只需要比較最後一個字元：$x_m$ 和 $y_n$。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
