# Cisco ACL 存取控制清單指南

- source: `raw/my-vault/Note/Tech/Cisco ACL 存取控制清單指南.md`
- source_sha256: `9a27616b4da0fb45013485fea20b861ff78dbdb3c9e8a7b41bcf1289e02c5ab6`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 ACL 概念、ACL 編號分類、改良的編輯方式、WC (Wildcard Mask)、計算範例、標準型 ACL。

## Source Notes

- 假設有兩條路由：192.168.1.0/24 和 192.168.1.0/30，只要抓取 192.168.1.0/24 使用 ACL 無法實現，只能兩條都抓，或都不抓
- 使用序號（sequence number）進行精確的 ACL 條目管理。
- Wildcard mask 是一個用於決定哪些 IP 位址該精確匹配的 32 位元數值

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
