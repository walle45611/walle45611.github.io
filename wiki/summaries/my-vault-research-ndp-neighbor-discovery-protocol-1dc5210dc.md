# NDP (Neighbor Discovery Protocol) 基本介紹

- source: `raw/my-vault/Note/Research/NDP (Neighbor Discovery Protocol) 基本介紹.md`
- source_sha256: `8e4d5f56d4f6fbf9855954b3640d6b762a2a892ba62523de51b1d8729c77181f`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 用處、RS/RA、使用NDP RS及RA探索路由器、使用NDP RS及RA探索SLAAC定址資訊、NS/NA、使用NDP NS及NA探索鄰居的鏈路地址。

## Source Notes

- SLAAC : 使用SLAAC時，主機使用NDP訊息來獲得其位址的第一部分與首碼長度
- 路由器探索 : 主機使用NDP訊息像相同子網中可用的路由器獲取IPv6位址
- 重複位置偵測 : 無論主機的IPv6位址是經由人工設定或是自動獲取，該主機沒辦法知道位址是否重複，所以要透過DAD(Duplicate address Detectin)的程序使用NDP訊息

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
