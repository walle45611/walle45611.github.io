# Kubernetes GitOps Delivery

## Current View

在目前知識庫中，Kubernetes GitOps 交付可被整理為一條工具鏈：Helm 負責套件化安裝，Argo CD 負責宣告式同步，Image Updater 負責映像版本推進，Sealed Secrets 負責將密鑰安全地納入版本控制。這些元件在 MicroK8s 上可直接落地，但前提是安裝方式與憑證管理一致。

## Stable Conclusions

1. GitOps 可降低手動操作漂移，但前提是部署入口與工具命令要單一化。
2. `helm` 與 `microk8s helm3` 混用會增加維運混亂，應固定一種操作語境。
3. Argo CD 的初始可達性（NodePort/Proxy）與管理員憑證取得流程屬必要基礎設定。
4. Image Updater 要可靠運作，必須先完成 registry 認證 secret 與對應設定檔。
5. Secrets 若不經過加密封裝，GitOps 流程會在安全審計上形成明顯缺口。

## Working Heuristics

- 將「安裝控制面」與「應用交付面」分層：先叢集基礎元件，再啟用 GitOps 工具。
- 以 namespace 隔離平台元件（如 `argocd`），降低權限與故障影響範圍。
- 對外存取優先使用官方建議路徑（proxy），對 NodePort 做明確風險註記。
- 將 registry 憑證、密文封裝與 rollout restart 納入變更流程標準步驟。

## Open Questions

- 多 registry 與多環境（dev/staging/prod）下，Image Updater 規則如何標準化仍待補充。
- Sealed Secrets 與外部祕密管理系統（例如 Vault）混合架構的取捨仍需更多案例。

## Related Concepts

- [microk8s-production-readiness](./microk8s-production-readiness.md)

## Sources

- [microk8s-production-deployment-guide](../summaries/microk8s-production-deployment-guide.md)
