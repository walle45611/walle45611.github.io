# Kubernetes Design Principles

## Current View

依已收錄的設計演講，Kubernetes 可從「描述期望狀態、元件各自收斂、應用與底層實作分離」理解。這補上既有 GitOps 與 MicroK8s 部署筆記的設計理由；目前原則的直接來源主要為 2018 年演講，並非跨版本完整規格。

## Working Model

1. **宣告目標**：API 物件保存希望系統維持的狀態，控制流程持續縮小實際狀態與期望狀態的差距。
2. **分散執行責任**：Scheduler、kubelet 等元件透過共同 API 物件協作；元件可在恢復後重新讀取狀態，減少對單次命令送達的依賴。
3. **降低應用耦合**：檔案與環境變數可承接設定、Secret 與 Pod 資訊，應用不一定需要內建 Kubernetes API 用戶端。
4. **分離需求與資源**：PVC 描述儲存需求，PV 表達儲存資源；StorageClass 承接環境的佈建選擇。可攜性仍以目標環境滿足需求為條件。
5. **以契約支援擴充**：CRD 與自訂控制器讓新類型與行為沿用 API machinery；宣告物件本身並不自動實作對應行為。

## 與既有部署知識的整合

- GitOps 把版本控制中的部署描述接到宣告式同步流程；設計原則解釋持續收斂的價值，既有 GitOps 來源則提供 Helm、Argo CD 等具體操作。
- MicroK8s 的部署成功與長期可用性仍需分別驗證。元件可以獨立恢復，不代表控制面無需高可用；PVC 抽象也不取代後端容量、存取條件與資料保護。

## Boundaries

Level-triggered 指依目前狀態決定行動，不等於不使用事件。工作負載收斂的最終一致性討論，不等於所有 API 或資料層的一致性語意。StorageClass 同名不保證後端能力等價；2018 年儲存實作細節須與當前環境分開看。

## Related Concepts

- [kubernetes-gitops-delivery](./kubernetes-gitops-delivery.md)
- [microk8s-production-readiness](./microk8s-production-readiness.md)

## Sources

- [kubernetes-design-principles-understand-the-why](../summaries/kubernetes-design-principles-understand-the-why.md)
