# Colima + containerd 指令小抄

- source: `raw/my-vault/Note/Tech/Colima + containerd 指令小抄.md`
- source_sha256: `4cf9e40497184b9dc24a3a1a07df7592e698ed5d76b6b48da33f03826f4c55bf`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 啟動與狀態、Docker 與 Kubernetes 支援、安裝 nerdctl、基本操作（安裝前）、基本操作（安裝後）、Compose 指令。

## Source Notes

- Colima 可以同時支援 Docker runtime 與 Kubernetes (k3s)
- 開源與可攜：Colima 基於 Lima，設定透明、可版本控管；OrbStack 為封閉商業產品。
- 更貼近生產環境：多數 K8s 節點採用 containerd。本地用 containerd + nerdctl，工具與行為與伺服器一致（映像倉庫、cgroup、鏡像管理）。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
