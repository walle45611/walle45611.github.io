# K8s CNI 比較：Cilium、Calico 與 Flannel

## Bridge 與 CNI：考前先看

> [!tip] 一句話
> Bridge 是主機內的虛擬 L2 switch；CNI 是容器網路設定介面；Calico／Cilium／Flannel 是網路方案。三者不在同一個層次。

同 Node 的入門模型：`Pod netns → veth → Linux bridge → 另一個 Pod`。Bridge 依 MAC 轉送 frame，不能單靠一座 bridge 解決跨 Node 路由與 NetworkPolicy；實際 CNI 也不一定使用 bridge。

| 類別 | 名稱 | 記法 |
|---|---|---|
| 網路連接 plugin／技術 | bridge | 把容器介面接上 Linux bridge |
| 網路連接 plugin／技術 | vlan | 使用 802.1Q VLAN 區隔 L2 網路 |
| 網路連接 plugin／技術 | macvlan | 在父介面建立具有各自 MAC 的虛擬介面 |
| 網路連接 plugin／技術 | ipvlan | 虛擬介面共用父介面的 MAC，依模式處理 L2／L3 |
| 平台 | Windows | 有對應 Windows 網路 plugins；不是單一 Linux plugin 的替代名稱 |
| IPAM | host-local | 在本機管理 IP 配置；不是 DNS，也不自動協調全叢集位址池 |
| IPAM | dhcp | 向 DHCP 服務取得租約；需相關服務配合 |
| 整體方案 | Flannel／Calico／Cilium／VMware NSX | 能力與支援介面依產品、版本及整合方式而異 |

**CNI vs CNM**：CNI 是容器網路設定介面，常見於 Kubernetes；CNM（Container Network Model）是 Docker libnetwork 的網路模型，以 sandbox、endpoint、network 描述連接。不是同一套規格。K8s 的 CRI runtime 通常負責呼叫 CNI 完成 Pod sandbox 網路設定。

Network addon 安裝題，就是部署讓 Pod 網路能運作的元件；若要求 NetworkPolicy，還要確認方案有政策執行能力。依題目指定的版本、CIDR、資料平面與安裝來源操作，不能只看到 Pod Running 就認定政策有效。

底層實驗見 [[Linux Network Namespace：netns、veth 與 Bridge]]；安裝與介面判讀見 [[K8s Calico 安裝與網路判讀小抄]]。

參考：[CNI plugins 分類](https://www.cni.dev/plugins/current/)、[Docker 容器網路](https://docs.docker.com/engine/network/)、[Kubernetes 網路模型](https://kubernetes.io/docs/concepts/cluster-administration/networking/)。

## 先區分實作與底層技術

Cilium、Calico、Flannel 是 Kubernetes 網路方案；eBPF、VXLAN、BGP 是不同層次的技術，不能當成三個互斥選項。

| 名稱 | 解決的問題 |
|---|---|
| CNI | 容器網路設定介面，讓 plugin 建立／移除網路連接 |
| eBPF | 在 Linux kernel 的 hook 執行程式，可用於 policy、NAT、負載均衡與轉送 |
| VXLAN | 把內層 Ethernet frame 封裝在 UDP/IP，跨底層網路傳遞 |
| BGP | 交換可達前綴與路徑資訊，讓路由器知道網段位置 |
| Native routing | 不用 overlay 封裝，底層網路需要能轉送 Pod IP |

政策 YAML、Ingress／Egress 與 selector 範例見 [[K8s NetworkPolicy 網路存取控制]]。

## 三種方案比較

| 方案 | 主要特性 | 跨 Node 網路 | 注意事項 |
|---|---|---|---|
| Cilium | eBPF 資料平面、policy、可觀測性 | VXLAN／Geneve 或 native routing | 可搭配 BGP 宣告 Pod／Service 路由；替代 kube-proxy 需對應設定 |
| Calico | L3 networking、NetworkPolicy | 無封裝路由、VXLAN、IP-in-IP，依模式配置 BGP | 不能把 Calico 等同於 BGP；也有 eBPF 資料平面選項 |
| Flannel | 提供 Pod 網路連通 | VXLAN、host-gw 等 backend | 單獨 flanneld 不執行 NetworkPolicy；需搭配政策控制器，官方 Helm chart 可用 netpol.enabled 整合 |

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
