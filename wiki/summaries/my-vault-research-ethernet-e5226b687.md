# Ethernet 乙太網路協定指南

- source: `raw/my-vault/Note/Research/Ethernet 乙太網路協定指南.md`
- source_sha256: `d727977276ca9f759ab08b17407b75fbf1d28092e29f9930d29eade64af99e63`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 Ethernet II 格式、IEEE 802.3 Ethernet、整體流程、封裝與解封裝過程、Encapsulation（封裝）、重要特性。

## Source Notes

- 在眾多 Data Link Layer 協定中，使用最廣泛的莫過於 Ethernet。它簡單、易於 NIC 及 Driver 實現，初期 Ethernet NIC 相對於其他網卡，最初的速度從 10Mbps、1Gbps 到最後都可以達到更高速的網路。現在 Ethernet 已經成為最具有相容性與未來發展的資料鏈路層協定。
- 早期是由兩家美國公司設計的通訊方式，後來由 IEEE 802.3 規範化，但是兩個的 frame 內容又有點不一樣，因此 IEEE 802.3 所規範的 Ethernet 又被稱為 802.3 Ethernet。
- 有一些 Layer 2 的協定會使用這個協定，如 CDP、VTP，但通常還是使用 Ethernet II。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
