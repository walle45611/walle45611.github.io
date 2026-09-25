# Windows Server DFS namespace and DFS Replication 設定指南

- source: `raw/my-vault/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南.md`
- source_sha256: `6308299ee0e10b44707c6f79db8fcee37f70bdc9cd91d4af79a4d07e14e7572f`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 DFS 架構、DFS replication、replication topology、case、資料。

## Source Notes

- 透過DFS將相同檔案同時儲存到網路上多台server
- 可以提高檔案存取率 : DFS來存取資料時，DFS會引黨用戶端從，最接近用戶端的server來存取檔案，讓用戶端快速存取到檔案
- DFS會提供用戶端一份server list，這些server內都有用戶端所需的資料，但是DFS會將最接近用戶端的server給client使用，例如跟用戶端同一個ADDS站台的server，放在清單的最前面。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
