# LeetCode 3106 刷題紀錄

- source: `raw/leetcode-3106.md`
- original title: 刷題紀錄 leetcode-3106
- author: Walle
- published: 2024-08-15
- source_created: 2024-08-15
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇解題紀錄處理 cyclic 字母變換與最小距離，對每個字元比較正向與反向的 26 字母距離，在剩餘 `k` 足夠時更新結果字串。

## Key Points

- 字元變換距離取 `abs(s[i] - new_char)` 與 `26 - abs(s[i] - new_char)` 的較小值。
- 每次接受變換後扣除對應距離，並持續更新結果 `t`。
- 原文保留題目連結與解題程式，摘要只保留演算法思路。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
