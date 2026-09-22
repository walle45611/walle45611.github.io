# Container Networking

## Current View

從已收錄的容器網路演講看，理解連通性可分為隔離、節點內連接、跨節點路由，以及必要時的封裝。Network namespace 隔離網路狀態；veth 連接 namespace 與主機；bridge 處理節點內二層連接；路由決定跨子網流向。

## Working Model

1. 同一節點的多個容器可用 veth 與 bridge 相連。
2. 在演講的同 L2 多節點拓樸中，容器子網可透過節點作為 next hop，配合 IP forwarding 直接到達。
3. 若底層網路無法提供容器子網路由，overlay 可將容器封包包在節點間可路由的封包裡。
4. 排錯時分別觀察容器、bridge、tunnel 與主機介面，區分內層容器位址和外層節點位址。

## Boundaries

Overlay 是一種連通方式，不是所有容器網路的必要條件。MTU、回程路由與 reverse-path filtering 必須配合實際拓樸；2018 年教學示範的參數不作為通用生產預設。現有來源不足以提供當前各 CNI 的效能或完整功能比較。

## Related Concepts

- [microk8s-production-readiness](./microk8s-production-readiness.md)：部署基線之外，另需理解節點與 Pod 的封包路徑。

## Sources

- [container-networking-from-scratch](../summaries/container-networking-from-scratch.md)
