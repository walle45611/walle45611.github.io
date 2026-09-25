# Microsoft EntraID

- source: `raw/my-vault/Note/Tech/Microsoft EntraID.md`
- source_sha256: `bc498feb86ddb10663e4d986bca3c45b9b889381cff0d5f39ae629527b2b23c6`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 1. Revoke session (撤銷工作階段)、2. Delete (刪除帳戶)、重點對比總結。

## Source Notes

- 主要用途：當帳戶疑似遭入侵 (Compromised) 時使用的安全性操作。
- 執行效果：此操作會立即中斷所有當前已通過驗證的活動連線。
- 帳戶狀態：帳戶本身依然存在。通常在執行撤銷後，管理員會接著停用 (Disable/Block) 該帳戶，以防止攻擊者在連線被切斷後重新驗證登入。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
