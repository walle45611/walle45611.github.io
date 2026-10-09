# Peterson's Solution

- source: `raw/my-vault/Note/Research/Peterson's Solution.md`
- source_sha256: `5574c9b8fdf15cc90ac382f2c4cdeba4c0130269535c5385922518dd27f73c09`
- source_reviewed: 2026-10-09
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

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：演算法 1：使用 turn 變數 (Strict Alternation)、1. 程式碼邏輯 (Process $P_i$)、2. 正確性分析、演算法 2：使用 flag 旗標、1. 程式碼邏輯 (Process $P_i$)、2. 正確性分析、演算法 3：Peterson's Solution (彼得森解法)、1. 程式碼邏輯 (Process $P_i$)、2. 正確性分析、A. Mutual Exclusion: ✅ 滿足、B. Progress: ✅ 滿足、C. Bounded Waiting: ✅ 滿足、⚠️ 現代架構下的限制 (Modern Architecture Limitations)、原因：重新排序 (Reordering)、解決方案。
