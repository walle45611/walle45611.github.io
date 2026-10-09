# OSPFv2

- source: `raw/my-vault/Note/Research/OSPFv2 與 OSPFv3 全面整理：原理、流程、LSA 類型與實作案例.md`
- source_sha256: `66a84df6ed1446c8222cff44911472e6e02d5c9ff67584c84ff78268955a33f8`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 OSPF概念、OSPF流程、OSPF table、OSPF packet type、OSPF area、區域的用意。

## Source Notes

- (Open Shortest Path First,開放最點路徑優先)是一種鏈路狀態路由協定，無路由循環(全局topology)，屬於IGP。RFC2328，”開放”意味著是公開的協定。
- 所有OSPF路由器 224.0.0.5; DR BDR 224.0.0.6 。
- OSPF支持IP、VLSM、CIDR、只能手動用路由彙總功能

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：OSPFv2、OSPF概念、OSPF流程、OSPF table、OSPF packet type、OSPF area、區域的用意、區域的專業用語和路由類型、OSPF的區域設計的注意事項、OSPF network type、Lookback、P2P (CISCO)、Broadcast MA (CISCO)、NBMA (RFC)、P2mP (RFC)、p2mp nbma (CISCO)、OSPF interface type、Command、OSPF網路的接口類型的總結、LSA、什麼是LSA、LSA類型、OSPF基本配置、Basic、show、OSPF同步過程、1. 建立鄰居關係、OSPF建立鄰居關係 Hello封包、2. 建立鄰居流程、DR BDR選舉、出現原因、角色特性、Router ID、OSPF Cost、選舉規則、LSDB sync、3. LSDB確認同步流程、建立完全鄰接關係、4. 建立鄰接關係開始同步流程、LSA的泛洪、各種狀態、鄰居無法建立常見的問題、鄰接關係鄰居關係差別、OSPF認證、明文、MD5、路徑篩選、Type 3 LSA filter、command、case、filter OSPF path 防止新增到 routing table、command、case、路徑彙整、ABR手動**summarization**、command、case、ASBR手動**summarization**、command、預設路徑、OSPF末梢區域、Topology、Stub (末梢區域)、Totally Stub、==Tips :== 使用 Totally Stub 或是 Stub是不能再使用外部路由的，因為沒有4、5類的LSA、NSSA (not-so-stubby area)、Totally NSSA、其他、Virtual link、Max-metric、passive-interface 被動介面、OSPF load balance、OSPF優化、Pacing Timer、SPF Throttle Timer、LSA Throttle Timer、OSPFv3、使用OSPF實作dual stack的兩種選擇、OSPFv2和OSPFv3的本質、OSPFv2和OSPFv3的差別、OSPFv3 LSA、改名的LSA、新的LSA、OSPFv3的設定、傳統的OSPFv3設定、傳統的OSPF設定、show、case、OSPFv3 address family、簡介、case。
