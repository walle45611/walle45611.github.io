# FortiGate 與 Cisco IOL：UTM OSPF Lab

- source: `raw/my-vault/Note/Tech/FortiGate 與 Cisco IOL：UTM OSPF Lab.md`
- source_sha256: `6fd6655a6214cb9e72955414d978133d073b617eac237b57bb93129f536c7304`
- source_created: not specified
- ingested_at: 2026-10-09
- type: source summary

## 摘要與關鍵資訊

實驗把 FortiGate ARM64 VM 經 Debian enp0s2／br-fgt 接到 Containerlab 的 Cisco IOL R1、R2，再連接 PC1 與 Server；三台 Router 使用 OSPF Area 0。

- 記錄環境為 Apple Silicon Mac／UTM、Debian ARM64、Cisco IOL 17.15.1 與 Alpine container；FortiGate 截圖顯示 FortiOS 8.0.1。
- FortiGate port2 為 10.0.12.1/30，R1 為 10.0.12.2/30；R1–R2 使用 10.0.23.0/30，R2 後方 LAN 為 10.10.10.0/24 與 10.20.20.0/24。
- FortiGate Router ID 3.3.3.3 是識別碼，不能推論存在同位址 loopback。GUI 顯示鄰居 10.0.12.2，但 FULL 狀態仍須 CLI 確認。
- 路由截圖共有 8 筆：5 OSPF、2 Connected、1 Static；兩個 LAN 路由已傳播到 FortiGate。
- Debian bridge 必須先建；Containerlab 引用既有 bridge。R1 範本使用 point-to-point，FortiGate 對應介面亦需配合；截圖沒有顯示當時 network type。
- IOL .partial 初始設定保留預設管理內容；NVRAM、重新部署與清除狀態的影響須分開理解。Rosetta／Alpine 安裝排錯收在 [[my-vault-tech-containerlab-4e538a1cc]]。

## 限制與待補證據

VM CPU、RAM、磁碟與映像取得細節尚未補齊；Fortinet 引用文件為 7.4.9，8.0.1 語法以實機為準。本次完成 routing，未實作 IaC；NAT／防火牆政策、外部 IP 與 DNS 測試屬延伸驗證，不列為已完成。

## 相關概念

- [[container-networking]]
- [[network-protocol-layers]]
