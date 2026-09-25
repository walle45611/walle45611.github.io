# K8s Cilium 與 Containerlab 網路實驗

## 實驗目標與拓撲

目標是觀察跨 Node Pod 流量、eBPF、VXLAN／native routing，再加入 Cisco BGP。以下是對話整理的實驗設計，IP 與 ASN 均為範例，並非已部署或已驗證的環境。

```text
OrbStack Linux machines
├─ Node 1：MicroK8s control plane + Cilium   192.168.139.10
├─ Node 2：MicroK8s worker + Cilium          192.168.139.20
└─ Containerlab host                        192.168.139.30
   └─ host interface 10.255.0.1/30
        ↕ veth
      Cisco IOL 10.255.0.2/30，AS 65000

Node 1／2 的 Cilium BGP：AS 65001
```

一個 control plane 加一個 worker 可以觀察跨 Node 網路，但不是 HA。使用者在原對話選擇先練網路、希望搭配 Cisco；HA 原理另見 [[K8s HA、Quorum 與 Split-brain]]。

## 分階段驗證

1. 建立兩個 Node，確認 CNI 在每個 Node 都正常。MicroK8s 的 CNI 替換必須按實際版本與 addon 程序處理，不能把 community addon 的歷史 workaround 當作通用安裝順序。
2. 用 nodeSelector／affinity 將測試 Pod 放到不同 Node；先確認 control-plane 節點允許排程。
3. 測 Pod IP 的跨節點連通與 DNS、Service，再測 NetworkPolicy。
4. 記錄 tunnel 模式的封包與 MTU；切換 native routing 前先準備 Pod CIDR 的正反向路由。
5. 接上 Containerlab router，先完成 Node IP 與 router IP 的雙向連通，再設定 BGP。
6. 分別驗證 BGP session、宣告前綴、路由安裝及實際資料流。

```bash
kubectl get nodes -o wide
kubectl get pods -A -o wide
cilium status
cilium connectivity test
# 各 Node：
ip addr
ip link
ip route
ip neigh
sudo tcpdump -ni any
```

`cilium` CLI 與 agent 內的 `cilium-dbg` 是不同工具；BPF map 排錯先看所裝版本的 help，不直接沿用舊對話中的 `cilium bpf ...`。封包擷取埠以實際 tunnel 設定為準。

## Containerlab 與 Cisco 的連接

Containerlab 的 host link 把 router 介面接到該 Linux host 的 namespace；它不會自動接通其他 VM。跨 VM 仍需路由、forwarding、防火牆與回程路由。若目標是模擬跨主機二層鏈路，才另外設計 bridge／VXLAN。

在本例中，Node 要能透過 `192.168.139.30` 到達 `10.255.0.0/30`；Cisco 要能經 `10.255.0.1` 到達 Node 網段。BGP peering 不直接相鄰時，兩側都需要相符的 multihop 設定。

Cisco 鄰居設定概念如下，介面及 IOS 命令需依映像版本調整：

```text
router bgp 65000
 bgp router-id 10.255.0.2
 neighbor 192.168.139.10 remote-as 65001
 neighbor 192.168.139.10 ebgp-multihop 3
 neighbor 192.168.139.20 remote-as 65001
 neighbor 192.168.139.20 ebgp-multihop 3
```

```text
show ip bgp summary
show ip bgp
show ip route bgp
```

Cilium 端需配置 BGP instance、peer、advertisement 等資源，資源版本以已安裝的 CRD 為準。只有 router 端設定不會自動建立完整實驗。

即使 Cisco 收到 Pod 前綴，中間的 Containerlab host 仍須知道如何將實際 Pod 封包送往正確 Node；到 Node IP 的路由不足以保證到 Pod IP 的轉送。這也是「BGP Established，但流量不通」的重要檢查點。

官方依據：[Cilium BGP Control Plane](https://docs.cilium.io/en/stable/network/bgp-control-plane/bgp-control-plane/)；Containerlab 安裝、Cisco 映像與 Apple Silicon 執行條件沿用 [[Containerlab 安裝與設定指南]]。

## 延伸環境討論

一般 Kubernetes 資源操作可使用 OrbStack 內建 Kubernetes；要觀察 CNI、Node OS 與跨節點流量，使用 OrbStack Linux VM 搭配 MicroK8s／Cilium。本實驗採後者，再以 Containerlab 搭配 Cisco IOL 觀察路由與 BGP。Kind 搭配自選 CNI 則適合可重建的容器節點實驗，不與這裡的 VM 拓撲混用。
