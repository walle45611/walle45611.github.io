# DOMjudge 安裝指南

- source: `raw/domjudge-install-guide.md`
- original title: DOMjudge 安裝指南
- author: Walle
- published: 2024-10-22
- source_created: 2024-10-22
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇文章整理在 Ubuntu 上安裝 DOMjudge 程式設計競賽管理系統的流程，涵蓋 DOMServer、Judgehost、資料庫、Apache/PHP FPM 與自動化 Shell 腳本。

## Key Points

- DOMServer 負責競賽計分、登入與對外操作。
- 安裝流程包含 MariaDB、Apache、PHP、DOMjudge 編譯與資料庫初始化。
- Judgehost 負責執行參賽者提交的程式，需要設定必要套件、chroot、GRUB 與 systemd 服務。
- 原文以自動化腳本收斂重複安裝步驟；完整指令與腳本保留在 Blog source。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
