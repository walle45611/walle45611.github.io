# K8s CSI、PV、PVC 與雲端儲存

- source: `raw/my-vault/Note/Tech/K8s CSI、PV、PVC 與雲端儲存.md`
- source_sha256: `8b1b9ba00cb2690900b990bf452c8da2389f0a843e7f7bfacc9bba5fe520a3fd`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 元件與宣告的分工、attach 與 mount 的差別、GKE 與本機叢集使用雲端儲存、本機 Kubernetes 接遠端 NFS 範例。

## Source Notes

- 動態配置的準備順序是「安裝／啟用 driver → 建立 StorageClass → 建立 PVC → Pod 使用 PVC」。實際 provisioning／binding 時機取決於 binding mode；WaitForFirstConsumer 會等待使用 PVC 的 Pod，配合排程與拓撲再配置／綁定，並非永遠先建立 PV 才開始排程。也可以預先建立 PV 做靜態配置。
- CSI 的 controller-side attach 常由 external-attacher 與 driver 協作；不是 kubelet 直接負責整套雲端 attach 流程。Node side 執行 stage／publish，實際步驟依 driver 能力與 volume mode 而定。Pod 只宣告需求，不是 Pod 內的程式自行執行 mount。
- NFS 通常沒有把 block device attach 到 VM 的步驟，而是透過網路掛載遠端檔案系統。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
