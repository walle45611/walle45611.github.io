# K8s kubelet 與 Pod 執行流程

- source: `raw/my-vault/Note/Tech/K8s kubelet 與 Pod 執行流程.md`
- source_sha256: `da1b0abf579ee812f97e464878ae4303ce4dbbf9e76ab480865e304a5541767e`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 kubelet 的位置與責任、API 與應用設定、檢查方向。

## Source Notes

- kubelet 是每個 Node 上的節點代理程式。Node 是執行工作負載的實體機或虛擬機，以及 Kubernetes 中對應的資源物件；kubelet 是其上的程式，兩者不是同義詞。
- Scheduler 決定 Pod 要放在哪台 Node；kubelet 持續協調已指派到本機的 Pod，透過 CRI 呼叫 containerd、CRI-O 等 runtime 建立 Pod sandbox、啟動及停止容器，並回報狀態。
- 這是一般由 API Server 管理的 Pod 流程；static Pod 可由 kubelet 讀取本機 manifest 管理。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
