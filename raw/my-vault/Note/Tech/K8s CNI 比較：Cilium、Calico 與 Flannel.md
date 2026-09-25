# K8s CNI 比較：Cilium、Calico 與 Flannel

## 先區分實作與底層技術

Cilium、Calico、Flannel 是 Kubernetes 網路方案；eBPF、VXLAN、BGP 是不同層次的技術，不能當成三個互斥選項。

| 名稱 | 解決的問題 |
|---|---|
| CNI | 容器網路設定介面，讓 plugin 建立／移除網路連接 |
| eBPF | 在 Linux kernel 的 hook 執行程式，可用於 policy、NAT、負載均衡與轉送 |
| VXLAN | 把內層 Ethernet frame 封裝在 UDP/IP，跨底層網路傳遞 |
| BGP | 交換可達前綴與路徑資訊，讓路由器知道網段位置 |
| Native routing | 不用 overlay 封裝，底層網路需要能轉送 Pod IP |

## 三種方案比較

| 方案 | 主要特性 | 跨 Node 網路 | 注意事項 |
|---|---|---|---|
| Cilium | eBPF 資料平面、policy、可觀測性 | VXLAN／Geneve 或 native routing | 可搭配 BGP 宣告 Pod／Service 路由；替代 kube-proxy 需對應設定 |
| Calico | L3 networking、NetworkPolicy | 無封裝路由、VXLAN、IP-in-IP，依模式配置 BGP | 不能把 Calico 等同於 BGP；也有 eBPF 資料平面選項 |
| Flannel | 提供 Pod 網路連通 | VXLAN、host-gw 等 backend | policy 需搭配控制器；官方 Helm chart 有整合選項，不能一概說完全不支援 |

參考：[Cilium Routing](https://docs.cilium.io/en/stable/network/concepts/routing/)、[Calico networking](https://docs.tigera.io/calico/latest/networking/determine-best-networking)、[Flannel](https://github.com/flannel-io/flannel)。

## eBPF、VXLAN 與 BGP 可以共存

```text
Cilium + VXLAN：
Pod A → eBPF 處理 → VXLAN 封裝 → Node 網路 → 解封裝 → eBPF 處理 → Pod B

Native routing：
Pod A → Node 轉送 → 可達 Pod 網段的底層網路 → 目的 Node → Pod B
```

eBPF 仍然會處理封包。它可以減少某些傳統網路堆疊與規則查找的成本，但效能取決於核心、模式、policy、流量及硬體，不能直接保證所有情境都比較快。

BGP 是路由資訊交換，不是資料封包的封裝。Native routing 可搭配靜態路由或其他路由安排，並非一定要使用 BGP。

Cilium BGP Control Plane 對外宣告路由，**不會替你設定整個叢集的資料平面可達性**。BGP session Established 只代表鄰居建立，還要確認 advertisement、路由安裝、下一跳、回程路由與 NetworkPolicy。依據：[Cilium BGP Control Plane](https://docs.cilium.io/en/stable/network/bgp-control-plane/bgp-control-plane/)。
