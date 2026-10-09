# TCP/OSI 網路模型完整指南

- source: `raw/my-vault/00_Dashboard/Networking Overview.md`
- source_sha256: `ff17805d8314fb32064164a432def66e1a43938c873c1c999640856ec9db47bf`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Dashboard

## Summary

原筆記涵蓋 為什麼需要分層、分層的必要性、分層的優勢、WAN (Wide Area Network) 廣域網路、為什麼需要 WAN、WAN 在 OSI 模型中的位置。

## Source Notes

- 標準統一：沒有分層會導致每家廠商都需從 Physical Layer 開始開發，無法統一標準，不利於專業分工
- 模組化設計：層與層之間有清晰邊界，有助於理解與模組化
- 服務層次：每層是服務者也是被服務者，對上層提供服務，對下層提出需求

## Navigation

- [網路協定分層](../concepts/network-protocol-layers.md)
- [回到 Dashboard 歸檔](<../archives/my-vault-dashboard.md>)

## 2026-10-09 來源複核

Network Layer 新增 VRF-Lite 路由隔離入口；網路模擬器列 Containerlab。相關摘要為 [[vrf-lite-routing-isolation]]、[[containerlab-network-labs-as-code]]。

目前來源章節：TCP/OSI 網路模型完整指南、為什麼需要分層、分層的必要性、分層的優勢、WAN (Wide Area Network) 廣域網路、為什麼需要 WAN、WAN 在 OSI 模型中的位置、WAN 接入方式、1. 專線（Point-to-Point）、2. 電路交換、3. 分組交換、WAN 物理層、WAN 常見封裝協定、專線協定、電路交換協定、Frame Relay、VPN 技術、基本的網路連線、靜態 IP 位址設定、動態 IP 指派、OSI 七層詳解、Physical Layer (實體層)、實體傳輸與網路線、RJ45 T568B T568A、跳線與平行線使用規則、full-duplex, half-duplex、auto-negotiation、**Auto MDI/MDIX**、Data Link Layer (資料鏈路層)、Switching 相關協定、通訊協定、Network Layer (網路層)、路由隔離、通訊協定、Routing 協定、Traceroute 工具、Traceroute 運作原理、使用範例、Transport Layer (傳輸層)、常見協定、Application Layer (應用層)、Cisco 裝置管理相關、常見協定、網路模擬器、網路認證與 VPN 排錯。
