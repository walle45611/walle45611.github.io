# K8s HA、Quorum 與 Split-brain

- source: `raw/my-vault/Note/Tech/K8s HA、Quorum 與 Split-brain.md`
- source_sha256: `a402ccf280224bdec89a1e603dd16f05086c8a0e23912fc3d8ff76d642bb9222`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

Split-brain 是分散式系統因網路分割而出現互相衝突的 active side，不專指 Kubernetes master。對使用 Raft 的 datastore，提交新寫入需要多數 voting members，少數派不能自行形成合法的提交多數。

## Source Notes

- 避免衝突提交的是 quorum 與共識協定；使用奇數能以較少成員取得相同故障容忍能力。Control-plane Node 數量不能直接等同於 datastore voting members，必須確認實際部署。
- MicroK8s 的 HA datastore 使用 dqlite；worker-only 節點不會增加 control-plane HA 能力。不要把 dqlite 的操作方式寫成 etcd 的操作方式。參考：MicroK8s High Availability。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
