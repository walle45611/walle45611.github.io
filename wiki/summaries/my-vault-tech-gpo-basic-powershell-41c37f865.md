# GPO Basic + PowerShell 操作整理

- source: `raw/my-vault/Note/Tech/GPO Basic + PowerShell 操作整理.md`
- source_sha256: `52d3a4517e305d0f813e879281b15f1f4390f5ee17d45c42b094a05f1fbb5cee`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 概述（Group Policy Object - GPO）、GPO 套用時機、GPO 設定方式、開啟 GPO 編輯工具、找到需要設定的 GPO、停止 GPO 繼承 / 強制 GPO 套用。

## Source Notes

- 可以使用 GPO 讓在網域內的電腦受到一些限制或自動安裝軟體等功能
- 若本機與網域安全性原則設定衝突，優先使用 網域設定，本機設定僅在網域設定未定義時才會生效
- 非 DC：預設每 90~120 分鐘套用（每 16 小時強制套用一次）

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
