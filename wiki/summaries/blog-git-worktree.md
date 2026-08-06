# Git Worktree 與線性 Commit Log

- source: `raw/git-worktree.md`
- original title: 使用 Git Worktree 提高開發效率與維持線性 Commit Log
- author: Walle
- published: 2025-02-20
- source_created: 2025-02-20
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇文章以 Hotfix 與新功能開發為例，說明如何利用 Git Worktree 同時檢出多個分支，減少 stash、環境切換與混亂 merge commit。

## Key Points

- Hotfix 可以從 `develop` 建立獨立 worktree，在不切換主要工作目錄的情況下修正問題。
- 新功能也可以用另一個 worktree 並行開發，保留各自的依賴與執行環境。
- 原文示範 bare repository、worktree 建立、分支操作與提交 PR 的流程。
- Worktree 的價值在於工作目錄分離與歷史可讀性，不是取代分支或版本控制本身。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
