# EtherChannel 介紹

- source: `raw/my-vault/Note/Research/EtherChannel 介紹.md`
- source_sha256: `8b9a71f14a44c95229c3284936bdd61868a96365a0d6a4465df91dcfcde7431a`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 概觀、EtherChannel限制、EtherChannel流量分配、EtherChannel load-balance設定、Etherchannel protocol、PAgP。

## Source Notes

- 在很多的時候要做冗餘的設計，也就是多拉幾條線，但是這樣STP會把路徑blocking掉造成花費更多的錢買更多高級的線材就只是為了冗餘(Redundancy)的設計，這樣有點浪費了，所以有了ehterchannel這項技術的時候就可以將多條線邏輯成一條線路並將所有線材的STP status從blocking轉到forwarding狀態，這樣不只線材損壞時有多的備用路徑，還可以有更大的BW。
- 一般來說，必須是屬於通一個VLAN或是trunk的交換port必須楚瑜在trunking mode，且必須要有相同的navtive vlan。每個交換port在aggregate的時候speed和duplex需要一樣才能設定成功。
- 在EtherChannel中的資料以特定的方式在鏈路之間的分配，而是以hash演算法的結果，但附載並不需要等量的負載均衡，hash演算法可以採用source ip，或是destination ip，source ip and destination ip，source mac address、destination address組合或是TCP/UDP通訊編號。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
