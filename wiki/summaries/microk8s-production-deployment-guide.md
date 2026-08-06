# MicroK8s 完整部署指南：從安裝到生產級應用

- source: `raw/MicroK8s 完整部署指南：從安裝到生產級應用.md`
- blog source: `blog/source/_posts/MicroK8s-完整部署指南：從安裝到生產級應用.md`
- source link: [https://blog.walle4561.com/20250808/ecf5/#more](https://blog.walle4561.com/20250808/ecf5/#more)
- original title: MicroK8s 完整部署指南：從安裝到生產級應用
- author: Walle
- published: 2025-08-08
- type: blog post summary

## Summary

這份來源提供一條從 Ubuntu 安裝 MicroK8s 到可用於中型生產環境的實務路徑：先完成核心插件（DNS、儲存、Ingress、MetalLB、Metrics），再補齊叢集節點治理、Helm、Argo CD、Sealed Secrets 與 Argo CD Image Updater。文章核心主張是：MicroK8s 可進入 production，但前提是明確配置 HA、權限與網路元件，並以 GitOps 工具鏈維持部署可重現性。

## Key Claims

1. MicroK8s 被官方定位為 production-ready，但是否適配仍取決於規模與情境。
2. 中型生產環境可行，建議至少搭配 HA、RBAC、MetalLB 等基礎能力。
3. Helm 與 Argo CD 可形成可維護的交付流程，但要避免混用 `helm` 與 `microk8s helm3`。
4. 秘密管理與映像倉儲認證要先制度化（Sealed Secrets、GHCR credentials），否則 GitOps 流程難以落地。
5. Dashboard 對外存取以 `dashboard-proxy` 為優先，NodePort 屬非官方首選。

## Important Details

- 安裝流程以 `snap install microk8s --classic` 開始，並要求使用者加入 `microk8s` 群組與初始化 kubeconfig。
- 插件名稱更正：`hostpath-storage` 取代舊稱 `hostpath-provisioner`。
- 負載與網路基線包含 `ingress`、`metrics-server`、`metallb`，並示範固定 IP pool 設定。
- 節點面向同時覆蓋 worker/control-plane 加入流程，以及控制平面 taint/label。
- Argo CD 採 Helm 安裝並示範 NodePort 曝露與初始密碼取得方式。
- Sealed Secrets 兼顧控制器與 `kubeseal` CLI 安裝，支撐 GitOps 的密文提交流程。
- Argo CD Image Updater 補上 GHCR secret 與 `registries.conf` 範例，確保映像版本自動更新可運作。
- 疑難排解聚焦在 MetalLB 重設、節點 join 防火牆開孔、daemon log 檢查。

## Practical Takeaways From This Source

- 先固定一條「可重複執行」的安裝順序，再逐層擴充到 GitOps 與供應鏈元件。
- 進入 production 前，優先檢查網路（MetalLB、UFW）、權限（RBAC/群組）與高可用策略。
- 將 registry 憑證與 secrets 管理納入初始設計，避免後期在 CI/CD 流程補洞。
- 以 Argo CD + Image Updater 統一版本漂移治理，減少手動更新成本。

## Related Concepts

- [microk8s-production-readiness](../concepts/microk8s-production-readiness.md)
- [kubernetes-gitops-delivery](../concepts/kubernetes-gitops-delivery.md)
- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)

## Alignment With Current Wiki

- 這是知識庫第一份 Kubernetes/MicroK8s 主題來源，屬於新領域擴充，現階段無既有結論衝突。
- 新來源同時引入「平台可用性」與「GitOps 交付」兩條可持續累積的概念軸線。
