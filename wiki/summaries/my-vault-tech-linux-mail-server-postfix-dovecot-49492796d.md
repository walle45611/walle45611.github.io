# Linux Mail Server：Postfix、Dovecot 與 Maildir 設定指南

- source: `raw/my-vault/Note/Tech/Linux Mail Server：Postfix、Dovecot 與 Maildir 設定指南.md`
- source_sha256: `4b86808e5f78c5ab7aebd930341eb6617e9abed4c59b207f34d8e6dad3f7a7f1`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 架構與角色、實驗環境、Postfix 本機投遞、Dovecot 與 IMAP 讀信、兩台主機的 CDB 路由、DB 是什麼？。

## Source Notes

- 目錄：DevOps Technology Overview
- 本實驗先完成單台主機的本機投遞與 IMAP 讀信，再設定兩台 Postfix 之間的 SMTP 路由。以下整理最後採用的 CDB + IP transport map，保留實際操作中遇到的問題。
- 分工的意義是把「郵件如何送達」、「郵件放在哪裡」與「使用者如何讀取」分開。Maildir 是儲存格式，不是網路服務；本實驗由 Postfix 寫入、Dovecot 讀取同一個信箱。Dovecot 也有 LDA／LMTP 投遞能力，但本實驗未使用。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
