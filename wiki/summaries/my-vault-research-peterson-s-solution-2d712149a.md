# Peterson's Solution

- source: `raw/my-vault/Note/Research/Peterson's Solution.md`
- source_sha256: `d178aa273cf1627b2a830e87ee03d78f8e19d626db95604a4a7d96d1be036031`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 演算法 1：使用 turn 變數 (Strict Alternation)、程式碼邏輯 (Process $P_i$)、正確性分析、演算法 2：使用 flag 旗標、演算法 3：Peterson's Solution (彼得森解法)、A. Mutual Exclusion: ✅ 滿足。

## Source Notes

- 共享變數：int turn; (初始值為 $i$ 或 $j$)
- 此方法嘗試讓處理程序「宣布」它們進入臨界區域的意願。
- 共享變數：boolean flag[2]; (初始值皆為 false)

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
