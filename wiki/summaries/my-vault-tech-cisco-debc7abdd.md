# Cisco 監控模組與路由處理器備援

- source: `raw/my-vault/Note/Tech/Cisco 監控模組與路由處理器備援.md`
- source_sha256: `e75099b9e5c2ce40d0d28b09dd51f85ca9835ec52f7e0f617771278f72ba29c1`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 備援交換監控模組（Supervisor Module Redundancy）、三種備援模式、RPR (Route Processor Redundancy)、RPR+ (Route Processor Redundancy Plus)、SSO (Stateful Switchover)、設定備援模式。

## Source Notes

- HSRP、VRRP 和 GLBP 提供 Gateway 位址的高可用性機制。當主要路由器故障時，備援路由器會自動接替角色。
- 然而，若是直接連接故障的路由器可能會導致封包無法轉送，尤其當交換機或路由器的處理器故障時。部分 Cisco Catalyst 系列（如 4500R、6500、6800）具備備援監控模組，可在發生故障時接手，確保不中斷的運作。
- 支援機型：Catalyst 4500R / 6500 / 6800

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
