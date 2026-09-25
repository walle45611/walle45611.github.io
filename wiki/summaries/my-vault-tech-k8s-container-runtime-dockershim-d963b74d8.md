# K8s Container Runtime 與 dockershim 遷移

- source: `raw/my-vault/Note/Tech/K8s Container Runtime 與 dockershim 遷移.md`
- source_sha256: `2fb1efce033e0ed1b49954483822e568744d9d4782b87120266d32abbe297b72`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 移除的是 dockershim、管理員盤點與遷移、操作工具的差異。

## Source Notes

- Kubernetes 從 v1.24 移除內建 dockershim。Docker Engine 本身不提供 CRI，因此常改用 containerd 或 CRI-O；需要保留 Docker Engine 時可透過 cri-dockerd 銜接。這不影響 Docker／Buildx 建置相容映像，再推送至 registry 供 Kubernetes 使用。
- CRI 是 kubelet 與 runtime 的介面；containerd 管理映像及容器生命週期；runc 是依 OCI runtime specification 啟動容器的低階 runtime。「dockerx」若指 Docker Buildx，與 dockershim 是不同元件。
- 依據：Migrating from dockershim。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
