# 網路協定分層

## Current View

網路分層把不同範圍的工作分開：命名與解析、端點間傳輸、跨網路轉送，以及底層連接。排查問題時先問是哪一層的責任，再對應檢查位址、埠、路由或名稱解析。

## Working Model

1. 分層提供相對清楚的服務邊界，有助於模組化與跨設備協作；OSI/TCP/IP 模型是理解路徑的框架，不能直接當成實際封包的完整行為描述。[網路總覽](../summaries/my-vault-dashboard-networking-overview-895167887.md)
2. IP 負責跨網路封包轉送，提供盡力而為的服務；封包能否到達還受路由與 MTU 等條件影響。[IP 筆記](../summaries/my-vault-research-ip-internet-protocol-c8fb1204a.md)
3. TCP/UDP 筆記以埠辨識端點，並進一步整理 TCP 標頭與滑動視窗；不能把傳輸層埠與 IP 位址混為同一種識別。[TCP/UDP 筆記](../summaries/my-vault-research-tcp-udp-ed699ac3f.md)
4. DNS 負責名稱查詢與資源紀錄；名稱解析成功不表示傳輸連線一定成功。[DNS 筆記](../summaries/my-vault-research-dns-domain-name-service-5c6d0878f.md)

## Boundaries

來源筆記混合協定概念與個別環境的設定示例；埠範圍、部署慣例等具時間性的敘述不在此當作通用規則。容器的 veth、bridge、節點路由與 overlay 另見 [Container Networking](./container-networking.md)。

## Sources

- [網路模型總覽](../summaries/my-vault-dashboard-networking-overview-895167887.md)
- [IP](../summaries/my-vault-research-ip-internet-protocol-c8fb1204a.md)
- [TCP 與 UDP](../summaries/my-vault-research-tcp-udp-ed699ac3f.md)
- [DNS](../summaries/my-vault-research-dns-domain-name-service-5c6d0878f.md)
