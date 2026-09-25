# K8s Cilium 與 Containerlab 網路實驗

- source: `raw/my-vault/Note/Tech/K8s Cilium 與 Containerlab 網路實驗.md`
- source_sha256: `e7b4d57527455847ca68e52a8583e5615aa84f97ae0866d3988962ca076109c5`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 實驗目標與拓撲、分階段驗證、Containerlab 與 Cisco 的連接、延伸環境討論。

## Source Notes

- 目標是觀察跨 Node Pod 流量、eBPF、VXLAN／native routing，再加入 Cisco BGP。以下是對話整理的實驗設計，IP 與 ASN 均為範例，並非已部署或已驗證的環境。
- 一個 control plane 加一個 worker 可以觀察跨 Node 網路，但不是 HA。使用者在原對話選擇先練網路、希望搭配 Cisco；HA 原理另見 K8s HA、Quorum 與 Split-brain。
- 建立兩個 Node，確認 CNI 在每個 Node 都正常。MicroK8s 的 CNI 替換必須按實際版本與 addon 程序處理，不能把 community addon 的歷史 workaround 當作通用安裝順序。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
