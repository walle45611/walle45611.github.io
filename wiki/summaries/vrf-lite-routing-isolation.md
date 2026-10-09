# VRF-Lite：路由隔離設定案例

- source: `raw/my-vault/Note/Research/VRF-Lite (Virtual Routing and Forwarding).md`
- source_sha256: `2f53f18f10bcff7041cf0b41199165f377ad9141a106ccf81ebfd70f4a264cbe`
- source_created: not specified
- ingested_at: 2026-10-09
- type: source summary

## 摘要與關鍵資訊

筆記把一台 router 視為多個 virtual router，以 VLAN 子介面配合 VRF 與不同 OSPF process 區隔流量。

- R1 的 .2／.3／.4 子介面分別接 VOICE、DATA、VIDEO；設定 dot1Q、ip vrf forwarding、IP 與 OSPF。
- SW1 以 trunk 接 R1，access VLAN 2／3／4 連接 R2／R3／R4；各遠端 router 提供自己的 LAN 與 OSPF 設定。
- 來源保留兩張拓樸圖與 Cisco 設定，並連回 MLS；VRF-Lite 已從 MLS 抽成獨立筆記。

## 設定一致性與來源邊界

R1 建立 VRF 的片段寫有 VOICE、VOIDE、DATA，後續卻引用 VIDEO；這是來源內待核對的名稱不一致，不能當成可直接執行的完整設定。未提供實測路由表、跨 VRF 通訊結果或 route leaking 設計；不推論已驗證隔離效果。

- source link: https://www.infraexpert.com/study/mpls12.html
- 相關來源摘要：[[my-vault-research-mls-mls-multilayer-switching-61f7c8912]]。

## 相關概念

- [[network-protocol-layers]]
