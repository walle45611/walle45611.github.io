# Windows Server Trust

- source: `raw/my-vault/Note/Tech/Windows Server Trust.md`
- source_sha256: `521cdee2bcdd3695057ca963b07450f82980cf80ef7ecbc2b6d66b50c8267170`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

兩個domain必須要有信任關西trust relationship，才可以存取對發的網域資源，加入到新的ADDS網域會自動信任上一層的父系網域，父系網域也會信任下層的，這些具有two-way transitive，稱為kerberos trust，網域信任也可以是單向信任

## Source Notes

- 其餘細節請直接查原文；此頁只整理已讀到的文字、章節與內容形式。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
