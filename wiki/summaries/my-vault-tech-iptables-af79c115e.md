# iptables 防火牆完整指南

- source: `raw/my-vault/Note/Tech/iptables 防火牆完整指南.md`
- source_sha256: `cb442ad002e7e88814b8777debec88b2edb0117f5b3a10e23bf8c1e7008cc666`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 iptables 檢查流程和 table 優先順序、iptables Tables 和 Chain、iptables 安裝、iptables 基本語法、自定義 Chain、匹配條件。

## Source Notes

- ip_conntrack 模組追蹤封包，可以利用 cat /proc/net/ip_conntrack 來看有哪些封包經過 iptables，最大只能儲存 32768 個。
- 紅色代表不允許、黃色代表可以但是需要轉送、綠色代表可以
- 如果需要將 inClt 加入到網域，需要開啟這些 port，從 internal 到 dmz

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
