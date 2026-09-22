# Kubernetes Design Principles: Understand the Why

- source: `raw/Kubernetes Design Principles Understand the Why - Saad Ali, Google.md`
- source link: https://www.youtube.com/watch?v=ZuIQurh_kDk
- original title: Kubernetes Design Principles: Understand the Why - Saad Ali, Google
- author: Saad Ali（Google，講者）；CNCF（發布頻道）
- published: 2018-12-16（剪藏 metadata）
- source_created: 2026-09-23
- ingested_at: 2026-09-23
- type: 演講影片剪藏與逐字稿

## 摘要

本演講以「為什麼如此設計」解釋 Kubernetes：使用者描述期望狀態，各元件透過共同的 API 觀察狀態並各自推進工作；應用程式可沿用檔案與環境變數介面，儲存需求則透過 PVC 與底層實作分離。這些設計以責任分離支援故障恢復、擴充與跨環境部署。

## 四項設計原則

1. **Declarative APIs（4:05–6:12）**：描述希望維持的狀態，而非只發送一次「啟動容器」命令。演講以 ReplicaSet 說明維持副本的目的；持續比對與修復才構成自我恢復機制。
2. **No hidden internal APIs（9:01–16:42）**：內部元件透過使用者也能存取的 Kubernetes API 協作。Scheduler 觀察尚未排程的 Pod，更新節點指派；kubelet 觀察指派到自身節點的 Pod，推進本地容器狀態。這是演講描述的控制面協作模式，不是對所有系統通訊介面的窮盡列舉。
3. **Meet users where they are（17:15–20:41）**：Secret、ConfigMap 與 Downward API 資訊可透過檔案或環境變數提供，減少既有應用為了部署到 Kubernetes 而加入專屬 API 呼叫的需求。
4. **Workload portability（25:52–30:33）**：Pod 直接綁定特定雲端磁碟會限制可攜性。PVC 表達容量與存取模式等需求，PV 表達可用儲存資源；StorageClass 則承接環境相關的動態佈建設定。目的為分離應用需求與叢集實作。

## 運作例子與重要細節

- **Level-triggered reconciliation（11:11–11:51）**：元件恢復後重新觀察目前狀態，不必只靠收到某一次歷史事件才能決定工作。此處是狀態驅動的設計說明，不代表控制器不使用事件通知。
- **儲存流程（22:26–25:19）**：演講示範 Pod 節點指派、attach/detach controller 處理遠端磁碟，以及 kubelet 讓容器取得儲存的分工；並在 24:16 提及 Kubernetes 1.13／CSI 1.0 與 VolumeAttachment 的當時演進。
- **靜態與動態佈建（27:12–28:52）**：管理員可預先提供 PV，也可由 StorageClass 對應的機制建立儲存。換環境仍須有能滿足 PVC 的資源與設定，不能把相同名稱當成所有儲存語意一致的保證。
- **CRD 與自訂控制器（33:14–35:18）**：擴充 API 物件與控制器可沿用共同 API machinery；講者「若從頭設計，內建類型也可作為 CRD」是設計觀點，並非現有 Pod、Node 都是 CRD 的事實。

## 證據與限制

- 本頁依本地剪藏及其逐字稿整理，未另行核對影片音訊。逐字稿有辨識錯字，術語依上下文整理；不據此增加效能數據或當前版本結論。
- 演講在 15:17–16:13 將元件可獨立運作概括為沒有單點故障。本頁僅保留其「部分元件可依最後觀察狀態持續運作」的理由；這不足以證明 API server 中斷時仍可正常進行所有新排程、更新或故障恢復，也不能取代 HA 設計。
- 36:01–37:20 的問答指出持續收斂模型與時間點快照需求之間的張力。這是在討論控制流程的最終收斂，不宜擴張成 Kubernetes 所有讀寫或底層儲存一概不具一致性保證。
- 2018 年的 volume plugin 與 CSI 說明保留為歷史脈絡，不直接用作當前安裝指南；持久化儲存也不等同自動備份或任意故障下的資料保證。

## 與既有知識的關係

本來源補上既有部署操作背後的理由，未取代工具設定：[Kubernetes GitOps Delivery](../concepts/kubernetes-gitops-delivery.md) 可由期望狀態與持續收斂解釋；[MicroK8s Production Readiness](../concepts/microk8s-production-readiness.md) 仍需獨立落實 HA 與儲存治理。可重用的設計原則整理在 [Kubernetes Design Principles](../concepts/kubernetes-design-principles.md)。
