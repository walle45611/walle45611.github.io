# 概述

- source: `raw/my-vault/Note/Research/IP (internet protocol) 基本介紹.md`
- source_sha256: `e20357aadf29c85fc1e3f6974044db7cdcfb2fa442efde958f8c860e18ac682b`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 IPv4 header、QoS、ToS、DSCP、MTU、Loopback。

## Source Notes

- 是一種不可靠(unreliable)的傳輸協定，不能保證IP能夠成功的到目的地，IP僅提供最好的傳輸服務，如果生錯誤也部會理會。
- 無連接(connectionless)意思是IP並不維護任何後續封包的狀態訊息，妹個封包的處理是互相獨立的，也就是說不會通過seq number來判斷順序
- Version = 0x0100 代表 ipv4 Version = 0x0110 代表 ipv6

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
