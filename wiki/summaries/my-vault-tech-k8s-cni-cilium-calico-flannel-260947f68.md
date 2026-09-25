# K8s CNI 比較：Cilium、Calico 與 Flannel

- source: `raw/my-vault/Note/Tech/K8s CNI 比較：Cilium、Calico 與 Flannel.md`
- source_sha256: `b909942891b34e3c9b2179ef06990ec1d4fd224f2d28ad41664140acbefd2457`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 先區分實作與底層技術、三種方案比較、eBPF、VXLAN 與 BGP 可以共存。

## Source Notes

- Cilium、Calico、Flannel 是 Kubernetes 網路方案；eBPF、VXLAN、BGP 是不同層次的技術，不能當成三個互斥選項。
- 參考：Cilium Routing、Calico networking、Flannel。
- eBPF 仍然會處理封包。它可以減少某些傳統網路堆疊與規則查找的成本，但效能取決於核心、模式、policy、流量及硬體，不能直接保證所有情境都比較快。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
