# Kubernetes 網路：從 Linux 到 Pod、Service 與 DNS

- source: `raw/web-clipper/Understanding Kubernetes Networking in 30 Minutes - Ricardo Katz & James Strong.md`
- source_sha256: `ffd0ca38ca354e78ec652c1d8b0442cb7c050cb933be28619b81abdd117ae5a9`
- source link: https://www.youtube.com/watch?v=Mj04QOqAaJ8
- published: 2024-11-15
- source_created: 2026-10-04
- ingested_at: 2026-10-09
- type: source summary

## 摘要與關鍵資訊

Ricardo Katz 與 James Strong 的 2024 年演講，用 kind 示範 Linux 網路原語如何組成 Kubernetes 網路模型，再分開說明 CNI、Service、kube-proxy 與叢集 DNS 的責任。

- Pod 內容器共用 network namespace，以 localhost 互通；pause infrastructure container 維持該網路環境。不同 Pod 的網路隔離容許各自使用相同 port。
- 示範以 veth、bridge 與節點路由連接 Pod；每節點 Pod CIDR 是演示中的配置，跨節點路由或 tunnel 取決於網路實作。Pod CIDR 應避免與外部網段衝突。
- CNI 是網路介面標準及插件生態；插件配置介面、IP 與路由，實現 Kubernetes 網路模型，不等同 Service 的後端映射。
- Pod 重建後 IP 可能改變；ClusterIP 提供穩定入口，NodePort 透過節點 port 提供入口。Service 選擇後端 Pod，不是固定把流量送至原先 Pod IP。
- kube-proxy 依 API server／EndpointSlice 更新節點上的封包處理規則；各 kube-proxy 不靠彼此交換 Service 狀態。講者也提及 eBPF replacement，不能假設所有叢集一定跑 kube-proxy。
- CoreDNS 與 Pod DNS search path 讓應用使用 Service 名稱；同名 Service 配合 namespace 可形成可重用架構。
- 預設連通模型還需要 NetworkPolicy 做存取限制；Ingress／Gateway API 為延伸議題。Local traffic policy 可能使沒有本地 endpoint 的節點無法服務流量。

## 來源邊界

內容限於 Linux 與演講的 kind 示範。逐字稿中的產品拼字及特定 DNS IP 不視為所有叢集的預設。未提供當前 CNI 效能比較、完整 NetworkPolicy 或 Gateway API 實作；補強既有容器網路與 MicroK8s 部署頁的封包路徑說明。

## 相關概念

- [[container-networking]]
- [[microk8s-production-readiness]]
- [[kubernetes-design-principles]]
