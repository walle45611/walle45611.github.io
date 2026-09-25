# STP (Spanning tree) 完整指南

- source: `raw/my-vault/Note/Research/STP (Spanning tree) 完整指南.md`
- source_sha256: `57d1ef8cc97546c4db441c1524885fa70a8fc247bab2c40b05f8135413ef7047`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 為什麼需要、Spanning-tree standards、BPDU (bridge protocol data unit)、Bridge ID、STP Cost、port id。

## Source Notes

- 因為switch備援路線會導致網路有回環產生進而產生出廣播風暴和Mac address泛洪的問題，所以要解決這個問題就有人提出使用Spanning tree這項演算法來解決此問題。
- PVST+ Cisco私有協定，沒個VLAN有自己的Spanning tree
- 利用BPDU傳送STP的各種訊息在switch之間來進行選舉。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
