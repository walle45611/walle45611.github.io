# Linux Mail Server：Postfix、Dovecot 與 Maildir 設定指南

- source: `raw/my-vault/Note/Tech/Linux Mail Server：Postfix、Dovecot 與 Maildir 設定指南.md`
- source_sha256: `f4de908246448d0afbf0d90c8bff92926dde7dadd9b7a3f2a1c5e449b48ed01d`
- source_reviewed: 2026-09-30
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 架構與角色、實驗環境、Postfix 本機投遞、Dovecot 與 IMAP 讀信、兩台主機的 CDB 路由、DB 是什麼？。

## Source Notes

- 本實驗先完成單台主機的本機投遞與 IMAP 讀信，再設定兩台 Postfix 之間的 SMTP 路由。以下整理最後採用的 CDB + IP transport map，保留實際操作中遇到的問題。
- 分工的意義是把「郵件如何送達」、「郵件放在哪裡」與「使用者如何讀取」分開。Maildir 是儲存格式，不是網路服務；本實驗由 Postfix 寫入、Dovecot 讀取同一個信箱。Dovecot 也有 LDA／LMTP 投遞能力，但本實驗未使用。

## 實驗與驗證邊界

- 兩台主機以 `transport_maps = cdb:/etc/postfix/transport` 指定路由；修改文字 map 後需重新執行 `postmap`，reload 不會重建索引。
- `smtp:[IP]` 避免這個 next hop 的名稱解析；方括號抑制 MX 查詢，不代表所有 SMTP 場景都不需要 DNS。
- 手動連到 mail2 的 SMTP 測試會繞過 mail1 的佇列與 transport map，不能用它單獨證明 mail1 的路由成功。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)

## 相關概念

- [[network-protocol-layers]]：將二層拓樸、SMTP／IMAP 服務與主機防火牆放回各自的排錯層次。
