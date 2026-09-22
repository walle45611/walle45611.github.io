# Container Networking From Scratch

- source: `raw/Container Networking From Scratch - Kristen Jacobs, Oracle.md`
- source link: https://www.youtube.com/watch?v=6v_BDHIgOY8
- original title: Container Networking From Scratch - Kristen Jacobs, Oracle
- author: Kristen Jacobs（講者）；CNCF（發布頻道）
- published: 2018-12-16
- source_created: 2026-09-22
- ingested_at: 2026-09-22
- type: YouTube transcript summary

## Summary

演講以四個逐步擴大的實驗，說明容器如何從單機 network namespace 連線，走到跨主機 overlay。核心是辨識 namespace、veth、bridge、路由與封裝各自負責的層次，而不是把容器網路視為 Kubernetes 插件的黑箱。

## Key Points

1. **單一 namespace（2:49–7:48）**：network namespace 提供隔離的介面、路由與防火牆規則；veth pair 一端放進 namespace，另一端留在主機，配合路由建立連通性。即使沒有應用程式，namespace 的 kernel networking stack 仍可回應 ICMP。
2. **同一節點多個 namespace（8:18–12:29）**：把各 veth 的主機端接到 Linux bridge；同網段封包經二層轉送，離開該網段則透過 gateway 與主機路由。演示中同橋 ping 的 TTL 保持 64。
3. **同一 L2 網路的多節點（12:55–18:00）**：各主機配置遠端容器子網的 next-hop 路由，並開啟 IP forwarding。此拓樸可直接路由，不需要 overlay；演示跨兩個節點的 ping TTL 減少 2。
4. **跨網路 overlay（18:17–30:00）**：若無法讓中間路由器或雲端網路掌握容器子網，可將原始 IP packet 導向 TUN，由使用者空間程序以 UDP 封裝，送往遠端節點再解封裝。底層只需能到達節點位址。

## Tools and Evidence

- `ip` 建立 namespace、veth、bridge 與路由；`socat` 在示範中連接 TUN 與 UDP；`tcpdump`／`tshark` 用來觀察封包。
- 擷取主機實體介面可見外層節點 IP；在 TUN 上可見解封裝後的容器 IP；bridge 上則可見對應 Ethernet frame。
- 講者將 host-gw 對應直接路由、UDP backend 對應教學 overlay，並提到 kernel VXLAN、雲端路由，以及不同節點與子網映射的管理方式。

## Limits

這是 2018 年演講的教學拓樸與工具觀察，不是現行 Flannel、Calico 或雲端服務的功能清單。逐字稿未保留完整腳本，不宜據此重建可直接部署的設定。

講者在示範中把 MTU 從 1500 調為 1492，並關閉 reverse-path filtering 以處理該拓樸的回程路徑；這些是示範設定，不能泛化為所有封裝網路的 MTU 公式或生產安全設定。UDP 封裝本身不提供可靠傳送；講者以內層 TCP 的重試說明可靠性所在層次。

## Alignment and Related Concepts

補強既有 MicroK8s 部署筆記的網路原理層；部署元件清單與底層封包路徑應分別理解。

- [container-networking](../concepts/container-networking.md)
- [microk8s-production-readiness](../concepts/microk8s-production-readiness.md)
