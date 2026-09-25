# K8s 各 Component 說明

- source: `raw/my-vault/Note/Tech/K8s 各 Component 說明.md`
- source_sha256: `37e9d156805847295adb203d0d495db6c46a8526b4938e78294ee2baf513ca00`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 Kubernetes 設計原則、API Server、Cluster Store、Controller Manager 和 Controllers、Scheduler（調度器）、Kubernetes Node。

## Source Notes

- 來源：KubeCon + CloudNativeCon North America 2018 投影片〈Kubernetes Principles Introduced〉。
- 宣告式優先於命令式（Kube API declarative over imperative）：描述系統應達到的期望狀態，由 Kubernetes 持續協調實際狀態與期望狀態之間的差異。
- 沒有隱藏的內部 API（No hidden internal APIs）：系統內部使用的 API 也應對外開放，讓外部工具能透過相同介面整合。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
