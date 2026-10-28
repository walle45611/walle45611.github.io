---
blog: true
blog_title: 'K8s Calico 安裝與網路判讀小抄'
blog_date: '2026-10-29'
blog_url: https://blog.walle4561.com/articles/posts/k8s-calico-install-network-troubleshooting/
blog_tag: OS
blog_topic: Kubernetes
---

# K8s Calico 安裝與網路判讀小抄

> [!tip] 先分清楚
> CNI 建立 Pod 網路；Calico 可執行 NetworkPolicy；kube-proxy 處理 Service 轉送。安裝成功、Pod 互通、政策生效，是三件都要驗證的事。

## Calico v3.31.6 實驗必記

| 條件 | 要確認的內容 |
|---|---|
| 版本 | manifests 與安裝文件對應 **v3.31.6**，不要混用 latest |
| 安裝方式 | Tigera Operator，並安裝對應 CRDs |
| 資料平面 | 標準 Linux **Iptables**，不選 BPF |
| Pod CIDR | **172.17.0.0/16**，這是此實驗指定值，不是所有叢集的通用值 |

順序記成：**CRDs → Operator → 修改 Installation → 建立 custom resources → 等待並驗證**。CRD 讓 API Server 認得新種類，Operator 依 Installation 把實際元件部署出來；CR 顯示 `created` 只表示設定已收下。

在官方 custom-resources.yaml 的 Installation 中修改以下欄位，其他 ipPools 設定保留：

```yaml
# Installation 的 spec 片段，不是完整 manifest
spec:
  calicoNetwork:
    linuxDataplane: Iptables
    ipPools:
      - cidr: 172.17.0.0/16
```

APIServer 是 Calico API server；Goldmane 是流量彙整、Whisker 是觀測介面。這裡的 APIServer 資源不是另一台 Kubernetes kube-apiserver。

```bash
kubectl get tigerastatus
kubectl get pods -n calico-system
kubectl get pods -n tigera-operator
```

TigeraStatus 預期 `AVAILABLE=True`、`PROGRESSING=False`、`DEGRADED=False`；長駐 Pod 應 Running 且 Ready。政策仍要用「允許／拒絕連線」實測。僅在題目明示可忽略時，才忽略該實驗的 csi-node-driver 錯誤。

## 判斷主機介面：先看 IP 與路由

```bash
ip -br addr
ip route
ip route get 1.1.1.1
kubectl get nodes -o wide
```

- `ip -br addr`：介面是否 UP、有哪些位址。
- `ip route`：default route 的 `dev` 是主要對外出口線索。
- `ip route get`：到特定目的地實際選到的 `dev`、`src`；不會真的送出探測封包。
- Node 的 InternalIP 應與預期的 Node 間通訊位址相符；多網卡時不能只憑 `eth0` 名稱猜。

Bridge、veth、tunl、vxlan 等可能是虛擬介面；default route 也不保證就是叢集節點間應使用的網卡。封包模型見 [[Linux Network Namespace：netns、veth 與 Bridge]]。

## kube-proxy 和 iptables 是什麼關係

**kube-proxy 是程式，iptables／IPVS／nftables 是它可使用的 Linux 轉送後端**；通常由 kernel 依規則轉送封包，不是每個封包都交給 kube-proxy 程式處理。

| 模式 | 記法 |
|---|---|
| iptables | kube-proxy 寫入 netfilter 規則 |
| IPVS | 使用 kernel IPVS；Kubernetes 1.35 起已棄用 |
| nftables | 另一種 Linux 後端；需符合核心與版本條件 |
| userspace | 舊教材的歷史模式，已從現代版本移除 |

具備 Service 替代能力的 CNI 可接手 kube-proxy 功能，但不是所有 CNI 都會取代它。**Calico 的 Iptables dataplane 與 kube-proxy 的 proxy mode 是不同設定**。

相關：[[K8s CNI 比較：Cilium、Calico 與 Flannel]]、[[K8s 各 Component 說明#kube-proxy]]。

## 參考

- [Calico v3.31.6 manifests](https://github.com/projectcalico/calico/tree/v3.31.6/manifests)
- [Calico Operator 安裝流程：閱讀時選對版本](https://docs.tigera.io/calico/latest/getting-started/kubernetes/self-managed-onprem/onpremises)
- [Kubernetes Service 轉送與 proxy modes](https://kubernetes.io/docs/reference/networking/virtual-ips/)
