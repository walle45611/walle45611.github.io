# Virtual Judge：HDU-1232

- source: `raw/virtual-judge-hdu-1232.md`
- original title: virtual-judge 刷題紀錄 08/15
- author: Walle
- published: 2024-08-15
- source_created: 2024-08-15
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇 Virtual Judge 刷題紀錄以 HDU-1232 為例，使用 DFS 計算圖的連通分量，並由分量數推導使圖連通所需新增的邊數。

## Key Points

- DFS 走訪所有尚未訪問的節點，每啟動一次就代表找到一個 connected component。
- 若連通分量數為 `connect`，要把它們連成一個連通圖需要 `connect - 1` 條邊。
- 原文的解題程式保留在 Blog source；此頁只保存問題抽象與演算法思路。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
