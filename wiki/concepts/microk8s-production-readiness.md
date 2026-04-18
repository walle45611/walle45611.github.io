# MicroK8s Production Readiness

## Current View

在目前知識庫中，MicroK8s 可視為「可進入生產環境的輕量 Kubernetes 發行版」，但適用範圍集中在小到中型叢集、邊緣部署與需要快速交付的場景。是否可用於 production 的關鍵不在於發行版名稱，而在於是否補齊高可用、網路、權限與運維流程。

## Stable Conclusions

1. MicroK8s 並非只能開發測試；官方與社群都有 production 使用案例。
2. 中型生產可行性高度依賴基礎元件配置完整度（DNS、Storage、Ingress、MetalLB、Metrics）。
3. 叢集治理必須明確區分 control-plane 與 worker，並以 label/taint 控制排程行為。
4. 操作面要有可重複流程與故障排查路徑，否則部署成功不代表可穩定運維。
5. 大規模、多雲或高度整合場景，需評估 kubeadm/RKE2/託管雲服務等替代方案。

## Working Heuristics

- 先建立最小可用基線：單機安裝、核心插件、kubeconfig 與健康檢查。
- 再擴展到多節點：先 worker，後 control-plane，並同步完成污點與角色標籤策略。
- 針對裸機/內網環境，將 LoadBalancer IP 池與防火牆開孔視為第一級部署檢查點。
- 將疑難排解命令（狀態、daemon log、join 連線）納入標準操作手冊。

## Open Questions

- 對不同硬體條件下的 HA 故障恢復時間，仍缺少跨來源量化資料。
- 在長期升級週期中，Snap 通道策略與版本相容性治理仍需更多實證。

## Related Concepts

- [kubernetes-gitops-delivery](./kubernetes-gitops-delivery.md)

## Sources

- [microk8s-production-deployment-guide](../summaries/microk8s-production-deployment-guide.md)
