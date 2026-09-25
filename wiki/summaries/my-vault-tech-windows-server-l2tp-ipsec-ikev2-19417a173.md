# Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南

- source: `raw/my-vault/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南.md`
- source_sha256: `0a5784ebda421d03f5f6810184f5c569acb8a63c27510ebfd8fdcdd67fe67f73`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 L2TP/IPSec Pre-Shared Key Tunnel、網路拓撲、設定步驟、先決條件、VPNS2 設定、L2TP/Certificate IPSec Tunnel。

## Source Notes

- 須先建立 PPTP Site-to-Site VPN，Pre-shared Key 需設定一樣。
- 在 DC 上安裝服務：安裝 Domain Controller、ADCS 和 DHCP（供內部使用）
- VPNS1 加入網域：將 VPNS1 加入 DC 的網域

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
